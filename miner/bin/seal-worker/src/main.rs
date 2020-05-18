mod commands;
mod error;
mod worker;

use log::info;
use structopt::StructOpt;

use plum_api_client::{CommonApi, HttpTransport};

use utils::native_log;

use self::commands::{Command, SubCommand};
use self::worker::Worker;

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
    native_log::init_logger(opt.log.as_ref().map(|v| v.as_ref()).unwrap_or(""));
    match opt.cmd {
        SubCommand::Run(com) => {
            info!("{:?}", com);
            let storage_api = HttpTransport::new("http://127.0.0.1:1234/rpc/v0");
            let version = async_std::task::block_on(async { storage_api.version().await.unwrap() });
            info!("version:{:?}", version);
        }
        _ => info!("{:?}", opt.cmd),
    }
}
