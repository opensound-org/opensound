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

    /// Result<(), Box<dyn std::error::Error + Send + Sync>>
    pub type CommonRes = Result<(), Box<dyn std::error::Error + Send + Sync>>;
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
/// The OpenSound Kernel
///
/// OpenSound内核
pub mod kernel;

/// Version number constant prefixed with "v"
///
/// 带“v”前缀的版本号常量
pub const VERSION: &'static str = concat!("v", clap::crate_version!());

use common::{ostp, CommonRes};

/// Default boot procedure entry
///
/// 默认启动过程入口
pub async fn boot() -> CommonRes {
    ostp::install_default();
    kernel::main().await
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
