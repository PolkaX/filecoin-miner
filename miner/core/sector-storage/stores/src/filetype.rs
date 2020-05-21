use std::collections::HashMap;
use std::slice::Iter;

use bitmask::bitmask;

use plum_sector::{RegisteredProof, SectorId};

use crate::error::{Result, StoresError};

bitmask! {
    #[derive(Hash, Debug)]
    pub mask SectorFileTypes: i32 where

    #[derive(Hash, Debug)]
    flags SectorFileType {
        FTUnsealed = 1 << 0,
        FTSealed = 1 << 1,
        FTCache = 1 << 2,
    }
}

lazy_static::lazy_static! {
    static ref FS_OVERHEAD_SEAL: HashMap<SectorFileType, u64> = {
        let mut m = HashMap::new();
        m.insert(SectorFileType::FTUnsealed, 10);
        m.insert(SectorFileType::FTSealed, 10);
        m.insert(SectorFileType::FTCache, 10);
        m
    };
}

lazy_static::lazy_static! {
    static ref FS_OVERHEAD_FINALIZED: HashMap<SectorFileType, u64> = {
        let mut m = HashMap::new();
        m.insert(SectorFileType::FTUnsealed, 10);
        m.insert(SectorFileType::FTSealed, 10);
        m.insert(SectorFileType::FTCache, 2);
        m
    };
}

impl SectorFileTypes {
    pub fn iter() -> Iter<'static, SectorFileType> {
        static ITEMS: &[SectorFileType] = &[
            SectorFileType::FTUnsealed,
            SectorFileType::FTSealed,
            SectorFileType::FTCache,
        ];
        ITEMS.iter()
    }

    pub fn seal_space_use(&self, proof: RegisteredProof) -> u64 {
        let ssize = proof.sector_size();
        let mut need = 0;
        for t in Self::iter() {
            if !self.contains(*t) {
                continue;
            }
            let oh = FS_OVERHEAD_SEAL.get(t).expect("must exist");
            need += oh * ssize / 10;
        }
        need
    }
}

impl ToString for SectorFileTypes {
    fn to_string(&self) -> String {
        // todo trick to solve this... bug in go version
        if self.count_ones() != 1 {
            return format!("<unknown {}>", self.mask);
        }
        if self.contains(SectorFileType::FTUnsealed) {
            "unsealed".to_string()
        } else if self.contains(SectorFileType::FTSealed) {
            "sealed".to_string()
        } else {
            "cache".to_string()
        }
    }
}

impl ToString for SectorFileType {
    fn to_string(&self) -> String {
        match self {
            SectorFileType::FTUnsealed => "unsealed".to_string(),
            SectorFileType::FTSealed => "sealed".to_string(),
            SectorFileType::FTCache => "cache".to_string(),
        }
    }
}

pub struct SectorPaths {
    id: SectorId,
    pub paths: HashMap<SectorFileType, String>,
}
impl SectorPaths {
    pub fn new(id: SectorId) -> Self {
        SectorPaths {
            id,
            paths: Default::default(),
        }
    }

    pub fn sector_id(&self) -> &SectorId {
        &self.id
    }

    pub fn path_by_type(&self, type_: SectorFileType) -> Option<&str> {
        self.paths.get(&type_).map(AsRef::as_ref)
    }

    pub fn set_path_by_type(&mut self, type_: SectorFileType, p: String) {
        self.paths.insert(type_, p);
    }
}

lazy_static::lazy_static! {
    static ref TESTNET_RE: regex::Regex = regex::Regex::new(r"s-t0(\d+)-(\d+)").unwrap();
}
pub fn parse_sector_id(base_name: &str) -> Result<SectorId> {
    // TODO testnet, mainnet?
    let r = TESTNET_RE.captures(base_name).ok_or(StoresError::Tmp)?;
    if r.len() != 3 {
        return Err(StoresError::Tmp);
    }

    let actor_id: u64 = (&r[1]).parse().map_err(|_| StoresError::Tmp)?;
    let sector_number: u64 = (&r[2]).parse().map_err(|_| StoresError::Tmp)?;

    Ok(SectorId {
        miner: actor_id,
        number: sector_number,
    })
}

pub fn sector_name(sid: SectorId) -> String {
    // TODO testnet mainnet?
    format!("s-t0{}-{}", sid.miner, sid.number)
}
