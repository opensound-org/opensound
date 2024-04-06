use crate::common::ostp;

/// Kernel entry
///
/// 内核入口
pub fn main() {
    ostp::emit::debug("Hello, world!", "kernel", None, true);
}
