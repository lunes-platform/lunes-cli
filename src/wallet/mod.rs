/// Commands to management your Lunes Wallet
#[derive(clap::Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    new: Option<String>,
    from_seed: Option<String>,
    from_prvk: Option<String>,
}
