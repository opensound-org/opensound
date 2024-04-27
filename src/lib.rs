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

    #[allow(dead_code)]
    mod reqres;
}

/// Version number constant prefixed with "v"
///
/// 带“v”前缀的版本号常量
pub const VERSION: &'static str = concat!("v", clap::crate_version!());

use common::{ostp, CommonRes};

/// Default boot procedure entry
///
/// 默认启动过程入口
pub async fn boot_default() -> CommonRes {
    ostp::install_default();
    microkernel::launch().await
}
