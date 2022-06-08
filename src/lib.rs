pub mod node;
pub mod wallet;

use clap::{Parser, Subcommand};
use node::Node;
use std::ffi::OsString;
use wallet::Wallet;

/// lunes cli management for full-node and wallet
#[derive(Parser, Debug)]
#[clap(name = "lunes")]
#[clap(bin_name = "lunes")]
#[clap(about = "🕹 Lunes CLI management for full-node and wallet", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Node(Node),
    Wallet(Wallet),
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}
