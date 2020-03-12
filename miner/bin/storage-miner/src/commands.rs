use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug)]
pub struct RepoPath(pub PathBuf);
impl RepoPath {
    pub fn into(self) -> PathBuf {
        self.0
    }
}
impl fmt::Display for RepoPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.display(), f)
    }
}
impl<T: ?Sized + AsRef<OsStr>> From<&T> for RepoPath {
    fn from(s: &T) -> RepoPath {
        RepoPath(PathBuf::from(s.as_ref().to_os_string()))
    }
}

impl Default for RepoPath {
    fn default() -> Self {
        let mut home = dirs::home_dir().unwrap();
        home.push(".rust-fil-miner");
        RepoPath(home)
    }
}

#[derive(StructOpt, Debug)]
pub struct Command {
    /// Sets a custom logging filter.
    #[structopt(short, long)]
    pub testnet: bool,
    /// Sets a custom logging filter.
    #[structopt(short, long, value_name = "LOG_PATTERN")]
    pub log: Option<String>,
    #[structopt(long, default_value, parse(from_os_str))]
    pub repo_path: RepoPath,
    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "choose subcommand")]
pub enum SubCommand {
    Run(RunCommand),
    Init(InitCommand),
    /// Print storage miner info
    Info,
    /// Store random data in a sector
    PledgeSector,
    Sectors(SubSectorsCmd),
}

/// Start a storage miner process
#[derive(StructOpt, Debug)]
pub struct RunCommand {
    #[structopt(long, default_value = "127.0.0.1:2345")]
    pub api: String,
    /// don't check full_node sync status
    #[structopt(long)]
    pub no_sync: bool,
    /// enable use of GPU for mining operations
    #[structopt(long)]
    pub gpu_proving: bool,
}

/// Initialize a storage miner repo
#[derive(StructOpt, Debug)]
pub struct InitCommand {
    /// specify the address of an already created miner actor
    #[structopt(long)]
    pub actor: Option<String>,
    /// enable genesis mining (DON'T USE ON BOOTSTRAPPED NETWORK)
    #[structopt(long)]
    pub genesis_miner: bool,
    /// create separate worker key
    #[structopt(long)]
    pub create_worker_key: bool,
    /// worker key to use (overrides --create-worker-key)
    #[structopt(long, short)]
    pub worker: Option<String>,
    /// owner key to use
    #[structopt(long, short)]
    pub owner: String,
    /// specify sector size to use
    #[structopt(long)]
    pub sector_size: Option<usize>,
    /// specify set of presealed sectors for starting as a genesis miner
    #[structopt(long)]
    pub pre_sealed_sectors: Option<String>,
    /// specify the metadata file for the presealed sectors
    #[structopt(long)]
    pub pre_sealed_metadata: Option<String>,
    /// don't check full_node sync status
    #[structopt(long)]
    pub no_sync: bool,
    /// attempt to symlink to presealed sectors instead of copying them into place
    #[structopt(long)]
    pub symlink_imported_sectors: bool,
    /// Gateway of fetch params ("https://ipfs.io/ipfs/")
    #[structopt(
        long,
        default_value = "https://proof-parameters.s3.cn-south-1.jdcloud-oss.com/ipfs/"
    )]
    pub ipfs_gateway: String,
    /// Path of fetch params
    #[structopt(long, default_value = "/var/tmp/filecoin-proof-parameters/")]
    pub params_path: String,
}

#[derive(StructOpt, Debug)]
pub struct SubSectorsCmd {
    #[structopt(subcommand)]
    pub cmd: SectorsCmd,
}

/// Interact with sector store
#[derive(StructOpt, Debug)]
pub enum SectorsCmd {
    Status(SectorsStatusCmd),
    /// List sectors
    List,
    /// List References to sectors
    Refs,
    UpdateState(SectorsUpdateCmd),
}

/// Get the seal status of a sector by its ID
#[derive(StructOpt, Debug)]
pub struct SectorsStatusCmd {
    /// display event log
    #[structopt(long)]
    pub log: bool,
}

/// ADVANCED: manually update the state of a sector, this may aid in error recovery
#[derive(StructOpt, Debug)]
pub struct SectorsUpdateCmd {
    /// pass this flag if you know what you are doing
    #[structopt(long)]
    pub really_do_it: bool,
}
