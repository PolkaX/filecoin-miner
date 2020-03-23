// Copyright 2020 PolkaX

pub mod fs;
pub mod interface;
#[cfg(test)]
mod test;
mod types;

use filecoin_proofs_api::{
    seal::{seal_pre_commit_phase1, seal_pre_commit_phase2, SealPreCommitPhase2Output},
    PieceInfo, RegisteredSealProof, Ticket,
};

use datastore::Batching;

use plum_actor::abi::sector::SectorSize;
use plum_address::Address;

pub use self::types::Config;
pub use filecoin_proofs_api::fr32;
pub use filecoin_proofs_api::Candidate as EPostCandidate;

#[derive(Debug)]
pub struct SectorBuilder<DS: Batching> {
    ds: DS,
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

impl<DS: Batching> SectorBuilder<DS> {
    pub fn new(cfg: &Config, datastore: DS) -> Self {
        let sector_builder = SectorBuilder {
            ds: datastore,
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
    ) -> Result<SealPreCommitPhase2Output, anyhow::Error> {
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
        let phase1 = seal_pre_commit_phase1(
            RegisteredSealProof::StackedDrg2KiBV1,
            cache_dir.clone(),
            staged_path,
            sealed_path.clone(),
            prover_id,
            sector_id.into(),
            ticket,
            pieces,
        )
        .unwrap();
        seal_pre_commit_phase2(phase1, cache_dir, sealed_path)
    }
}
