// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 代理客户端创建模块
//!
//! 所有 HTTP 请求路径共用同一套客户端配置（[`build_client`] 单一构建入口），
//! 代理来源优先级：配置文件代理 > 环境变量代理（回退）> 直连。
//!
//! 注意：不使用 `http2_prior_knowledge`（跳过 ALPN 协商对不支持 HTTP/2 的
//! 图床与代理会导致全部请求失败），依赖 TLS ALPN 自动协商协议版本。

use std::time::Duration;
use tracing::{debug, error, info, warn};

/// 连接超时
const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
/// 请求总超时
const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);
/// 空闲连接保活时长
const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);

/// 统一的 HTTP 客户端构建入口
///
/// `proxy_url` 为 Some 时挂载代理；配置（超时/连接池/TCP_NODELAY/压缩）全路径一致
fn build_client(proxy_url: Option<&str>) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let mut builder = reqwest::Client::builder()
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(POOL_IDLE_TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .tcp_nodelay(true)
        // 启用gzip压缩（reqwest默认支持）
        .gzip(true)
        // 启用brotli压缩（需要features支持）
        .brotli(true);
    if let Some(proxy_url) = proxy_url {
        debug!("[代理客户端] 尝试创建代理客户端，代理URL: {}", proxy_url);
        builder = builder.proxy(reqwest::Proxy::all(proxy_url)?);
    }
    Ok(builder.build()?)
}

/// 从环境变量中检测代理配置
///
/// # 返回
/// 返回检测到的代理 URL（如果有）
pub fn get_proxy_from_env() -> Option<String> {
    // 优先检查 HTTPS_PROXY（用于 HTTPS 请求）
    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY")
        && !https_proxy.is_empty()
    {
        return Some(https_proxy);
    }

    // 其次检查 HTTP_PROXY（用于 HTTP 请求）
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY")
        && !http_proxy.is_empty()
    {
        return Some(http_proxy);
    }

    // 最后检查 ALL_PROXY（通用代理）
    if let Ok(all_proxy) = std::env::var("ALL_PROXY")
        && !all_proxy.is_empty()
    {
        return Some(all_proxy);
    }

    None
}

/// 创建带代理的 HTTP 客户端
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL（可选）
/// - `proxy_enabled`: 代理是否启用
/// - `use_env_fallback`: 是否使用环境变量作为回退
///
/// # 代理优先级
/// 1. 配置文件代理（proxy_enabled=true 且 proxy 非空）
/// 2. 环境变量代理（use_env_fallback=true 且配置文件代理未设置）
/// 3. 无代理
pub fn create_proxy_client(
    proxy: Option<String>,
    proxy_enabled: bool,
    use_env_fallback: bool,
) -> reqwest::Client {
    // 优先级1: 使用配置文件代理
    if proxy_enabled
        && let Some(proxy_url) = proxy
        && !proxy_url.is_empty()
    {
        info!("[代理客户端] 使用配置文件代理: {}", proxy_url);
        return build_client(Some(&proxy_url)).unwrap_or_else(|e| {
            warn!("[代理客户端] 代理客户端创建失败: {}，回退到直连", e);
            reqwest::Client::new()
        });
    }

    // 优先级2: 使用环境变量代理（如果启用回退）
    if use_env_fallback
        && let Some(env_proxy_url) = get_proxy_from_env()
    {
        info!("[代理客户端] 使用环境变量代理: {}", env_proxy_url);
        return build_client(Some(&env_proxy_url)).unwrap_or_else(|e| {
            warn!("[代理客户端] 环境变量代理客户端创建失败: {}，回退到直连", e);
            reqwest::Client::new()
        });
    }

    // 优先级3: 无代理
    debug!("[代理客户端] 使用直连客户端");
    build_client(None).unwrap_or_else(|e| {
        error!("[代理客户端] 直连客户端创建失败: {}", e);
        reqwest::Client::new()
    })
}

/// 创建带代理和环境变量回退的 HTTP 客户端（下载路径通用版本）
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL（可选）
/// - `url`: 请求 URL（用于日志）
/// - `log_prefix`: 日志前缀（例如："[缩略图缓存]" 或 "[下载任务]"）
/// - `log_level_info`: 是否使用 info 级别（否则使用 debug 级别）
pub fn create_client_with_env_fallback(
    proxy: Option<String>,
    url: &str,
    log_prefix: &str,
    log_level_info: bool,
) -> reqwest::Client {
    // 尝试使用配置文件代理
    if let Some(proxy_url) = proxy
        && !proxy_url.is_empty()
    {
        if log_level_info {
            info!(
                "[{}] [URL:{}] 使用配置文件代理: {}",
                log_prefix, url, proxy_url
            );
        } else {
            debug!(
                "[{}] [URL:{}] 使用配置文件代理: {}",
                log_prefix, url, proxy_url
            );
        }
        return build_client(Some(&proxy_url)).unwrap_or_else(|e| {
            warn!(
                "[{}] [URL:{}] 代理客户端创建失败: {}，回退到直连",
                log_prefix, url, e
            );
            reqwest::Client::new()
        });
    }

    // 尝试使用环境变量代理
    if let Some(env_proxy_url) = get_proxy_from_env() {
        if log_level_info {
            info!(
                "[{}] [URL:{}] 使用环境变量代理: {}",
                log_prefix, url, env_proxy_url
            );
        } else {
            debug!(
                "[{}] [URL:{}] 使用环境变量代理: {}",
                log_prefix, url, env_proxy_url
            );
        }
        return build_client(Some(&env_proxy_url)).unwrap_or_else(|e| {
            warn!(
                "[{}] [URL:{}] 环境变量代理客户端创建失败: {}，回退到直连",
                log_prefix, url, e
            );
            reqwest::Client::new()
        });
    }

    // 回退到直连
    debug!("[{}] [URL:{}] 使用直连客户端", log_prefix, url);
    build_client(None).unwrap_or_else(|e| {
        error!("[{}] [URL:{}] 直连客户端创建失败: {}", log_prefix, url, e);
        reqwest::Client::new()
    })
}

/// 检测环境变量代理配置
///
/// # 返回
/// 返回检测到的环境变量代理信息
pub fn detect_env_proxy() -> Option<String> {
    let mut proxy_info = Vec::new();

    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY")
        && !https_proxy.is_empty()
    {
        proxy_info.push(format!("HTTPS_PROXY={}", https_proxy));
    }
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY")
        && !http_proxy.is_empty()
    {
        proxy_info.push(format!("HTTP_PROXY={}", http_proxy));
    }
    if let Ok(all_proxy) = std::env::var("ALL_PROXY")
        && !all_proxy.is_empty()
    {
        proxy_info.push(format!("ALL_PROXY={}", all_proxy));
    }
    if let Ok(no_proxy) = std::env::var("NO_PROXY")
        && !no_proxy.is_empty()
    {
        proxy_info.push(format!("NO_PROXY={}", no_proxy));
    }

    if proxy_info.is_empty() {
        None
    } else {
        Some(format!("检测到环境变量代理: {}", proxy_info.join(", ")))
    }
}
