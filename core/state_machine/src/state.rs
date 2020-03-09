// Copyright 2020 PolkaX

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectorState {
    UndefinedSectorState,
    Empty,
    Packing,
    Unsealed,
    PreCommitting,
    WaitSeed,
    Committing,
    CommitWait,
    FinalizeSector,
    Proving,
    SealFailed,
    PreCommitFailed,
    SealCommitFailed,
    CommitFailed,
    PackingFailed,
    FailedUnrecoverable,
    Faulty,
    FaultReported,
    FaultedFinal,
}
