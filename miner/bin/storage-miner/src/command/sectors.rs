use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Sectors {
    /// List sectors
    List,
    /// List References to sectors
    Refs,
    /// Get the seal status of a sector by its ID
    Status {
        /// display event log
        #[structopt(long)]
        log: bool,
    },
    /// Pass this flag if you know what you are doing
    UpdateState {
        /// ADVANCED: manually update the state of a sector, this may aid in error recovery
        #[structopt(long)]
        really_do_it: bool,
    },
    /// Store random data in a sector
    PledgeSector,
}

impl Sectors {
    pub fn run(&self) -> Result<()> {
        todo!("Implement sectors subcommand");
    }
}
