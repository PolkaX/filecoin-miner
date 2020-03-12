// Copyright 2020 PolkaX

use crate::SectorState;
use cid::Cid;
use filecoin_proofs_api::{ChallengeSeed, Commitment, PieceInfo, Ticket};

pub type Piece = PieceInfo;
type SealTicket = Ticket;
type SealSeed = ChallengeSeed;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SectorInfo {
    pub state: SectorState,
    sector_id: u64,
    nonce: u64,

    pieces: Vec<Piece>,
    commd: Commitment,
    commr: Commitment,
    proof: Commitment,
    ticket: SealTicket,

    pre_commit_msg: Cid,
    seed: SealSeed,
    commit_msg: Cid,
    fault_report_msg: Cid,
}

impl SectorInfo {
    pub fn new() -> Self {
        SectorInfo {
            state: SectorState::Empty,
            sector_id: 0,
            nonce: 0,
            pieces: vec![],
            commd: [0; 32],
            commr: [0; 32],
            proof: [0; 32],
            ticket: [0; 32],
            pre_commit_msg: cid::zero_cid(),
            seed: [0; 32],
            commit_msg: cid::zero_cid(),
            fault_report_msg: cid::zero_cid(),
        }
    }
}
