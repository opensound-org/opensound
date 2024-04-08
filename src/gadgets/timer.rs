use crate::common::ostp;

pub async fn main() {
    ostp::emit::debug("Hello, world!", Some("你好，世界！"), "timer", "main", None);
}
