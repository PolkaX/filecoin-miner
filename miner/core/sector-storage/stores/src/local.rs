use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use url::Url;

use plum_sector::{RegisteredProof, SectorId};

use crate::error::{Result, StoresError};
use crate::filetype::{parse_sector_id, sector_name, SectorFileType, SectorFileTypes, SectorPaths};
use crate::index::Index;
use crate::index::{StorageId, StorageInfo};
use crate::traits::{stat, FsStat, Store};
use crate::TARGET;

use log::{debug, error, info};

pub struct StoragePath {
    pub id: StorageId,
    pub weight: u64,

    pub local_path: PathBuf,

    pub can_seal: bool,
    pub can_store: bool,
}

// [path]/sectorstore.json
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalStorageMeta {
    #[serde(rename = "ID")]
    id: StorageId,
    weight: u64, // 0 = readonly

    can_seal: bool,
    can_store: bool,
}

// .lotusstorage/storage.json
pub struct StorageConfig {
    storage_paths: Vec<PathBuf>,
}

pub trait LocalStorage {
    fn get_storage(&self) -> Result<StorageConfig>;
    fn set_storage(&mut self, f: impl Fn(&StorageConfig)) -> Result<()>;
}

pub const META_FILE: &'static str = "sectorstore.json";

pub struct Local<Storage: LocalStorage> {
    local_storage: Storage,
    index: Index,
    urls: Vec<Url>,

    paths: HashMap<StorageId, PathBuf>,
}

impl<Storage: LocalStorage> Local<Storage> {
    pub fn new(s: Storage, index: Index, urls: Vec<Url>) -> Self {
        Local {
            local_storage: s,
            index,
            urls,
            paths: Default::default(),
        }
    }

    pub fn open(&mut self) -> Result<()> {
        let config = self.local_storage.get_storage()?;
        for path in config.storage_paths.iter() {
            self.open_path(path.as_path())?;
        }
        Ok(())
    }

    pub fn open_path(&mut self, path: &Path) -> Result<()> {
        let mut meta_path = path.to_path_buf();
        meta_path.push(META_FILE);

        let m = fs::read(meta_path.as_path())?;
        let meta: LocalStorageMeta = serde_json::from_slice(&m)?;

        let stat = stat(path)?;
        self.index.storage_attach(
            StorageInfo {
                id: meta.id.clone(),
                urls: self.urls.clone(),
                weight: meta.weight,
                can_seal: meta.can_seal,
                can_store: meta.can_store,
            },
            stat,
        )?;

        for t in SectorFileTypes::iter() {
            let mut p = path.to_path_buf();
            p.push(t.to_string());
            let dirs = fs::read_dir(p.as_path())?;
            for entry in dirs {
                let file_name = entry?.file_name();
                let name = file_name.to_string_lossy();
                let sid = parse_sector_id(&name)?;
                self.index
                    .storage_declare_sector(&meta.id, sid, (*t).into())
            }
        }
        self.paths.insert(meta.id, path.to_path_buf());
        Ok(())
    }

    pub fn local(&self) -> Result<Vec<StoragePath>> {
        let mut v = vec![];
        for (storage_id, path) in self.paths.iter() {
            if path.as_os_str().len() == 0 {
                continue;
            }
            let info = self.index.storage_info(storage_id)?;
            v.push(StoragePath {
                id: storage_id.clone(),
                weight: info.weight,
                local_path: path.clone(),
                can_seal: info.can_seal,
                can_store: info.can_store,
            })
        }
        Ok(v)
    }
}

impl<Storage: LocalStorage> Store for Local<Storage> {
    fn acquire_existing_sector(
        &self,
        s: SectorId,
        existing: SectorFileType,
    ) -> (SectorPaths, SectorPaths) {
        let mut out = SectorPaths::new(s);
        let mut storage_ids = SectorPaths::new(s);
        let storage_info_list = self.index.storage_find_sector(s, existing.into(), false);
        for info in storage_info_list {
            match self.paths.get(&info.id) {
                None => continue,
                Some(p) => {
                    if p.as_os_str().len() == 0 {
                        // TODO: can that even be the case?
                        continue;
                    }
                    let mut p = p.clone();
                    p.push(existing.to_string());
                    p.push(sector_name(s));
                    out.set_path_by_type(existing, sector_name(s));
                    storage_ids.set_path_by_type(existing, info.id.to_string());
                    break;
                }
            }
        }
        (out, storage_ids)
    }

