use crate::common::{ostp, CommonRes};

/// Kernel entry
///
/// 内核入口
pub async fn main() -> CommonRes {
    ostp::emit::debug(
        "Hello, world!",
        Some("你好，世界！"),
        "kernel",
        "main",
        None,
    );
    Ok(())
}
