use crate::common::ostp;

pub fn main() {
    ostp::emit::debug("Hello, world!", "kernel", None, true);
}
