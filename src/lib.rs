mod node;
mod wallet;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "lunes")]
#[clap(bin_name = "lunes")]
pub enum Lunes {
    Node(node::Commands),
    Wallet(wallet::Commands),
}
