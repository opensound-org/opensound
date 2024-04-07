//! Extension to `std::time` and `tokio::time`
//!
//! 对`std::time`和`tokio::time`的扩展
use std::time::Duration;

/// Sleep for minimum amount of time (which is 1ms)
///
/// 睡眠最短的时间（1ms）
///
/// This function is useful for those who want a minimal `yield`, but with `sleep`.
///
/// 这个函数对于想要一个最短的，但带有“睡眠”的`yield`很有用。
pub async fn sleep_minimal() {
    tokio::time::sleep(Duration::from_millis(1)).await
}
