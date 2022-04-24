use clap::StructOpt;
use lunes::{
    node::exec::{config, down, install, logs, status, up, version},
    node::NodeCommands,
    wallet::WalletCommands,
    Cli, Commands,
};

fn main() {
    use human_panic::setup_panic;
    setup_panic!();
    let args = Cli::parse();

    match args.command {
        Commands::Node(subcommand) => match subcommand.command.unwrap_or(NodeCommands::Version) {
            NodeCommands::Install(arg) => install(arg.version),
            NodeCommands::Version => version(),
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
            WalletCommands::New(arg) => println!("unavaliable {:?}", arg),
        },
        Commands::External(args) => {
            println!("{:?} Not a valid command", &args[0]);
        }
    }
}
