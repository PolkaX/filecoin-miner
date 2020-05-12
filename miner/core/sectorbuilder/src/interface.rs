// Copyright 2020 PolkaX

use anyhow::Result;
use cid::Cid;
use filecoin_proofs_api::{PieceInfo, RegisteredSealProof, UnpaddedBytesAmount};
use plum_sector::{SectorNumber, SectorInfo, PoStProof, WinningPoStVerifyInfo};
use plum_types::Randomness;
use std::io::{Read, Seek, Write};

pub trait Interface {
    fn add_piece<R, W>(
        &self,
        registered_proof: RegisteredSealProof,
        source: R,
        target: W,
        piece_size: UnpaddedBytesAmount,
        piece_lengths: &[UnpaddedBytesAmount],
    ) -> Result<(PieceInfo, UnpaddedBytesAmount)>
    where
        R: Read,
        W: Read + Write + Seek;

    fn seal_pre_commit(
        &self,
        number: SectorNumber,
        ticket: Randomness,
        pieces: plum_piece::PieceInfo,
    ) -> Result<(Cid, Cid)>;

    fn seal_commit(
        &self,
        number: SectorNumber,
        ticket: Randomness,
        seed: Randomness,
        pieces: &[plum_piece::PieceInfo],
        sealed_cid: Cid,
        unsealed_cid: Cid,
    ) -> Result<(PoStProof)>;

    fn compute_election_post(
        sector_info: SectorInfo,
        challengeSeed: Randomness,
        winners: &[WinningPoStVerifyInfo],
    ) -> Result<PoStProof>;

    fn finalize_sector(&self, number: SectorNumber);

    fn acquire_sector_id(&mut self) -> Result<u64>;
}
