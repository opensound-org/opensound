use clap::{Parser, Subcommand};
use opensound::{
    boot,
    common::{ostd::signal::disable_ctrlc, CommonRes},
    gadgets::{timer, uuid},
    BootArgs, VERSION,
};

/// OpenSound PoC Binary
#[derive(Parser, Debug)]
#[command(version = VERSION, author, long_about = None)]
// Damn clap does not support languages other than English,
// so the command line help documentation cannot support i18n or l10n for the time being...
//
// TMD clap不支持英文以外的其它语言，因此命令行帮助文档暂时就没办法i18n或者l10n了……
enum Commands {
    /// Boot the API Server
    Boot(BootArgs),
    #[command(flatten)]
    Gadgets(Gadgets),
}

#[derive(Subcommand, Debug)]
enum Gadgets {
    #[command(hide = true)]
    Timer,
    #[command(hide = true)]
    Uuid,
}

#[tokio::main]
async fn main() -> CommonRes {
    disable_ctrlc();

    match Commands::parse() {
        Commands::Boot(args) => boot(args).await,
        Commands::Gadgets(cmd) => match cmd {
            Gadgets::Timer => timer::main().await,
            Gadgets::Uuid => uuid::main().await,
        },
    }
}