    fn acquire_alloc_sector(
        &mut self,
        s: SectorId,
        spt: RegisteredProof,
        allocate: SectorFileType,
        sealing: bool,
    ) -> Result<(SectorPaths, SectorPaths)> {
        let mut out = SectorPaths::new(s);
        let mut storage_ids = SectorPaths::new(s);
        let storage_info_list = self.index.storage_best_alloc(allocate, spt, sealing)?;
        let mut best = PathBuf::new();
        let mut best_id: StorageId = Default::default();
        for info in storage_info_list {
            match self.paths.get(&info.id) {
                None => continue,
                Some(p) => {
                    if p.as_os_str().len() == 0 {
                        // TODO: can that even be the case?
                        continue;
                    }

                    if sealing && !info.can_seal {
                        continue;
                    }

                    if !sealing && !info.can_store {
                        continue;
                    }

                    // TODO: Check free space
                    let mut p = p.clone();
                    p.push(allocate.to_string());
                    p.push(sector_name(s));
                    best = p;
                    best_id = info.id;
                }
            }
        }
        if best.as_os_str().len() == 0 {
            return Err(StoresError::NoSuitablePath);
        }
        out.set_path_by_type(allocate, (*best.to_string_lossy()).to_string());
        storage_ids.set_path_by_type(allocate, best_id.to_string());

        Ok((out, storage_ids))
    }

    fn remove(&mut self, s: SectorId, single_type: SectorFileType) -> Result<()> {
        let storage_info_list = self.index.storage_find_sector(s, single_type.into(), false);
        if storage_info_list.is_empty() {
            return Err(StoresError::NotFoundSector(s, single_type));
        }
        for info in storage_info_list {
            match self.paths.get(&info.id) {
                None => continue,
                Some(p) => {
                    if p.as_os_str().len() == 0 {
                        // TODO: can that even be the case?
                        continue;
                    }
                    self.index
                        .storage_drop_sector(&info.id, s, single_type.into());
                    let mut p = p.clone();
                    p.push(single_type.to_string());
                    p.push(sector_name(s));
                    info!(target: TARGET, "remove {:?}", p.as_path());
                    if let Err(e) = fs::remove_dir_all(p.as_path()) {
                        error!(
                            target: TARGET,
                            "removing sector ({:?}) from {:?}: {:?}",
                            s,
                            p.as_path(),
                            e
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn move_storage(
        &mut self,
        s: SectorId,
        spt: RegisteredProof,
        single_type: SectorFileType,
    ) -> Result<()> {
        let (dest, dest_ids) = self.acquire_alloc_sector(s, spt, single_type, false)?;
        let (src, src_ids) = self.acquire_existing_sector(s, single_type);

        // let id = SectorPaths::new(s);
        let id = src_ids.path_by_type(single_type).ok_or(StoresError::Tmp)?;
        let sst = self.index.storage_info(&id.into())?;
        let id = dest_ids.path_by_type(single_type).ok_or(StoresError::Tmp)?;
        let dst = self.index.storage_info(&id.into())?;
        if sst.id == dst.id {
            debug!(
                target: TARGET,
                "not moving {:?}({:?}); src and dest are the same", s, single_type
            );
            return Ok(());
        }
        if sst.can_store {
            debug!(
                target: TARGET,
                "not moving {:?}({:?}); source supports storage", s, single_type
            );
            return Ok(());
        }
        debug!(
            target: TARGET,
            "moving {:?}({:?}) to storage: {}(se:{}; st:{}) -> {}(se:{}; st:{})",
            s,
            single_type,
            sst.id,
            sst.can_seal,
            sst.can_store,
            dst.id,
            dst.can_seal,
            dst.can_store
        );

        let id = src_ids.path_by_type(single_type).ok_or(StoresError::Tmp)?;
        self.index
            .storage_drop_sector(&id.into(), s, single_type.into());

        let from = Path::new(src.path_by_type(single_type).ok_or(StoresError::Tmp)?);
        let to_ = Path::new(dest.path_by_type(single_type).ok_or(StoresError::Tmp)?);
        crate::unix_utils::move_(from, to_)?;

        let id = dest_ids.path_by_type(single_type).ok_or(StoresError::Tmp)?;
        self.index
            .storage_declare_sector(&id.into(), s, single_type.into());
        Ok(())
    }

    fn fs_stat(&self, id: StorageId) -> Result<FsStat> {
        let path = self.paths.get(&id).ok_or(StoresError::PathNotFound(id))?;
        stat(path.as_path())
    }
}
