use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use config::LoadConfig;
use datastore::{key, namespace};
use ds_rocksdb::{DatabaseConfig, RocksDB};
use fs_lock::{lock, unlock};
use log::info;
use parity_multiaddr::Multiaddr;

use plum_wallet::{KeyInfo, KeyStore as KeyStoreT};

use node_utils::{base32_decode, other_io_err};

pub const FS_API: &'static str = "api";
pub const FS_APITOKEN: &'static str = "token";
pub const FS_CONFIG: &'static str = "config.toml";
pub const FS_DATASTORE: &'static str = "datastore";
pub const FS_LOCK: &'static str = "repo.lock";
pub const FS_KEYSTORE: &'static str = "keystore";

pub type RepoDatastore = namespace::NSDatastore<RocksDB>;

const LOG_TARGET: &'static str = "repo";

#[derive(Debug, Copy, Clone)]
pub enum RepoType {
    FullNode,
    StorageMiner,
}

pub struct FsRepo {
    path: PathBuf,
    repo_type: RepoType,
    config: DatabaseConfig,
}

fn exist(path: &PathBuf) -> bool {
    let mut path = path.to_owned();
    path.push(FS_DATASTORE);
    path.exists()
}

fn default_config(repo_type: RepoType) -> io::Result<String> {
    match repo_type {
        RepoType::FullNode => {
            config::set_role(config::Role::FullNode);
            config::config_comment(&config::default_full_node())
        }
        RepoType::StorageMiner => {
            config::set_role(config::Role::StorageMiner);
            config::config_comment(&config::default_storage_miner())
        }
    }
}

fn init_config(path: &PathBuf, repo_type: RepoType) -> io::Result<()> {
    let mut path = path.to_owned();
    path.push(FS_CONFIG);
    if path.exists() {
        return Ok(());
    }

    let s = default_config(repo_type)?;
    fs::write(path.as_path(), s)
}

fn init_keystore(path: &PathBuf) -> io::Result<()> {
    let mut path = path.to_owned();
    path.push(FS_KEYSTORE);
    if path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "repo exists"));
    }
    fs::create_dir_all(path)?;
    // TODO set permission for keystore.
    Ok(())
}

impl FsRepo {
    /// init repo directory, if the directory is exist, return `Ok(None)`
    pub fn init(
        path: PathBuf,
        repo_type: RepoType,
        config: DatabaseConfig,
    ) -> io::Result<Option<Self>> {
        if exist(&path) {
            return Ok(None);
        }
        info!(target: LOG_TARGET, "Initializing repo at '{:?}'", path);
        fs::create_dir(path.as_path())?;
        // init config
        init_config(&path, repo_type)?;
        // init keystore
        init_keystore(&path)?;

        Ok(Some(FsRepo {
            path,
            repo_type,
            config,
        }))
    }

    pub fn open(path: PathBuf, repo_type: RepoType, config: DatabaseConfig) -> io::Result<Self> {
        if !exist(&path) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "repo {{{:}}} is not exist, you should init first.",
                    path.display()
                ),
            ));
        }
        Ok(FsRepo {
            path,
            repo_type,
            config,
        })
    }
}

// interface in go, but do not need write in trait in rust.
impl FsRepo {
    pub fn api_endpoint(&self) -> io::Result<Multiaddr> {
        let mut path = self.path.to_owned();
        path.push(FS_API);
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "API not running (no endpoint).",
            ));
        }
        let s = fs::read_to_string(path.as_path())?;
        Multiaddr::from_str(&s).map_err(other_io_err)
    }

    pub fn api_token(&self) -> io::Result<Vec<u8>> {
        let mut path = self.path.to_owned();
        path.push(FS_APITOKEN);
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "API not running (no endpoint).",
            ));
        }
        let buf = fs::read(path.as_path())?;
        Ok(buf)
    }

    pub fn lock(&self) -> io::Result<FsLockedRepo> {
        let file = lock(&self.path, FS_LOCK)?;
        let metadata = MetaData {
            path: self.path.to_owned(),
            repo_type: self.repo_type,
            file,
        };

        let mut path = self.path.to_owned();
        path.push(FS_DATASTORE);
        let datastore_path = path.as_path().to_str().ok_or(io::Error::new(
            io::ErrorKind::Other,
            format!("path to string failed. path:{:?}", path),
        ))?;

        let ds = RocksDB::new(datastore_path, &self.config).map_err(other_io_err)?;
        Ok(FsLockedRepo { metadata, ds })
    }
}

