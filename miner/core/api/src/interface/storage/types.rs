// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};

pub type SectorState = String;
/*
use serde_repr::{Deserialize_repr, Serialize_repr};
///
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum SectorState {
    ///
    Undefined = 0,
    ///
    Empty = 1,
    ///
    Packing = 2,
    ///
    Unsealed = 3,
    ///
    PreCommitting = 4,
    ///
    WaitSeed = 5,
    ///
    Committing = 6,
    ///
    CommitWait = 7,
    ///
    FinalizeSector = 8,
    ///
    Proving = 9,

    ///
    FailedUnrecoverable = 20,
    ///
    SealFailed = 21,
    ///
    PreCommitFailed = 22,
    ///
    SealCommitFailed = 23,
    ///
    CommitFailed = 24,
    ///
    PackingFailed = 25,

    ///
    Faulty = 29,
    ///
    FaultReported = 30,
    ///
    FaultedFinal = 31,
}
*/

/*
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectorInfo {
    ///
    pub sector_id: u64,
    ///
    pub state: SectorState,
    ///
    pub comm_d: Vec<u8>,
    ///
    pub comm_r: Vec<u8>,
    ///
    pub proof: Vec<u8>,
    ///
    pub deals: Vec<u64>,
    ///
    pub ticket: sectorbuilder::SealTicket,
    ///
    pub seed: sectorbuilder::SealSeed,
    ///
    pub retries: u64,
    ///
    pub last_err: String,
    ///
    pub log: Vec<SectorLog>,
}

///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectorLog {
    ///
    pub kind: String,
    ///
    pub timestamp: u64,
    ///
    pub trace: String,
    ///
    pub message: String,
}
*/

///
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealedRef {
    ///
    #[serde(rename = "SectorID")]
    pub sector_id: u64,
    ///
    pub offset: u64,
    ///
    pub size: u64,
}

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SealedRefs {
    ///
    pub refs: Vec<SealedRef>,
}
