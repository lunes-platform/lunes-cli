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
    Version,
    Status,
    Up,
    Down,
    Logs,
    Install(NodeInstall),
}

#[derive(Debug, Args)]
pub struct NodeInstall {
    #[clap(short, long)]
    pub version: Option<String>,
}
