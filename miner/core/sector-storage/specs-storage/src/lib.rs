// Copyright 2020 PolkaX

use anyhow::Result;
use cid::Cid;
use filecoin_proofs_api::{
    seal, seal::SealPreCommitPhase2Output, ChallengeSeed, PieceInfo, PrivateReplicaInfo, ProverId,
    RegisteredPoStProof, SnarkProof, UnpaddedBytesAmount, SectorId as SectorNumber,
};
use plum_sector::SectorId;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};

pub trait Storage {
    fn new_sector(&self, sector: SectorId) -> Result<()>;

    fn add_piece<R, W>(
        &self,
        source: R,
        target: W,
        piece_siezes: UnpaddedBytesAmount,
        new_piece_size: &[UnpaddedBytesAmount],
    ) -> Result<(PieceInfo, UnpaddedBytesAmount)>
    where
        R: Read,
        W: Read + Write + Seek;
}

pub trait Prover {
    fn generate_winning_post(
        randomness: &ChallengeSeed,
        replicas: &BTreeMap<SectorNumber, PrivateReplicaInfo>,
        prover_id: ProverId,
    ) -> Result<Vec<(RegisteredPoStProof, SnarkProof)>>;

    fn generate_window_post(
        randomness: &ChallengeSeed,
        replicas: &BTreeMap<SectorNumber, PrivateReplicaInfo>,
        prover_id: ProverId,
    ) -> Result<Vec<(RegisteredPoStProof, SnarkProof)>>;
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
    fn seal_pre_commit2(
        &mut self,
        sector: SectorId,
        pc1o: PreCommit1Out,
    ) -> Result<SealPreCommitPhase2Output>;

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
