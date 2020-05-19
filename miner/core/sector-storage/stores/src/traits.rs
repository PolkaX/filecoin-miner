use nix::sys::statfs::statfs;
use std::path::Path;

use plum_sector::{RegisteredProof, SectorId};

use crate::error::Result;
use crate::filetype::{SectorFileType, SectorPaths};
use crate::index::StorageId;

pub trait Store {
    fn acquire_existing_sector(
        &self,
        s: SectorId,
        single_type: SectorFileType,
    ) -> (SectorPaths, SectorPaths);

    fn acquire_alloc_sector(
        &mut self,
        s: SectorId,
        spt: RegisteredProof,
        allocate: SectorFileType,
        sealing: bool,
    ) -> Result<(SectorPaths, SectorPaths)>;

    fn remove(&mut self, s: SectorId, single_type: SectorFileType) -> Result<()>;

    // move sectors into storage
    fn move_storage(
        &mut self,
        s: SectorId,
        spt: RegisteredProof,
        single_type: SectorFileType,
    ) -> Result<()>;

    fn fs_stat(&self, id: StorageId) -> Result<FsStat>;
}

pub fn stat(path: &Path) -> Result<FsStat> {
    let stat = statfs(path)?;
    Ok(FsStat {
        capacity: stat.blocks() * (stat.block_size() as u64),
        available: stat.blocks_available() * (stat.block_size() as u64),
        used: 0,
    })
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct FsStat {
    pub capacity: u64,
    pub available: u64,
    // Available to use for sector storage
    pub used: u64,
}
