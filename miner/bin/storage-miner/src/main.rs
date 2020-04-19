mod command;
mod error;
mod service;

use crate::command::StorageMiner;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let storage_miner = StorageMiner::from_args();
    storage_miner.run()?;
    Ok(())
}
