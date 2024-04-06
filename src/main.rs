use clap::{crate_version, Parser};
use opensound::ostp;

/// OpenSound PoC Binary
#[derive(Parser, Debug)]
#[command(version = concat!("v", crate_version!()), author, long_about = None)]
enum Commands {
    /// Boot the API Server
    Boot,
    #[command(hide = true)]
    Timer,
}

fn main() {
    println!("{:?}", Commands::parse());

    ostp::install_default();
    ostp::emit::debug("Hello, world!", "main", None, true);

    println!("Hello, world!");
}
