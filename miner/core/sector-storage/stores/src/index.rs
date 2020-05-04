use std::collections::HashMap;

use url::Url;

use plum_actor::abi::sector::{SectorId, SectorInfo};
use plum_bigint::BigUint;

use log::{info, warn};

use crate::error::{Result, StoresError};
use crate::filetype::{sector_name, SectorFileType, SectorFileTypes};
use crate::TARGET;

pub type StorageId = String; // todo may use uuid replace string

pub struct Index {
    sectors: HashMap<Decl, Vec<StorageId>>,
    stores: HashMap<StorageId, StorageEntry>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            sectors: Default::default(),
            stores: Default::default(),
        }
    }

    pub fn storage_list(&self) -> HashMap<&StorageId, Vec<Decl>> {
        let mut by_id: HashMap<&StorageId, HashMap<SectorId, SectorFileTypes>> = self
            .stores
            .iter()
            .map(|(k, _)| (k, Default::default()))
            .collect();
        for (k, v) in self.sectors.iter() {
            for i in v.iter() {
                let e = by_id.entry(i).or_default();
                let types = e.entry(k.sector_id).or_insert(k.types);
                *types |= k.types; // maybe have same id in `sectors`.
            }
        }
        by_id
            .into_iter()
            .map(|(k, v)| {
                let decls = v
                    .into_iter()
                    .map(|(sector_id, types)| Decl { sector_id, types })
                    .collect::<Vec<_>>();
                (k, decls)
            })
            .collect()
    }

    // create new storage entry
    pub fn storage_attach(&mut self, si: StorageInfo) -> Result<()> {
        info!(target: TARGET, "New sector storage: {:}", si.id);
        match self.stores.get_mut(&si.id) {
            Some(entry) => {
                for url in si.urls.iter() {
                    match entry.info.urls.iter().find(|cur| *cur == url) {
                        Some(_) => continue,
                        None => entry.info.urls.push(url.clone()),
                    }
                }
            }
            None => {
                self.stores.insert(si.id.clone(), StorageEntry { info: si });
            }
        }
        Ok(())
    }

    // declare just modify sectors
    pub fn storage_declare_sector(
        &mut self,
        storage_id: &StorageId,
        sector_id: SectorId,
        types: SectorFileTypes,
    ) {
        for t in SectorFileTypes::iter() {
            if !types.contains(*t) {
                continue;
            }
            let d = Decl {
                sector_id,
                types: (*t).into(),
            };
            let ids = self.sectors.entry(d).or_default();

            match ids.iter().find(|id| *id == storage_id) {
                None => ids.push(storage_id.clone()),
                Some(_) => {
                    // TODO way return in go?? maybe other type do not contain this storage_id
                    warn!(
                        target: TARGET,
                        "sector {:?} redeclared in {}", sector_id, storage_id
                    );
                }
            }
        }
    }

    pub fn storage_drop_sector(
        &mut self,
        storage_id: &StorageId,
        sector_id: SectorId,
        types: SectorFileTypes,
    ) {
        for t in SectorFileTypes::iter() {
            if !types.contains(*t) {
                continue;
            }
            let d = Decl {
                sector_id,
                types: (*t).into(),
            };
            if let Some(v) = self.sectors.get_mut(&d) {
                // remove all match storage id
                v.retain(|id| id != storage_id);
                if v.len() == 0 {
                    self.sectors.remove(&d);
                }
            }
        }
    }

    pub fn storage_find_sector(
        &self,
        sector_id: SectorId,
        types: SectorFileTypes,
        allow: bool,
    ) -> Vec<StorageInfo> {
        let mut storage_ids: HashMap<StorageId, usize> = Default::default();
        let d = Decl { sector_id, types };
        for t in SectorFileTypes::iter() {
            if !types.contains(*t) {
                continue;
            }
            if let Some(v) = self.sectors.get(&d) {
                for id in v.iter() {
                    match storage_ids.get_mut(id) {
                        Some(d) => *d += 1,
                        None => {
                            storage_ids.insert(id.clone(), 1);
                        }
                    }
                }
            }
        }

        // TODO types.to_string() may make error when types is not single type
        let get_urls = |entry: &StorageEntry| -> Vec<Url> {
            entry
                .info
                .urls
                .iter()
                .map(|url| {
                    url.clone()
                        .join(&types.to_string())
                        .unwrap()
                        .join(&sector_name(sector_id))
                        .unwrap()
                })
                .collect::<Vec<_>>()
        };

        let mut out = vec![];
        for (id, n) in storage_ids.iter() {
            match self.stores.get(id) {
                None => {
                    warn!(
                        target: TARGET,
                        "storage {} is not present in sector index (referenced by sector {:?})",
                        id,
                        sector_id
                    );
                    continue;
                }
                Some(entry) => out.push(StorageInfo {
                    id: id.clone(),
                    urls: get_urls(entry),
                    weight: entry.info.weight * (*n as u64),
                    can_seal: entry.info.can_seal,
                    can_store: entry.info.can_store,
                }),
            }
        }

        if allow {
            for (k, entry) in self.stores.iter() {
                match storage_ids.get(k) {
                    None => continue,
                    Some(n) => out.push(StorageInfo {
                        id: k.clone(),
                        urls: get_urls(entry),
                        weight: entry.info.weight * 0, // TODO: something better than just '0'
                        can_seal: entry.info.can_seal,
                        can_store: entry.info.can_store,
                    }),
                }
            }
        }
        out
    }

    pub fn storage_info(&self, storage_id: &StorageId) -> Result<StorageInfo> {
        self.stores
            .get(storage_id)
            .map(|enter| enter.info.clone())
            .ok_or(StoresError::Tmp)
    }

    pub fn storage_best_allow(
        &self,
        _allocate: SectorFileType,
        sealing: bool,
    ) -> Result<Vec<StorageInfo>> {
        let mut candidates = self
            .stores
            .iter()
            .filter(|(_, entey)| {
                if sealing && !entey.info.can_seal {
                    return false;
                }
                if !sealing && !entey.info.can_store {
                    return false;
                }
                true
            })
            .map(|(_, entey)| entey)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            return Err(StoresError::Tmp);
        }

        candidates.sort_by(|a, b| {
            let iw = BigUint::from(a.info.weight);
            let jw = BigUint::from(b.info.weight);
            // great then
            iw.cmp(&jw).reverse()
        });
        Ok(candidates
            .into_iter()
            .map(|entry| entry.info.clone())
            .collect())
    }

    pub fn find_sector(
        &self,
        sector_id: SectorId,
        single_type: SectorFileType,
    ) -> Option<Vec<StorageId>> {
        self.sectors
            .get(&Decl {
                sector_id,
                types: single_type.into(),
            })
            .map(|ids| ids.clone())
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub struct Decl {
    sector_id: SectorId,
    types: SectorFileTypes,
}

#[derive(Clone)]
pub struct StorageEntry {
    info: StorageInfo,
    // fsi  :FsStat,
}

#[derive(Clone)]
pub struct StorageInfo {
    id: StorageId,
    urls: Vec<Url>,
    // TODO: Support non-http transports
    weight: u64,

    can_seal: bool,
    can_store: bool,
}
