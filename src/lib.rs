/// Some common components
/// 
/// 一些通用组件
pub mod common {
    pub mod ostp;
}
/// The `Gadget` concept
/// 
/// `Gadget` 概念
/// 
/// Detailed documentation is todo
/// 
/// 详细文档todo
pub mod gadgets {
    pub mod timer;
    pub mod uuid;
}
/// The OpenSound Kernel
/// 
/// OpenSound内核
pub mod kernel;

/// Default boot procedure entry
///
/// 默认启动过程入口
pub async fn boot() {
    kernel::main().await;
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
