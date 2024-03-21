use opensound::ostp;

fn main() {
    ostp::install_default();
    ostp::emit::debug("Hello, world!", "main", None, true);

    println!("Hello, world!");
}
