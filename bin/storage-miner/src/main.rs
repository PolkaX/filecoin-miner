mod commands;

use structopt::StructOpt;

use commands::{Command, SubCommand};

fn main() {
    let opt = Command::from_args();
    match opt.cmd {
        SubCommand::Run(com) => {
            println!("{:?}", com);
        }
        _ => println!("{:?}", opt.cmd),
    }
}
