use clap::StructOpt;
use lunes::Lunes;

fn main() {
    let x = Lunes::parse();
    dbg!(x);
}
