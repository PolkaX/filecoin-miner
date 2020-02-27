mod commands;
mod service;

use structopt::StructOpt;

use commands::{Command, SubCommand};

fn main() {
    let opt = Command::from_args();
    match opt.cmd {
        SubCommand::Run(com) => {
            println!("{:?}", com);

            node_service::run_service_until_exit(service::ServiceBuilder::new().build());
        }
        SubCommand::Init(init) => {
            println!("{:?}", init);
        }
        _ => println!("{:?}", opt.cmd),
    }
}
