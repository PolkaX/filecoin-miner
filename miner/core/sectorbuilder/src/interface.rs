// Copyright 2020 PolkaX

use actors::abi::{piece, sector};
use cid::Cid;

pub trait Interface {
    fn add_piece(
        &self,
        size: piece::UnpaddedPieceSize,
        number: sector::SectorNumber,
        piece_size: &[piece::UnpaddedPieceSize],
    ) -> Result<piece::PieceInfo, ()>;

    fn seal_pre_commit(
        &self,
        number: sector::SectorNumber,
        ticket: sector::Randomness,
        pieces: piece::PieceInfo,
    ) -> Result<(Cid, Cid), ()>;

    fn seal_commit(
        &self,
        number: sector::SectorNumber,
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

    fn finalize_sector(&self, number: sector::SectorNumber);
}
