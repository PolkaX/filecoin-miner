use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "choose subcommand")]
pub enum SubCommand {
    Run(RunCommand),
    Init(InitCommand),
    #[structopt(about = "Print storage miner info")]
    Info,
    #[structopt(about = "Store random data in a sector")]
    PledgeSector,
    Sectors {},
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
    pub actor: String,
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
    // todo default sector_size
    /// specify sector size to use
    #[structopt(long)]
    pub sector_size: usize,
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
}

/// Interact with sector store
#[derive(StructOpt, Debug)]
pub enum SectorsCommand {
    Status(SectorsStatusCmd),
    #[structopt(about = "List sectors")]
    List,
    #[structopt(about = "List References to sectors")]
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
