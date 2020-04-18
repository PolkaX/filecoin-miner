use crate::constants::SECTOR_SIZES;
use crate::error::*;
use ds_rocksdb::DatabaseConfig;
use log::{error, info, warn};
use node_paramfetch::get_params;
use repo::{FsRepo, RepoType};
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Init {
    /// Specify the address of an already created miner actor
    #[structopt(long)]
    pub actor: Option<String>,
    /// Enable genesis mining (DON'T USE ON BOOTSTRAPPED NETWORK)
    #[structopt(long)]
    pub genesis_miner: bool,
    /// Create separate worker key
    #[structopt(long)]
    pub create_worker_key: bool,
    /// Worker key to use (overrides --create-worker-key)
    #[structopt(long, short)]
    pub worker: Option<String>,
    /// Owner key to use
    #[structopt(long, short)]
    pub owner: String,
    /// Specify sector size to use
    #[structopt(long)]
    pub sector_size: Option<usize>,
    /// Specify set of presealed sectors for starting as a genesis miner
    #[structopt(long)]
    pub pre_sealed_sectors: Option<String>,
    /// Specify the metadata file for the presealed sectors
    #[structopt(long)]
    pub pre_sealed_metadata: Option<String>,
    /// Don't check full_node sync status
    #[structopt(long)]
    pub no_sync: bool,
    /// Attempt to symlink to presealed sectors instead of copying them into place
    #[structopt(long)]
    pub symlink_imported_sectors: bool,
    /// Gateway of fetch params ("https://ipfs.io/ipfs/")
    #[structopt(
        long,
        default_value = "https://proof-parameters.s3.cn-south-1.jdcloud-oss.com/ipfs/"
    )]
    pub ipfs_gateway: String,
    /// Path of fetch params
    #[structopt(long, default_value = "/var/tmp/filecoin-proof-parameters/")]
    pub params_path: String,
}

impl Init {
    pub fn run(&self, path: PathBuf, config: DatabaseConfig) -> Result<()> {
        info!("Initializing lotus storage miner");

        let ssize = self.sector_size.unwrap_or(SECTOR_SIZES[0]);
        let _sym_link = self.symlink_imported_sectors;

        info!("Checking proof parameters");

        let parameters: serde_json::Value =
            serde_json::from_reader(&include_bytes!("../../../assets/parameters.json")[..])
                .unwrap();
        let _ = get_params(
            ssize as u64,
            &self.ipfs_gateway,
            PathBuf::from(&self.params_path),
            parameters,
        )?;
        info!("Trying to connect to full node RPC");

        // todo check version
        info!("Checking full node version");

        info!("Initializing repo");
        let repo = FsRepo::init(path.to_owned(), RepoType::StorageMiner, config)?;
        let _repo = if let Some(repo) = repo {
            repo
        } else {
            warn!("repo at '{}' is already initialized", path.display());
            return Ok(());
        };

        if let Some(ref _pre) = self.pre_sealed_sectors {
            // TODO
        }

        info!("start storage miner init");
        storage_miner_init().map_err(|e| {
            error!("Failed to initialize miner: {:?}", e);
            info!("Cleaning up %s after attempt... {:}", path.display());
            let _ = fs::remove_dir_all(path.as_path())
                .map_err(|e| error!("Failed to clean up failed storage repo: {:?}", e));
            e
        })?;
        info!("Storage miner successfully created, you can now start it with 'run'");
        Ok(())
    }
}

fn storage_miner_init() -> io::Result<()> {
    // TODO
    Ok(())
}
