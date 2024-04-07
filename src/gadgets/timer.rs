use crate::common::ostp;

pub async fn main() {
    ostp::emit::debug("Hello, world!", "timer", None, true);
}
