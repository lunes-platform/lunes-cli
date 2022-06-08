pub mod exec;

use clap::{Args, Subcommand};

/// 🔑 Management your Lunes Wallet
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
    /// 🎉 Create new Wallet
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
    /// Name of wallet
    #[clap(long)]
    pub name: String,
    /// 1 for mainnet, 0 for testnet
    #[clap(short, long)]
    pub chain: Option<u8>,
    /// Nonce for create your seed
    #[clap(short, long)]
    pub nonce: Option<u32>,
    /// Number of Words for create your seed
    #[clap(short, long)]
    pub words: Option<u8>
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
