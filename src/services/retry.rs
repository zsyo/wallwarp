// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 通用重试封装
//!
//! 固定间隔重试，共尝试 `max_retries + 1` 次。
//! 错误类型只需实现 `Display`（`String` 与 `Box<dyn Error>` 均满足）。

use tracing::{error, info, warn};

/// 执行带重试的异步操作
///
/// # 参数
/// - `identifier`: 请求标识符（用于日志，如 "URL:https://..." 或任务 ID）
/// - `operation_name`: 操作名称（用于日志）
/// - `max_retries`: 最大重试次数（首次之外再重试的次数）
/// - `operation`: 返回未来对象的闭包，每次尝试调用一次
pub async fn retry_with_backoff<F, T, E, Fut>(
    identifier: &str,
    operation_name: &str,
    max_retries: usize,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error: Option<E> = None;

    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => {
                if attempt > 0 {
                    info!("[{}] [{}] 重试第 {} 次成功", operation_name, identifier, attempt);
                }
                return Ok(result);
            }
            Err(e) => {
                if attempt < max_retries {
                    warn!(
                        "[{}] [{}] 第 {} 次尝试失败，将在1秒后重试: {}",
                        operation_name, identifier, attempt + 1, e
                    );
                    last_error = Some(e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                } else {
                    error!(
                        "[{}] [{}] 所有重试失败，共尝试 {} 次: {}",
                        operation_name,
                        identifier,
                        max_retries + 1,
                        e
                    );
                    last_error = Some(e);
                }
            }
        }
    }

    Err(last_error.expect("至少执行过一次尝试"))
}
