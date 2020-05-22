// Copyright 2020 PolkaX

use crate::sealer::Sealer;
use anyhow::{bail, Result};
use filecoin_proofs_api::{seal, PieceInfo, UnpaddedBytesAmount};
use plum_sector::SectorId;
use specs_storage::Storage as StorageTrait;
use std::io::{Read, Seek, Write};

impl StorageTrait for Sealer {
    fn new_sector(&self, _sector: SectorId) -> Result<()> {
        bail!("")
    }

    fn add_piece<R, W>(
        &self,
        source: R,
        target: W,
        piece_sizes: UnpaddedBytesAmount,
        piece_lengths: &[UnpaddedBytesAmount],
    ) -> Result<(PieceInfo, UnpaddedBytesAmount)>
    where
        R: Read,
        W: Read + Write + Seek,
    {
        seal::add_piece(
            self.seal_proof_type,
            source,
            target,
            piece_sizes,
            piece_lengths,
        )
    }
}
