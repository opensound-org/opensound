/// Some common components
///
/// 一些通用组件
pub mod common {
    /// OpenSound extensions to Standard library
    ///
    /// OpenSound对标准库做的一些扩展
    ///
    /// This module will be separated into an independent crate at some point in the future.
    ///
    /// 本模块会在未来的某些时刻分离至独立的crate中。
    pub mod ostd {
        pub mod signal;
        pub mod time;
    }
    pub mod ostp;

    /// Calculation of Gadget Token
    ///
    /// Gadget Token的计算
    pub mod token;

    /// Result<(), anyhow::Error>
    pub type CommonRes = Result<(), anyhow::Error>;

    use std::{future::Future, pin::Pin};

    /// Pin<Box<dyn Future<Output = CommonRes> + Send>>
    pub type CommonFut = Pin<Box<dyn Future<Output = CommonRes> + Send>>;
}
/// The `Gadget` concept
///
/// `Gadget` 概念
///
/// Detailed documentation is todo
///
/// 详细文档todo
pub mod gadgets {
    pub mod timer;
    pub mod uuid;
}

mod microkernel {
    mod init;
    pub use init::*;

    mod backends {
        #[cfg(feature = "actix-web")]
        pub mod actix_web;
        #[cfg(feature = "axum")]
        pub mod axum;
        #[cfg(feature = "ntex")]
        pub mod ntex;
        #[cfg(feature = "poem")]
        pub mod poem;
        #[cfg(feature = "rocket")]
        pub mod rocket;
        #[cfg(feature = "salvo")]
        pub mod salvo;
        #[cfg(feature = "viz")]
        pub mod viz;
        #[cfg(feature = "warp")]
        pub mod warp;
    }

    #[allow(dead_code)]
    mod reqres;
}

/// Version number constant prefixed with "v"
///
/// 带“v”前缀的版本号常量
pub const VERSION: &'static str = concat!("v", clap::crate_version!());

use clap::Args;
use common::{ostp, CommonRes};
use microkernel::MicroKernel;

/// Boot arguments
///
/// 启动参数
///
/// Can be put into your command line program and used as [clap](https://crates.io/crates/clap)'s [Args](https://docs.rs/clap/latest/clap/trait.Args.html).
///
/// 可以放到您的命令行程序中作为[clap](https://crates.io/crates/clap)的[Args](https://docs.rs/clap/latest/clap/trait.Args.html)使用。
#[derive(Args, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BootArgs {
    /// The port used by the HttpServer. If not specified, it defaults to a random port.
    ///
    /// HttpServer使用的端口，如果不指定，则默认使用随机端口。
    #[arg(long, long_help = None)]
    pub http_port: Option<u16>,
}

/// Main boot procedure entry
///
/// 主启动过程入口
pub async fn boot(args: BootArgs) -> CommonRes {
    ostp::install_default();
    MicroKernel::launch(args.http_port).await?.join().await
}
