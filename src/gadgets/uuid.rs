use crate::common::{ostp, CommonRes};

pub async fn main() -> CommonRes {
    ostp::emit::debug("Hello, world!", Some("你好，世界！"), "uuid", "main", None);
    Ok(())
}
