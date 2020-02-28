// Copyright 2020 PolkaX

use libc::statfs;
use plum_address::Address;
use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    ffi::CString,
    fmt, fs, mem,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Mutex,
};

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Cache,
    Staging,
    Sealed,
    Unsealed,
}

impl DataType {
    fn over_head(&self) -> u64 {
        match self {
            Cache => 11,
            Staging => 1,
            Sealed => 1,
            Unsealed => 1,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    NotFound,
    Exists,
    NoSuitablePath,
}

#[derive(Debug)]
pub struct FileSystemError {
    kind: ErrorKind,
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for FileSystemError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::NotFound => "sector not found",
            ErrorKind::Exists => "sector already exists",
            ErrorKind::NoSuitablePath => "no suitable path for sector f    ond",
        }
    }
}

pub type StoragePath = PathBuf;
pub type SectorPath = Path;

pub trait SectorTrait {
    fn storage(&self) -> StoragePath;
    fn typ(&self) -> Option<DataType>;
    fn id(&self) -> Option<u64>;
    fn miner(&self) -> Option<Address>;
}

impl SectorTrait for SectorPath {
    fn storage(&self) -> StoragePath {
        self.parent().unwrap().parent().unwrap().to_path_buf()
    }

    fn typ(&self) -> Option<DataType> {
        let parent = self.parent().unwrap();
        if parent.ends_with("cache") {
            Some(DataType::Cache)
        } else if parent.ends_with("staging") {
            Some(DataType::Staging)
        } else if parent.ends_with("sealed") {
            Some(DataType::Sealed)
        } else if parent.ends_with("unsealed") {
            Some(DataType::Unsealed)
        } else {
            None
        }
    }

    fn id(&self) -> Option<u64> {
        let file_name = self.file_name().unwrap().to_str().unwrap();
        let v: Vec<&str> = file_name.rsplit('-').collect();
        if let Ok(i) = v[0].parse::<u64>() {
            Some(i)
        } else {
            None
        }
    }

    fn miner(&self) -> Option<Address> {
        let file_name = self.file_name().unwrap().to_str().unwrap();
        let v: Vec<&str> = file_name.rsplit('-').collect();
        if let Ok(address) = plum_address::Address::from_str(v[1]) {
            Some(address)
        } else {
            None
        }
    }
}

pub fn SectorName(miner: Address, sector_id: u64) -> String {
    fmt::format(format_args!("s-{}-{}", miner, sector_id))
}

pub trait StorageTrait {
    fn sector(&mut self, typ: DataType, miner: Address, id: u64) -> StoragePath;
}

impl StorageTrait for StoragePath {
    fn sector(&mut self, typ: DataType, miner: Address, id: u64) -> StoragePath {
        let path = match typ {
            DataType::Cache => self.join("cache"),
            DataType::Staging => self.join("staging"),
            DataType::Sealed => self.join("sealed"),
            DataType::Unsealed => self.join("unsealed"),
        };
        path.join(SectorName(miner, id)).to_owned()
    }
}

#[derive(Clone, Debug)]
pub struct PathInfo {
    cache: bool,
    weight: i64,
}

#[derive(Debug)]
pub struct FS {
    paths: HashMap<StoragePath, PathInfo>,
    reserved: HashMap<StoragePath, Vec<(DataType, u64)>>,
    // locks: HashMap<SectorPath, channel>,
    lk: Mutex<()>,
}

impl FS {
    pub fn new(cfg: &[PathConfig]) -> Self {
        let mut fs = FS {
            paths: HashMap::new(),
            reserved: HashMap::new(),
            lk: Mutex::new(()),
        };
        println!("cfg:{:?}", cfg);
        //cfg.iter().map(|path_cfg| {
        fs.paths.insert(
            cfg[0].path.clone(),
            PathInfo {
                cache: cfg[0].cache,
                weight: cfg[0].weight,
            },
        );
        //});

        fs
    }

    fn init_dir(&self) {
        for path in self.paths.keys() {
            let cache = path.clone().as_path().join("cache");
            let staging = path.clone().as_path().join("staging");
            let sealed = path.clone().as_path().join("sealed");
            let unsealed = path.clone().as_path().join("unsealed");
            fs::create_dir_all(cache).unwrap();
            fs::create_dir_all(staging).unwrap();
            fs::create_dir_all(sealed).unwrap();
            fs::create_dir_all(unsealed).unwrap();
        }
    }

    fn find_sector(&self, typ: &DataType, miner: &Address, id: u64) -> Option<PathBuf> {
        // return SectorPath
        for (path, path_info) in self.paths.iter() {
            let path = path.clone().sector(typ.clone(), miner.clone(), id);
            if let Ok(attr) = fs::metadata(path.as_path()) {
                if attr.is_file() & path_info.cache {
                    return Some(path);
                }
            }
        }
        None
    }

    fn reserved_bytes(&self, path: &StoragePath) -> i64 {
        let mut out = 0;
        if self.reserved.len() != 0 {
            self.reserved[path]
                .iter()
                .map(|(_, r)| out += r.clone() as i64);
        }
        out
    }

