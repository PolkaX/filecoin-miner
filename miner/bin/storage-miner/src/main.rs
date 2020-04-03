mod commands;
mod error;
mod init;
mod params;
mod service;

use std::io;
use structopt::StructOpt;

use ds_rocksdb::DatabaseConfig;
use node_utils::other_io_err;
use plum_address::{set_network, Network};
use repo::{FsRepo, RepoType};

use crate::commands::{Command, RunCommand, SubCommand};
use error::*;

use log::{error, info};
use utils::native_log;

const METADATA_SPACE: &'static str = "/metadata";
const BLOCK_SPACE: &'static str = "/block";
const STAGING_SPACE: &'static str = "/staging";
const SECTORBUILDER_SPACE: &'static str = "/sectorbuilder";

const ALL_NAMESPACE: [&'static str; 4] = [
    METADATA_SPACE,
    BLOCK_SPACE,
    STAGING_SPACE,
    SECTORBUILDER_SPACE,
];

fn main() {
    let opt: Command = Command::from_args();
    if opt.testnet {
        unsafe {
            set_network(Network::Test);
        }
    };

    // todo after start mainnet, remove this line
    unsafe {
        set_network(Network::Test);
    }

    // init log
    native_log::init_logger(opt.log.as_ref().map(|v| v.as_ref()).unwrap_or(""));

    match opt.cmd {
        SubCommand::Run(_) | SubCommand::Init(_) => {
            let _ = run(opt).map_err(|e| error!("{:?}", e));
        }
        _ => info!("{:?}", opt.cmd),
    }
}

pub fn run(opt: Command) -> Result<()> {
    // create fsrepo
    // rocks database config
    let mut config =
        DatabaseConfig::with_columns(ALL_NAMESPACE.iter().map(|s| s.to_string()).collect());
    // todo set cache for different column in database config
    config.memory_budget = [(METADATA_SPACE.to_string(), 10)].iter().cloned().collect();

    // create repo
    let path = opt.repo_path.clone().into();

    match opt.cmd {
        SubCommand::Run(com) => {
            let repo = FsRepo::open(path, RepoType::StorageMiner, config)?;
            run_service(com, repo)?
        }
        SubCommand::Init(init) => init::run_init(init, path, config)?,
        _ => unreachable!("must be unreachable"),
    }
    Ok(())
}

pub fn run_service(com: RunCommand, repo: FsRepo) -> Result<()> {
    let locked_repo = repo.lock()?;
    let service = service::ServiceBuilder::new(locked_repo).build()?;
    node_service::run_service_until_exit(service).map_err(other_io_err)?;
    Ok(())
}
