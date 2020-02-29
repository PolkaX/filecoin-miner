use std::fs;
use std::io;
use std::path::PathBuf;

use ds_rocksdb::DatabaseConfig;
use repo::{FsRepo, RepoType};

use crate::commands::InitCommand;
use crate::error::*;
use crate::params;

use log::{error, info, warn};

pub fn run_init(com: InitCommand, path: PathBuf, config: DatabaseConfig) -> Result<()> {
    info!("Initializing lotus storage miner");

    let ssize = com.sector_size.unwrap_or(params::SECTOR_SIZES[0]);
    let sym_link = com.symlink_imported_sectors;

    info!("Checking proof parameters");

    info!("Trying to connect to full node RPC");

    // todo check version
    info!("Checking full node version");

    info!("Initializing repo");
    let repo = FsRepo::init(path.to_owned(), RepoType::StorageMiner, config)?;
    let repo = if let Some(repo) = repo {
        repo
    } else {
        warn!("repo at '{}' is already initialized", path.display());
        return Ok(());
    };

    if let Some(pre) = com.pre_sealed_sectors {
        // TODO
    }

    info!("start storage miner init");
    storage_miner_init().map_err(|e| {
        error!("Failed to initialize miner: {:?}", e);
        info!("Cleaning up %s after attempt... {:}", path.display());
        let _ = fs::remove_dir_all(path.as_path())
            .map_err(|e| error!("Failed to clean up failed storage repo: {:?}", e));
        e
    })?;
    info!("Storage miner successfully created, you can now start it with 'run'");
    Ok(())
}

fn storage_miner_init() -> io::Result<()> {
    // TODO
    Ok(())
}
