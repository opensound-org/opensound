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
    }

    #[allow(dead_code)]
    mod reqres;
}

/// Version number constant prefixed with "v"
///
/// 带“v”前缀的版本号常量
pub const VERSION: &'static str = concat!("v", clap::crate_version!());

use clap::Args;
use common::{
    ostd::{signal::disable_ctrlc, time::sleep_infinite},
    ostp, CommonRes,
};
use microkernel::MicroKernel;
use tokio::signal::ctrl_c;

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

    /// Disable Ctrl-C termination. Ctrl-C is enabled by default.
    ///
    /// 禁用Ctrl-C终止。Ctrl-C默认是启用的。
    #[arg(short, long, long_help = None, default_value_t = false)]
    pub disable_ctrlc: bool,
}

impl BootArgs {
    /// Whether ctrlc termination is enabled.
    ///
    /// ctrlc终止是否已启用。
    ///
    /// This is a simple shortcut for `!self.disable_ctrlc`.
    ///
    /// 这就是`!self.disable_ctrlc`的一个简单的捷径。
    pub fn ctrlc_enabled(&self) -> bool {
        !self.disable_ctrlc
    }
}

/// Main boot procedure entry
///
/// 主启动过程入口
pub async fn boot(args: BootArgs) -> CommonRes {
    if args.disable_ctrlc {
        disable_ctrlc();
    }

    ostp::install_default();

    loop {
        let mut kernel = MicroKernel::launch(args.http_port).await?;

        if args.ctrlc_enabled() || kernel.sys_ctrl_enabled() {
            let should_reboot = tokio::select! {
                _ = ctrl_c(), if args.ctrlc_enabled() => {
                    ostp::emit::warn(
                        "Ctrl-C event received",
                        Some("收到ctrl-c事件"),
                        "Boot",
                        "Shutdown",
                        None,
                    );
                    false
                }
                evt = kernel.sys_event(), if kernel.sys_ctrl_enabled() => {
                    let should_reboot = evt.should_reboot();
                    let message_en = format!("{} event received", if should_reboot { "Reboot" } else { "Shutdown" });
                    let message_zh = Some(format!("收到{}事件", if should_reboot { "重启" } else { "关闭" }));
                    let message_zh = message_zh.as_deref();
                    ostp::emit::warn(&message_en, message_zh, "Boot", "Shutdown", None);
                    should_reboot
                }
            };

            ostp::emit::warn(
                "Begin the graceful shutdown process",
                Some("开始执行优雅停机流程"),
                "Boot",
                "Shutdown",
                None,
            );

            kernel.graceful_shutdown().await?;

            if !should_reboot {
                break;
            }

            ostp::emit::warn("Rebooting...", Some("正在重启……"), "Boot", "Shutdown", None);
        } else {
            sleep_infinite().await;
            break;
        }
    }

    Ok(())
}
