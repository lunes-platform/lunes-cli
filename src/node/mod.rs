/// Commands to management your Lunes Node
#[derive(clap::Args, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Commands {
    install: Option<String>,
    up: Option<String>,
    down: Option<String>,
    logs: Option<String>,
    status: Option<String>,
    config: Option<String>,
}
