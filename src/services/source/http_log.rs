// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 图源 HTTP 请求的公共错误分类与日志收口
//!
//! 各图源 client 的网络失败/状态码异常日志在此统一颗粒度，与最早接入的
//! Wallhaven client 保持一致：失败按超时/连接失败/其它分类，中间网络错误
//! 记 warn，最终失败（状态码异常）记 error。默认 info 档位下失败原因即可见，
//! 避免"刷新无数据但日志无线索"。

use tracing::{error, warn};

/// send 阶段失败的分类与日志
///
/// # 参数
/// - `source`: 图源名（日志前缀，如 Pixabay）
/// - `identifier`: 请求标识（如 page1）
/// - `error`: reqwest 传输层错误
/// - `timeout_secs`: 本次请求设置的超时秒数（超时日志展示用）
///
/// # 返回
/// 返回面向用户的错误描述，同时按分类落 warn 日志
pub fn log_send_error(
    source: &str,
    identifier: &str,
    error: &reqwest::Error,
    timeout_secs: Option<u64>,
) -> String {
    if error.is_timeout() {
        warn!(
            "[{} API] [{}] 请求超时（{}秒）",
            source,
            identifier,
            timeout_secs.unwrap_or(0)
        );
        "请求超时，请检查网络连接或设置代理".to_string()
    } else if error.is_connect() {
        warn!("[{} API] [{}] 连接失败: {}", source, identifier, error);
        "连接失败，请检查网络连接或设置代理".to_string()
    } else {
        warn!("[{} API] [{}] 请求失败: {}", source, identifier, error);
        format!("请求失败: {}", error)
    }
}

/// 响应状态码非 2xx 的日志收口
///
/// # 参数
/// - `source`: 图源名（日志前缀）
/// - `identifier`: 请求标识
/// - `status`: HTTP 状态码（如限流 429、鉴权失败 401）
///
/// # 返回
/// 返回面向用户的错误描述，同时落 error 日志
pub fn log_api_error_status(source: &str, identifier: &str, status: reqwest::StatusCode) -> String {
    let message = format!("API返回错误: {}", status);
    error!("[{} API] [{}] {}", source, identifier, message);
    message
}

/// 响应体摘要（trace 档排障用）
///
/// 超长正文截断到 `max_len` 个字符并附省略标记，按字符边界截取避免
/// UTF-8 边界问题；换行制表折叠为空格，保持日志一行一条
pub fn body_excerpt(body: &str, max_len: usize) -> String {
    let mut excerpt: String = body.chars().take(max_len).collect();
    if body.chars().count() > max_len {
        excerpt.push_str("...(已截断)");
    }
    excerpt.replace(['\n', '\r', '\t'], " ")
}
