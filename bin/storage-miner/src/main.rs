mod commands;
mod service;

use std::io;

use structopt::StructOpt;

use commands::{Command, SubCommand};

use crate::commands::RunCommand;
use repo::{FsRepo, RepoType};

fn main() {
    let opt = Command::from_args();
    println!("{:?}", opt);
    match opt.cmd {
        SubCommand::Run(ref com) => {
            // TODO exit early
            let _ = run_command(opt).map_err(|e| eprintln!("{:?}", e));
        }
        SubCommand::Init(ref init) => {
            println!("{:?}", init);
        }
        _ => println!("{:?}", opt.cmd),
    }
}

pub fn run_command(com: Command) -> io::Result<()> {
    // create fsrepo
    // rocks database config
    use ds_rocksdb::DatabaseConfig;;
    // todo config
    let config = DatabaseConfig::default();
    let path = com.repo_path.clone().into();
    let repo = FsRepo::open(path, RepoType::StorageMiner, config)?;
    let locked_repo = repo.lock()?;
    let service = service::ServiceBuilder::new(locked_repo).build();
    node_service::run_service_until_exit(service);
    Ok(())
}
