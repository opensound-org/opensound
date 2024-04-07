//! Extension to signal handling for `tokio`
//!
//! 对`tokio`信号处理的扩展

/// Disable `ctrl-c` termination
///
/// 禁用掉`ctrl-c`终止
pub fn disable_ctrlc() {
    tokio::spawn(async {
        loop {
            tokio::signal::ctrl_c().await.ok();
        }
    });
}
