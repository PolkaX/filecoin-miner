mod commands;
mod error;
mod worker;

use commands::{Command, SubCommand};
use log::info;
use structopt::StructOpt;
use worker::Worker;

#[allow(clippy::large_enum_variant)]
pub enum TaskType {
    GenerateCandidates,
    GeneratePoSt,
    SealPreCommit,
    SealCommit,
    Unseal,
    Shutdown,
}
pub struct Limits {
    transfers: u32,
    workers: u32,
}

fn main() {
    info!("seal-worker");
    let opt = Command::from_args();
    match opt.cmd {
        SubCommand::Run(com) => {
            info!("{:?}", com);
        }
        _ => info!("{:?}", opt.cmd),
    }
}
