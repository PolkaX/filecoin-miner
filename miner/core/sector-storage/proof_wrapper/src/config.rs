// Copyright 2020 PolkaX

use anyhow::{bail, Result};
use plum_sector::{RegisteredProof, SectorSize};

#[derive(Eq, PartialEq)]
pub struct Config {
    pub seal_proof_type: RegisteredProof,
}

pub fn size_from_config(cfg: Config) -> Result<SectorSize> {
    Ok(cfg.seal_proof_type.sector_size())
}

pub fn seal_proof_type_from_sector_size(ssize: SectorSize) -> Result<RegisteredProof> {
    let _two_kb: SectorSize = 2 << 10;
    let _eight_mb = 8 << 20;
    let _five_one_two_mb = 512 << 20;
    let _third_two_gb = 32 << 30;
    if ssize == 2 << 10 {
        return Ok(RegisteredProof::StackedDRG2KiBSeal);
    } else if ssize == 8 << 20 {
        return Ok(RegisteredProof::StackedDRG8MiBSeal);
    } else if ssize == 512 << 20 {
        return Ok(RegisteredProof::StackedDRG512MiBSeal);
    } else if ssize == 32 << 30 {
        return Ok(RegisteredProof::StackedDRG32GiBSeal);
    }
    bail!("")
}
