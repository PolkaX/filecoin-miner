// Copyright 2020 PolkaX

use crate::{interface::Interface, Batching, SectorBuilder};
use anyhow::{bail, Result};
use bytevec::ByteEncodable;
use cid::Cid;
use datastore::key::Key;
use filecoin_proofs_api::{seal::add_piece, PieceInfo, RegisteredSealProof, UnpaddedBytesAmount};
use plum_sector::{PoStProof, SectorInfo, SectorNumber, WinningPoStVerifyInfo};
use plum_types::Randomness;
use std::io::{Read, Seek, Write};

impl<DS: Batching> Interface for SectorBuilder<DS> {
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
        W: Read + Write + Seek,
    {
        add_piece(registered_proof, source, target, piece_size, piece_lengths)
    }

    fn seal_pre_commit(
        &self,
        number: SectorNumber,
        ticket: Randomness,
        pieces: plum_piece::PieceInfo,
    ) -> Result<(Cid, Cid)> {
        bail!("")
    }

    fn seal_commit(
        &self,
        number: SectorNumber,
        ticket: Randomness,
        seed: Randomness,
        pieces: &[plum_piece::PieceInfo],
        sealed_cid: Cid,
        unsealed_cid: Cid,
    ) -> Result<(PoStProof)> {
        bail!("")
    }

    fn compute_election_post(
        sector_info: SectorInfo,
        challengeSeed: Randomness,
        winners: &[WinningPoStVerifyInfo],
    ) -> Result<PoStProof> {
        bail!("")
    }

    fn finalize_sector(&self, number: SectorNumber) {}

    fn acquire_sector_id(&mut self) -> Result<u64> {
        self.last_id += 1;
        let id = self.last_id.encode::<u8>().unwrap();

        let last_sector_id_key: Key = Key::new("/last");
        self.ds.put(last_sector_id_key, id);
        Ok(self.last_id)
    }
}
