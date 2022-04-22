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
    New(WalletNew),
    List,
    Add,
    Del,
}

#[derive(Debug, Args)]
pub struct WalletNew {
    #[clap(short, long)]
    pub from_seed: Option<String>,
    pub from_private_key: Option<String>,
    pub nonce: Option<u32>,
    pub chain: Option<u8>,
}
