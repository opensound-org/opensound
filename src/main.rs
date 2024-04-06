use clap::{crate_version, Parser};
use opensound::ostp;

/// OpenSound PoC Binary
#[derive(Parser, Debug)]
#[command(version = concat!("v", crate_version!()), author, long_about = None)]
// TMD clap不支持中文，因此命令行帮助文档暂时就没办法i18n或者l10n了……
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
