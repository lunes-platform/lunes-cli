use clap::{Args, Subcommand};
/// Commands to management your Lunes Wallet
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Wallet {
    #[clap(subcommand)]
    pub command: Option<WalletCommands>,
}

#[derive(Debug, Subcommand)]
pub enum WalletCommands {
    /// Disabled
    Rename(WalletRename),
    /// Disabled
    New(WalletNew),
    /// Disabled
    Add(WalletAdd),
    /// Disabled
    List,
    /// Disabled
    Del,
}

#[derive(Debug, Args)]
pub struct WalletNew {
    #[clap(short, long)]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct WalletRename {
    #[clap(short, long)]
    pub old_name: Option<String>,
    #[clap(short, long)]
    pub new_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct WalletAdd {
    #[clap(short, long)]
    pub private_key: Option<String>,
    #[clap(short, long)]
    pub seed: Option<String>,
    #[clap(short, long)]
    pub name: Option<String>,
}
