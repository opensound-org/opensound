pub mod common {
    pub mod ostp;
}
pub mod gadgets {
    pub mod timer;
}
pub mod kernel;

pub fn boot() {
    kernel::main();
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