    fn available_bytes(&self, path: &StoragePath) -> Option<(i64, i64)> {
        let mut fs_stat = unsafe { mem::uninitialized() };
        let c_path = CString::new(path.as_os_str().to_str().unwrap()).unwrap();
        let value = unsafe { statfs(c_path.as_ptr(), &mut fs_stat) };
        if value == 0 {
            let fsavail = fs_stat.f_bavail as i64 * fs_stat.f_bsize;
            let avail = fsavail - self.reserved_bytes(path);
            Some((avail, fsavail))
        } else {
            println!("statfs return:{:?}", value);
            None
        }
    }

    fn find_best_path(&self, size: u64, cache: bool, strict: bool) -> Option<StoragePath> {
        let mut bestc = true;
        let mut bestw = 0u128;
        let mut best = StoragePath::new();
        for (path, path_info) in self.paths.iter() {
            if path_info.cache != cache && (bestc != path_info.cache || strict) {
                println!("path_info.cache: {:?}, cache: {:?}", path_info.cache, cache);
                continue;
            }
            if let Some((avail, _)) = self.available_bytes(path) {
                if avail < size as i64 {
                    println!("avail:{:?} < size:{:?}", avail, size);
                    continue;
                }
                let mut w = avail as u128;
                w = w.checked_mul(path_info.weight as u128).unwrap();
                //if w.cmp(&bestw) == Ordering::Greater {
                if path_info.cache == cache {
                    bestw = w;
                }
                best = path.clone();
                bestc = path_info.cache;
            /*} else {
                println!("w: {:?} not greater than bestw", w);
            }*/
            } else {
                println!("can't available_bytes");
                continue;
            }
        }

        if best == StoragePath::new() {
            None
        } else {
            Some(best)
        }
    }

    fn reserve(&mut self, typ: DataType, path: StoragePath, size: u64) -> bool {
        if let Some((avail, fsavail)) = self.available_bytes(&path) {
            if size as i64 > avail {
                return false;
            }
            if let Some(v) = self.reserved.remove(&path) {
                let vec = v
                    .into_iter()
                    .map(|(t, s)| if t == typ { (t, s + size) } else { (t, s) })
                    .collect();
                self.reserved.insert(path, vec);
            } else {
                let mut vec = Vec::new();
                vec.push((typ, size));
                self.reserved.insert(path, vec);
            }
            true
        } else {
            false
        }
    }

    fn alloc_sector(
        &mut self,
        typ: DataType,
        miner: Address,
        ssize: u64,
        cache: bool,
        id: u64,
    ) -> Option<PathBuf> {
        if let Some(path) = self.find_sector(&typ, &miner, id) {
            return Some(path);
        }
        if let Some(mut p) = self.find_best_path(typ.over_head(), cache, false) {
            let sp = p.sector(typ.clone(), miner, id);
            let storage = sp.as_path().storage();
            if self.reserve(typ.clone(), storage, typ.over_head()) {
                return Some(sp);
            } else {
                println!("reserve failed");
                return None;
            }
        } else {
            println!("can't find the best path");
            None
        }
    }

    pub fn force_alloc_sector(
        &mut self,
        typ: DataType,
        miner: Address,
        ssize: u64,
        cache: bool,
        id: u64,
    ) -> Option<PathBuf> {
        if let Some(path) = self.find_sector(&typ, &miner, id) {
            fs::remove_file(path);
        }
        return self.alloc_sector(typ, miner, ssize, cache, id);
    }

    fn prepare_cache_move(
        &mut self,
        sector: &SectorPath,
        ssize: u64,
        tocache: bool,
    ) -> Option<PathBuf> {
        if sector.miner().is_none() || sector.id().is_none() {
            return None;
        }
        if let Some(mut p) = self.find_best_path(ssize, tocache, true) {
            self.reserve(sector.typ().unwrap(), p.clone(), ssize);
            return Some(p.sector(
                sector.typ().unwrap(),
                sector.miner().unwrap(),
                sector.id().unwrap(),
            ));
        }
        None
    }

    fn move_sector(from: &SectorPath, to: &SectorPath) -> bool {
        if from == to {
            return false;
        }
        if let Ok(attr) = fs::metadata(from) {
            if attr.is_dir() {
                //To do: migrate dir.
            } else {
                fs::copy(from, to);
                fs::remove_file(from).unwrap();
            }
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathConfig {
    path: PathBuf,
    cache: bool,
    weight: i64,
}

impl Default for PathConfig {
    fn default() -> Self {
        let mut path = PathBuf::new();
        path.push("/tmp/test");
        PathConfig {
            path: path,
            cache: true,
            weight: 0,
        }
    }
}

pub fn OpenFs(cfg: &[PathConfig]) -> FS {
    let fs = FS::new(cfg);
    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_utils() {
        let sp = SectorPath::new("/aoe/aaa-oeu/cache/s-t0999-84");

        let i = sp.id().unwrap();
        assert_eq!(84, i);

        let a = sp.miner().unwrap();
        assert_eq!("t0999", a.to_string());

        let s = sp.storage();
        assert_eq!("/aoe/aaa-oeu", s.into_os_string().to_str().unwrap());
    }
}
