pub mod exec;
use clap::{Args, Subcommand};

/// Commands to management your Lunes Node
#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Node {
    #[clap(subcommand)]
    pub command: Option<NodeCommands>,
}

#[derive(Debug, Subcommand)]
pub enum NodeCommands {
    /// Install Lunes Node by version
    Install(NodeInstall),
    /// Version of your Lunes Node
    Version,
    /// Edit config of your Lunes Node
    Config,
    /// Status of your Lunes Node
    Status,
    /// Shutdown your Lunes Node
    Down,
    /// Follow your Lunes Node logs
    Logs,
    /// Turn On your Lunes Node
    Up,
}

#[derive(Debug, Args)]
pub struct NodeInstall {
    #[clap(short, long)]
    pub version: Option<String>,
}
