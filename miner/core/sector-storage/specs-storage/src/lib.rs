// Copyright 2020 PolkaX

use cid::Cid;
use plum_piece::UnpaddedPieceSize;
use plum_sector::{PoStProof, SectorId, SectorInfo};
use plum_types::{Randomness, ActorId};
use filecoin_proofs_api::{seal, PieceInfo, seal::SealPreCommitPhase2Output};
use anyhow::Result;

pub trait Storage<R: std::io::Read> {
    fn new_sector(sector: SectorId) -> Result<()>;

    fn add_piece(
        sector: SectorId,
        piece_siezes: UnpaddedPieceSize,
        new_piece_size: UnpaddedPieceSize,
        piece_data: std::io::BufReader<R>,
    ) -> Result<PieceInfo>;
}

pub trait Prover {
    fn generate_winning_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof>;

    fn generate_window_post(
        miner_id: ActorId,
        sector_info: &[SectorInfo],
        randomness: Randomness,
    ) -> Result<PoStProof>;
}

pub type PreCommit1Out = seal::SealPreCommitPhase1Output;
pub type Commit1Out = seal::SealCommitPhase1Output;
pub type Proof = seal::SealCommitPhase2Output;
pub type InteractiveSealRandomness = [u8; 32];
pub type SealRandomness = [u8; 32];

pub struct SectorCids {
    pub unsealed: Cid,
    pub sealed: Cid,
}

pub trait Sealer {
    fn seal_pre_commit1(
        &mut self,
        sector: SectorId,
        ticket: SealRandomness,
        pieces: &[PieceInfo],
    ) -> Result<PreCommit1Out>;
    fn seal_pre_commit2(&mut self, sector: SectorId, pc1o: PreCommit1Out) -> Result<SealPreCommitPhase2Output>;

    fn seal_commit1(
        &mut self, 
        sector: SectorId,
        ticket: SealRandomness,
        seed: InteractiveSealRandomness,
        pieces: &[PieceInfo],
        pco2: SealPreCommitPhase2Output,
    ) -> Result<Commit1Out>;
    fn seal_commit2(&mut self, sector: SectorId, c1o: Commit1Out) -> Result<Proof>;
    fn finalize_sector(&mut self, sector: SectorId) -> Result<()>;
}
