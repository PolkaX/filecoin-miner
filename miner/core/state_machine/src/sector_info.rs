// Copyright 2020 PolkaX

use cid::{Cid, Codec};
use filecoin_proofs_api::{ChallengeSeed, Commitment, PieceInfo, Ticket};

use crate::SectorState;

pub type Piece = PieceInfo;
type SealTicket = Ticket;
type SealSeed = ChallengeSeed;

fn zero_cid() -> Cid {
    Cid::new_v1(Codec::Raw, multihash::Identity::digest(b""))
}

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
            pre_commit_msg: zero_cid(),
            seed: [0; 32],
            commit_msg: zero_cid(),
            fault_report_msg: zero_cid(),
        }
    }
}
