use crate::common::ostp;

pub async fn main() {
    ostp::emit::debug("Hello, world!", "uuid", None, true);
}
