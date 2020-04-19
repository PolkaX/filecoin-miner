// Copyright 2020 PolkaX

use anyhow::Result;
use cid::Cid;
use filecoin_proofs_api::{PieceInfo, RegisteredSealProof, UnpaddedBytesAmount};
use plum_actor::abi::{piece, sector};
use plum_types::SectorNumber;
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
        ticket: sector::Randomness,
        pieces: piece::PieceInfo,
    ) -> Result<(Cid, Cid)>;

    fn seal_commit(
        &self,
        number: SectorNumber,
        ticket: sector::Randomness,
        seed: sector::Randomness,
        pieces: &[piece::PieceInfo],
        sealed_cid: Cid,
        unsealed_cid: Cid,
    ) -> Result<(sector::PoStProof)>;

    fn compute_election_post(
        sector_info: sector::SectorInfo,
        challengeSeed: sector::Randomness,
        winners: &[sector::PoStCandidate],
    ) -> Result<sector::PoStProof>;

    fn finalize_sector(&self, number: SectorNumber);

    fn acquire_sector_id(&mut self) -> Result<u64>;
}
