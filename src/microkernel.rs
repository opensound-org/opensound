use crate::common::{ostp, CommonRes};

/// MicroKernel entry
///
/// 微内核入口
pub async fn main() -> CommonRes {
    ostp::emit::debug(
        "Hello, world!",
        Some("你好，世界！"),
        "microkernel",
        "main",
        None,
    );
    Ok(())
}