pub struct MetaData {
    path: PathBuf,
    repo_type: RepoType,
    file: fs::File,
}

pub struct FsLockedRepo {
    metadata: MetaData,
    ds: RocksDB,
}

impl Drop for FsLockedRepo {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

impl FsLockedRepo {
    pub fn path(&self) -> &Path {
        self.metadata.path.as_path()
    }

    fn join(&self, file: &str) -> PathBuf {
        let mut path = self.metadata.path.to_owned();
        path.push(file);
        path
    }

    pub fn close(&self) -> io::Result<()> {
        fs::remove_file(self.join(FS_API).as_path())?;
        unlock(&self.metadata.file)?;
        Ok(())
    }

    pub fn datastore(&self, name: &str) -> io::Result<RepoDatastore> {
        let prefix = key::Key::new(name);
        unsafe {
            self.ds.add_column(&prefix).map_err(other_io_err)?;
        }
        Ok(namespace::wrap(self.ds.clone(), prefix))
    }

    pub fn set_api_endpoint(&self, addr: &Multiaddr) -> io::Result<()> {
        let path = self.join(FS_API);
        fs::write(path.as_path(), addr.to_string())
    }

    pub fn set_api_token(&self, token: &[u8]) -> io::Result<()> {
        let path = self.join(FS_APITOKEN);
        fs::write(path.as_path(), token)
    }

    pub fn keystore(&self) -> Keystore {
        Keystore {
            path: self.metadata.path.to_owned(),
        }
    }

    pub fn config<C: LoadConfig>(&self) -> io::Result<C> {
        let path = self.join(FS_CONFIG);
        C::from_file(path.as_path())
    }
}

pub struct Keystore {
    path: PathBuf,
}

impl KeyStoreT for Keystore {
    type Error = io::Error;

    fn list(&self) -> Result<Vec<String>, Self::Error> {
        let d = fs::read_dir(self.path.as_path())?;
        let mut v = vec![];
        for f in d {
            // TODO permission
            let f = f?;
            if f.file_type()?.is_file() {
                let name = f
                    .file_name()
                    .into_string()
                    .map_err(|e| other_io_err(format!("{:?}", e)))?;
                let key = base32_decode(name)
                    .map(|v| String::from_utf8_lossy(&v).to_string())
                    .map_err(other_io_err)?;
                v.push(key);
            }
        }
        Ok(v)
    }

    fn get<K: AsRef<str>>(&self, key: K) -> Result<Option<KeyInfo>, Self::Error> {
        let name = base32_decode(key.as_ref())
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .map_err(other_io_err)?;
        let mut path = self.path.to_owned();
        path.push(key.as_ref());
        // todo permission check
        let v = fs::read(path.as_path())?;
        Ok(Some(serde_json::from_slice(&v).map_err(other_io_err)?))
    }

    fn put(&mut self, key: String, info: KeyInfo) -> Result<(), Self::Error> {
        let filename = base32_decode(key.as_bytes())
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .map_err(other_io_err)?;

        let mut path = self.path.to_owned();
        path.push(filename.clone());
        if path.as_path().exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "this keystore already exist: name:{}, filename:{}",
                    key, filename
                ),
            ));
        }
        let v = serde_json::to_vec(&info).map_err(other_io_err)?;
        fs::write(path.as_path(), v)
    }

    fn delete<K: AsRef<str>>(&mut self, key: K) -> Result<(), Self::Error> {
        let filename = base32_decode(key.as_ref())
            .map(|v| String::from_utf8_lossy(&v).to_string())
            .map_err(other_io_err)?;

        let mut path = self.path.to_owned();
        path.push(filename.clone());
        fs::remove_file(path.as_path())
    }
}
