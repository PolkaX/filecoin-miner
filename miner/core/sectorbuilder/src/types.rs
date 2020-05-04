// Copyright 2020 PolkaX

use crate::fs;
use plum_actor::abi::piece;
use plum_address::Address;

#[derive(Debug, Clone)]
pub struct Config {
    pub sector_size: u64,
    pub miner: Address,
    pub worker_threads: u8,
    pub fall_back_last_id: u64,
    pub no_commit: bool,
    pub no_pre_commit: bool,
    pub paths: Vec<fs::PathConfig>,
}

pub fn user_bytes_for_sector_size(ssize: u64) -> piece::UnpaddedPieceSize {
    piece::PaddedPieceSize::new(ssize).unwrap().unpadded()
}
