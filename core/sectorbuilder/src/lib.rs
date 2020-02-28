// Copyright 2020 PolkaX

mod fs;
#[cfg(test)]
mod test;
mod types;

use filecoin_proofs_api::SectorSize;
use filecoin_proofs_api::{
    seal_pre_commit, PieceInfo, RegisteredSealProof, SealPreCommitResponse, Ticket,
};
use plum_address::Address;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use types::Config;

#[derive(Debug)]
pub struct SectorBuilder {
    ds: BTreeMap<String, String>,
    ssize: u64,
    last_id: u64,
    miner: Address,
    no_commit: bool,
    no_pre_commit: bool,

    add_piece_wait: i32,
    pre_commit_wait: i32,
    commit_wait: i32,
    unseal_wait: i32,
    file_system: fs::FS,
}

impl SectorBuilder {
    pub fn New(cfg: &Config) -> Self {
        let sector_builder = SectorBuilder {
            ds: BTreeMap::new(),
            ssize: cfg.sector_size,
            last_id: 0, // ds.get_last_sector_id.
            miner: cfg.miner.clone(),
            no_commit: true,
            no_pre_commit: true,
            add_piece_wait: 0,
            pre_commit_wait: 0,
            commit_wait: 0,
            unseal_wait: 0,
            file_system: fs::FS::new(&cfg.paths),
        };
        sector_builder
    }

    fn seal_pre_commit(
        &mut self,
        sector_id: u64,
        ticket: Ticket,
        pieces: &[PieceInfo],
    ) -> SealPreCommitResponse {
        let cache_dir = self
            .file_system
            .force_alloc_sector(
                fs::DataType::Cache,
                self.miner.clone(),
                self.ssize,
                true,
                sector_id,
            )
            .unwrap();
        let sealed_path = self
            .file_system
            .force_alloc_sector(
                fs::DataType::Sealed,
                self.miner.clone(),
                self.ssize,
                true,
                sector_id,
            )
            .unwrap();
        /*let mut sum = 0;
        pieces.iter().map(|pieces_info| sum += pieces_info.size);*/
        let staged_path = self
            .file_system
            .force_alloc_sector(
                fs::DataType::Staging,
                self.miner.clone(),
                self.ssize,
                true,
                sector_id,
            )
            .unwrap();
        let prover_id = [0; 32];
        seal_pre_commit(
            RegisteredSealProof::StackedDrg2KiBV1,
            cache_dir,
            staged_path,
            sealed_path,
            prover_id,
            sector_id.into(),
            ticket,
            pieces,
        )
    }
}
