use clap::StructOpt;
use lunes_cli::{node::NodeCommands, wallet::WalletCommands, Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Node(subcommand) => match subcommand.command.unwrap_or(NodeCommands::Version) {
            NodeCommands::Version => println!("version: 1.1.1"),
            NodeCommands::Config => println!("pass your config"),
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
            WalletCommands::Del => println!("wallet del"),
            WalletCommands::Rename(arg) => println!("Rename {:?}", arg),
            WalletCommands::Add(arg) => println!("new wallet add {:?}", arg),
            WalletCommands::New(arg) => println!("unavaliable {:?}", arg),
        },
        Commands::External(args) => {
            println!("{:?} Not a valid command", &args[0]);
        }
    }
}
