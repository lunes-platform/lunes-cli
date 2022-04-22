use clap::{Command, StructOpt, Subcommand};
use lunes_cli::node::NodeCommands;
use lunes_cli::wallet::{Wallet, WalletCommands};
use lunes_cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Node(subcommand) => match subcommand.command.unwrap_or(NodeCommands::Version) {
            NodeCommands::Version => println!("version: 1.1.1"),
            NodeCommands::Status => println!("status: ON"),
            NodeCommands::Up => println!("up lunes node"),
            NodeCommands::Down => println!("down lunes node"),
            NodeCommands::Logs => println!("Show logs of lunes node"),
            NodeCommands::Install(arg) => match arg.version {
                Some(v) => println!("Installing your lunes node version: {}", v),
                None => println!("Installing your lunes node version: latest"),
            },
        },
        Commands::Wallet(subcommand) => match subcommand.command.unwrap_or(WalletCommands::List) {
            WalletCommands::List => println!("w1, w2, w3"),
            WalletCommands::Add => println!("new wallet add"),
            WalletCommands::Del => println!("wallet del"),
            WalletCommands::New(_) => println!("unavaliable"),
        },
        Commands::External(args) => {
            println!("{:?} Not a valid command", &args[0]);
        }
    }
}
