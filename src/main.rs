use clap::StructOpt;
use lunes::{
    node::exec::{config, down, install, logs, restart, status, up, version},
    node::NodeCommands
};
use lunes::{
    wallet::exec::create_wallet,
    wallet::WalletCommands
};
use lunes::{Cli, Commands};

#[tokio::main]
async fn main() {
    use human_panic::setup_panic;
    setup_panic!();
    let args = Cli::parse();

    match args.command {
        Commands::Node(subcommand) => match subcommand.command.unwrap_or(NodeCommands::Version) {
            NodeCommands::Install(args) => install(args).await,
            NodeCommands::Version => version(),
            NodeCommands::Restart => restart(),
            NodeCommands::Config => config(),
            NodeCommands::Status => status(),
            NodeCommands::Down => down(),
            NodeCommands::Logs => logs(),
            NodeCommands::Up => up(),
        },
        Commands::Wallet(subcommand) => match subcommand.command.unwrap_or(WalletCommands::List) {
            WalletCommands::List => println!("w1, w2, w3"),
            WalletCommands::Del => println!("wallet del"),
            WalletCommands::Rename(arg) => println!("Rename {:?}", arg),
            WalletCommands::Add(arg) => println!("new wallet add {:?}", arg),
            WalletCommands::New(args) => create_wallet(args)
        },
        Commands::External(args) => {
            println!("{:?} Not a valid command", &args[0]);
        }
    }
}
