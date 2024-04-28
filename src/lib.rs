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
        cfg_if::cfg_if! {
            if #[cfg(not(any(
                feature = "actix-web",
                feature = "axum",
                feature = "ntex",
                feature = "poem",
                feature = "rocket",
                feature = "salvo",
                feature = "viz",
                feature = "warp",
            )))] {
                compile_error!("One and only one of the following features can be enabled: \"actix-web\", \"axum\", \"ntex\", \"poem\", \"rocket\", \"salvo\", \"viz\", \"warp\".");
                compile_error!("You haven't enabled any of them.");
            }
            else if #[cfg(all(
                feature = "actix-web",
                not(feature = "axum"),
                not(feature = "ntex"),
                not(feature = "poem"),
                not(feature = "rocket"),
                not(feature = "salvo"),
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod actix_web;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                feature = "axum",
                not(feature = "ntex"),
                not(feature = "poem"),
                not(feature = "rocket"),
                not(feature = "salvo"),
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod axum;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                feature = "ntex",
                not(feature = "poem"),
                not(feature = "rocket"),
                not(feature = "salvo"),
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod ntex;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                not(feature = "ntex"),
                feature = "poem",
                not(feature = "rocket"),
                not(feature = "salvo"),
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod poem;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                not(feature = "ntex"),
                not(feature = "poem"),
                feature = "rocket",
                not(feature = "salvo"),
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod rocket;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                not(feature = "ntex"),
                not(feature = "poem"),
                not(feature = "rocket"),
                feature = "salvo",
                not(feature = "viz"),
                not(feature = "warp"),
            ))] {
                pub mod salvo;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                not(feature = "ntex"),
                not(feature = "poem"),
                not(feature = "rocket"),
                not(feature = "salvo"),
                feature = "viz",
                not(feature = "warp"),
            ))] {
                pub mod viz;
            }
            else if #[cfg(all(
                not(feature = "actix-web"),
                not(feature = "axum"),
                not(feature = "ntex"),
                not(feature = "poem"),
                not(feature = "rocket"),
                not(feature = "salvo"),
                not(feature = "viz"),
                feature = "warp",
            ))] {
                pub mod warp;
            } else {
                compile_error!("One and only one of the following features can be enabled: \"actix-web\", \"axum\", \"ntex\", \"poem\", \"rocket\", \"salvo\", \"viz\", \"warp\".");
                compile_error!("You have multiple of them enabled.");
            }
        }
    }

    #[allow(dead_code)]
    mod reqres;
}

/// Version number constant prefixed with "v"
///
/// 带“v”前缀的版本号常量
pub const VERSION: &'static str = concat!("v", clap::crate_version!());

use common::{ostp, CommonRes};
use microkernel::MicroKernel;

/// Default boot procedure entry
///
/// 默认启动过程入口
pub async fn boot_default() -> CommonRes {
    ostp::install_default();
    MicroKernel::launch(None).await?.join().await
}
