mod commands;
mod service;

use structopt::StructOpt;

use commands::{Command, SubCommand};

fn main() {
    let opt = Command::from_args();
    match opt.cmd {
        SubCommand::Run(com) => {
            println!("{:?}", com);

            let (signal, exit) = exit_future::signal();

            node_service::run_service_until_exit(service::Mock {
                exit,
                signal: Some(signal),
            });
        }
        SubCommand::Init(init) => {
            println!("{:?}", init);
        }
        _ => println!("{:?}", opt.cmd),
    }
}
