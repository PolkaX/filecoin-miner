use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: SubCommand,
    #[structopt(long, default_value = "~/.lotusworker")]
    pub repo: String,
    #[structopt(long, default_value = "~/.lotusstorage")]
    pub storagerepo: String,
    /// Gateway of fetch params ("https://ipfs.io/ipfs/")
    #[structopt(
        long,
        default_value = "https://proof-parameters.s3.cn-south-1.jdcloud-oss.com/ipfs/"
    )]
    pub ipfs_gateway: String,
    #[structopt(short, long, value_name = "LOG_PATTERN")]
    pub log: Option<String>,
    /// Path of fetch params
    #[structopt(long, default_value = "/var/tmp/filecoin-proof-parameters/")]
    pub params_path: String,
    /// enable use of GPU for mining operations
    #[structopt(long)]
    pub enable_gpu_proving: bool,
    #[structopt(long)]
    pub no_precommit: bool,
    #[structopt(long)]
    pub no_commit: bool,
    /// Prints the version
    #[structopt(long)]
    pub version: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "choose subcommand")]
pub enum SubCommand {
    Run(RunCommand),
}

/// Start seal worker
#[derive(StructOpt, Debug)]
pub struct RunCommand {}
