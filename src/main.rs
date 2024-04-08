use clap::Parser;
use opensound::{
    boot,
    common::{ostd::signal::disable_ctrlc, CommonRes},
    gadgets::{timer, uuid},
    VERSION,
};

/// OpenSound PoC Binary
#[derive(Parser, Debug)]
#[command(version = VERSION, author, long_about = None)]
// TMD clap不支持中文，因此命令行帮助文档暂时就没办法i18n或者l10n了……
enum Commands {
    /// Boot the API Server
    Boot,
    #[command(hide = true)]
    Timer,
    #[command(hide = true)]
    Uuid,
}

#[tokio::main]
async fn main() -> CommonRes {
    disable_ctrlc();

    match Commands::parse() {
        Commands::Boot => boot().await,
        Commands::Timer => timer::main().await,
        Commands::Uuid => uuid::main().await,
    }
}
