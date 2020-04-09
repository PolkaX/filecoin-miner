// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{de, ser, Deserialize, Serialize};

///
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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

impl ser::Serialize for SectorState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for SectorState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match u8::deserialize(deserializer)? {
            0 => SectorState::Undefined,
            1 => SectorState::Empty,
            2 => SectorState::Packing,
            3 => SectorState::Unsealed,
            4 => SectorState::PreCommitting,
            5 => SectorState::WaitSeed,
            6 => SectorState::Committing,
            7 => SectorState::CommitWait,
            8 => SectorState::FinalizeSector,
            9 => SectorState::Proving,
            20 => SectorState::FailedUnrecoverable,
            21 => SectorState::SealFailed,
            22 => SectorState::PreCommitFailed,
            23 => SectorState::SealCommitFailed,
            24 => SectorState::CommitFailed,
            25 => SectorState::PackingFailed,
            29 => SectorState::Faulty,
            30 => SectorState::FaultReported,
            31 => SectorState::FaultedFinal,
            i => return Err(de::Error::custom(format!("unexpect integer {}", i))),
        })
    }
}

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
#[serde(rename = "PascalCase")]
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
#[serde(rename = "PascalCase")]
pub struct SealedRefs {
    ///
    pub refs: Vec<SealedRef>,
}
