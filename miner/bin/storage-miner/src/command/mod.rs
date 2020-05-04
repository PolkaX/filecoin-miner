mod info;
mod init;
mod run;
mod sectors;

use anyhow::Result;
use ds_rocksdb::DatabaseConfig;
use plum_address::{set_network, Network};
use repo::{FsRepo, RepoType};
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;
use utils::consts::{ALL_NAMESPACE, METADATA_SPACE};
use utils::native_log;

#[derive(Clone, Debug)]
pub struct RepoPath(pub PathBuf);

impl Into<PathBuf> for RepoPath {
    fn into(self) -> PathBuf {
        self.0
    }
}

impl<T: ?Sized + AsRef<OsStr>> From<&T> for RepoPath {
    fn from(s: &T) -> RepoPath {
        RepoPath(PathBuf::from(s.as_ref().to_os_string()))
    }
}

impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.display(), f)
    }
}

impl Default for RepoPath {
    fn default() -> Self {
        let mut home = dirs::home_dir().unwrap();
        home.push(".rust-fil-miner");
        RepoPath(home)
    }
}

/// Filecoin decentralized storage network storage miner
#[derive(StructOpt, Debug)]
pub struct StorageMiner {
    /// Set a custom logging filter.
    #[structopt(short, long)]
    pub testnet: bool,
    /// Set a custom logging filter.
    #[structopt(short, long, value_name = "LOG_PATTERN")]
    pub log: Option<String>,
    #[structopt(long, default_value, parse(from_os_str))]
    pub repo_path: RepoPath,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Print storage miner info
    Info,
    /// Start a storage miner process
    Run(crate::command::run::Run),
    /// Initialize a storage miner repo
    Init(crate::command::init::Init),
    /// Interact with sector store
    Sectors(crate::command::sectors::Sectors),
}

impl StorageMiner {
    pub fn run(&self) -> Result<()> {
        if self.testnet {
            unsafe {
                set_network(Network::Test);
            }
        };

        // TODO: after start mainnet, remove this line
        unsafe {
            set_network(Network::Test);
        }

        // init log
        native_log::init_logger(self.log.as_ref().map(|v| v.as_ref()).unwrap_or(""));

        match &self.command {
            Command::Run(r) => {
                let repo = FsRepo::open(
                    self.repo_path.clone().into(),
                    RepoType::StorageMiner,
                    self.db_config(),
                )?;
                r.run(repo)?;
            }
            Command::Init(init) => init.run(self.repo_path.clone().into(), self.db_config())?,
            Command::Sectors(sectors) => sectors.run()?,
            Command::Info => crate::command::info::run(),
        }

        Ok(())
    }

    fn db_config(&self) -> DatabaseConfig {
        // create fsrepo
        // rocks database config
        let mut config =
            DatabaseConfig::with_columns(ALL_NAMESPACE.iter().map(|s| s.to_string()).collect());
        // TODO: set cache for different column in database config
        config.memory_budget = [(METADATA_SPACE.to_string(), 10)].iter().cloned().collect();
        config
    }
}
