use crate::service;
use anyhow::Result;
use node_utils::other_io_err;
use repo::FsRepo;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Run {
    #[structopt(long, default_value = "127.0.0.1:2345")]
    pub api: String,
    /// Don't check full_node sync status
    #[structopt(long)]
    pub no_sync: bool,
    /// Enable use of GPU for mining operations
    #[structopt(long)]
    pub gpu_proving: bool,
}

impl Run {
    pub fn run(&self, repo: FsRepo) -> Result<()> {
        let locked_repo = repo.lock()?;
        let service = service::ServiceBuilder::new(locked_repo).build()?;
        node_service::run_service_until_exit(service).map_err(other_io_err)?;
        Ok(())
    }
}
