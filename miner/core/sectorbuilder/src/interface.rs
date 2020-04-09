// Copyright 2020 PolkaX

use cid::Cid;

use plum_actor::abi::{piece, sector};
use plum_types::SectorNumber;

pub trait Interface {
    fn add_piece(
        &self,
        size: piece::UnpaddedPieceSize,
        number: SectorNumber,
        piece_size: &[piece::UnpaddedPieceSize],
    ) -> Result<piece::PieceInfo, ()>;

    fn seal_pre_commit(
        &self,
        number: SectorNumber,
        ticket: sector::Randomness,
        pieces: piece::PieceInfo,
    ) -> Result<(Cid, Cid), ()>;

    fn seal_commit(
        &self,
        number: SectorNumber,
        ticket: sector::Randomness,
        seed: sector::Randomness,
        pieces: &[piece::PieceInfo],
        sealed_cid: Cid,
        unsealed_cid: Cid,
    ) -> Result<(sector::PoStProof), ()>;

    fn compute_election_post(
        sector_info: sector::SectorInfo,
        challengeSeed: sector::Randomness,
        winners: &[sector::PoStCandidate],
    ) -> Result<sector::PoStProof, ()>;

    fn finalize_sector(&self, number: SectorNumber);
}
