mod commands;
mod init;
mod params;
mod service;
mod util;

use std::io;

use ds_rocksdb::DatabaseConfig;
use log::{error, info};
use node_utils::other_io_err;
use repo::{FsRepo, RepoType};
use structopt::StructOpt;

use crate::commands::{Command, RunCommand, SubCommand};

const METADATA_SPACE: &'static str = "/metadata";
const BLOCK_SPACE: &'static str = "/block";
const STAGING_SPACE: &'static str = "/staging";

const ALL_NAMESPACE: [&'static str; 3] = [METADATA_SPACE, BLOCK_SPACE, STAGING_SPACE];

fn main() {
    let opt: Command = Command::from_args();

    // init log
    util::init_logger(opt.log.as_ref().map(|v| v.as_ref()).unwrap_or(""));

    match opt.cmd {
        SubCommand::Run(_) | SubCommand::Init(_) => {
            let _ = run(opt).map_err(|e| error!("{:?}", e));
        }
        _ => info!("{:?}", opt.cmd),
    }
}

pub fn run(opt: Command) -> io::Result<()> {
    // create fsrepo
    // rocks database config
    let config =
        DatabaseConfig::with_columns(ALL_NAMESPACE.iter().map(|s| s.to_string()).collect());
    // todo set cache for different column in database config

    // create repo
    let path = opt.repo_path.clone().into();

    match opt.cmd {
        SubCommand::Run(com) => {
            let repo = FsRepo::open(path, RepoType::StorageMiner, config)?;
            run_service(com, repo)
        }
        SubCommand::Init(init) => init::run_init(init, path, config),
        _ => unreachable!("must be unreachable"),
    }
}

pub fn run_service(com: RunCommand, repo: FsRepo) -> io::Result<()> {
    let locked_repo = repo.lock()?;
    let service = service::ServiceBuilder::new(locked_repo).build();
    node_service::run_service_until_exit(service).map_err(other_io_err)?;
    Ok(())
}
