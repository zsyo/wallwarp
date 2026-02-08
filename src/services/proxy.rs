// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 代理客户端创建模块
//!
//! 提供统一的 HTTP 客户端创建功能，支持：
//! - 配置文件代理（优先级最高）
//! - 环境变量代理（回退选项）
//! - 无代理（最终回退）

use tracing::{debug, info, warn};

/// 从环境变量中检测代理配置
///
/// # 返回
/// 返回检测到的代理 URL（如果有）
pub fn get_proxy_from_env() -> Option<String> {
    // 优先检查 HTTPS_PROXY（用于 HTTPS 请求）
    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY") {
        if !https_proxy.is_empty() {
            return Some(https_proxy);
        }
    }

    // 其次检查 HTTP_PROXY（用于 HTTP 请求）
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY") {
        if !http_proxy.is_empty() {
            return Some(http_proxy);
        }
    }

    // 最后检查 ALL_PROXY（通用代理）
    if let Ok(all_proxy) = std::env::var("ALL_PROXY") {
        if !all_proxy.is_empty() {
            return Some(all_proxy);
        }
    }

    None
}

/// 代理配置选项
#[derive(Debug, Clone)]
pub enum ProxyConfig {
    /// 使用配置文件中的代理
    Config(String),
    /// 使用环境变量代理
    Environment,
    /// 不使用代理
    None,
}

/// 创建带代理的 HTTP 客户端
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL（可选）
/// - `proxy_enabled`: 代理是否启用
/// - `use_env_fallback`: 是否使用环境变量作为回退
///
/// # 返回
/// 返回配置好的 HTTP 客户端
///
/// # 代理优先级
/// 1. 配置文件代理（proxy_enabled=true 且 proxy 非空）
/// 2. 环境变量代理（use_env_fallback=true 且配置文件代理未设置）
/// 3. 无代理
pub fn create_proxy_client(proxy: Option<String>, proxy_enabled: bool, use_env_fallback: bool) -> reqwest::Client {
    // 优先级1: 使用配置文件代理
    if proxy_enabled {
        if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                info!("[代理客户端] 使用配置文件代理: {}", proxy_url);
                match create_client_with_proxy(&proxy_url) {
                    Ok(client) => return client,
                    Err(e) => {
                        warn!("[代理客户端] 配置文件代理创建失败: {}，尝试环境变量代理", e);
                        // 继续尝试环境变量代理
                    }
                }
            }
        }
    }

    // 优先级2: 使用环境变量代理（如果启用回退）
    if use_env_fallback {
        if let Some(env_proxy_url) = get_proxy_from_env() {
            info!("[代理客户端] 使用环境变量代理: {}", env_proxy_url);
            match create_client_with_proxy(&env_proxy_url) {
                Ok(client) => {
                    info!("[代理客户端] 环境变量代理客户端创建成功");
                    return client;
                }
                Err(e) => {
                    warn!("[代理客户端] 环境变量代理客户端创建失败: {}，回退到无代理", e);
                }
            }
        } else {
            debug!("[代理客户端] 未检测到环境变量代理");
        }
    }

    // 优先级3: 无代理
    info!("[代理客户端] 使用无代理客户端");
    reqwest::Client::new()
}

/// 创建带代理的优化 HTTP 客户端
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL（可选）
/// - `proxy_enabled`: 代理是否启用
/// - `use_env_fallback`: 是否使用环境变量作为回退
///
/// # 返回
/// 返回配置好的优化 HTTP 客户端
///
/// # 优化配置
/// - 连接池：最大100个连接，每个主机最多10个连接
/// - 超时：连接30秒，总超时300秒
/// - TCP：启用 TCP_NODELAY 减少延迟
/// - HTTP/2：启用 HTTP/2 协议
/// - 压缩：启用 gzip 和 brotli 压缩
pub fn create_optimized_proxy_client(
    proxy: Option<String>,
    proxy_enabled: bool,
    use_env_fallback: bool,
) -> reqwest::Client {
    // 优先级1: 使用配置文件代理
    if proxy_enabled {
        if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                info!("[代理客户端] 使用配置文件代理（优化）: {}", proxy_url);
                match create_optimized_client_with_proxy(&proxy_url) {
                    Ok(client) => return client,
                    Err(e) => {
                        warn!("[代理客户端] 配置文件代理创建失败: {}，尝试环境变量代理", e);
                        // 继续尝试环境变量代理
                    }
                }
            }
        }
    }

    // 优先级2: 使用环境变量代理（如果启用回退）
    if use_env_fallback {
        if let Some(env_proxy_url) = get_proxy_from_env() {
            info!("[代理客户端] 使用环境变量代理（优化）: {}", env_proxy_url);
            match create_optimized_client_with_proxy(&env_proxy_url) {
                Ok(client) => {
                    info!("[代理客户端] 环境变量代理客户端创建成功（优化）");
                    return client;
                }
                Err(e) => {
                    warn!("[代理客户端] 环境变量代理客户端创建失败: {}，回退到无代理", e);
                }
            }
        } else {
            debug!("[代理客户端] 未检测到环境变量代理");
        }
    }

    // 优先级3: 无代理（优化版）
    info!("[代理客户端] 使用无代理客户端（优化）");
    create_optimized_client()
}

/// 使用指定代理 URL 创建 HTTP 客户端
fn create_client_with_proxy(proxy_url: &str) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    debug!("[代理客户端] 尝试创建代理客户端，代理URL: {}", proxy_url);

    let proxy = reqwest::Proxy::all(proxy_url)?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;

    debug!("[代理客户端] 代理客户端创建成功");
    Ok(client)
}

/// 使用指定代理 URL 创建优化的 HTTP 客户端
pub fn create_optimized_client_with_proxy(proxy_url: &str) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    debug!("[代理客户端] 尝试创建优化代理客户端，代理URL: {}", proxy_url);

    let proxy = reqwest::Proxy::all(proxy_url)?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        // 连接池配置：最大100个连接，每个主机最多10个连接
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        // 超时配置
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(300))
        // TCP配置：启用TCP_NODELAY减少延迟
        .tcp_nodelay(true)
        // 启用HTTP/2
        .http2_prior_knowledge()
        // 启用gzip压缩（reqwest默认支持）
        .gzip(true)
        // 启用brotli压缩（需要features支持）
        .brotli(true)
        .build()?;

    debug!("[代理客户端] 优化代理客户端创建成功");
    Ok(client)
}

/// 创建无代理的优化 HTTP 客户端
pub fn create_optimized_client() -> reqwest::Client {
    reqwest::Client::builder()
        // 连接池配置：最大100个连接，每个主机最多10个连接
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        // 超时配置
        .connect_timeout(std::time::Duration::from_secs(30))
        .timeout(std::time::Duration::from_secs(300))
        // TCP配置：启用TCP_NODELAY减少延迟
        .tcp_nodelay(true)
        // 启用HTTP/2
        .http2_prior_knowledge()
        // 启用gzip压缩（reqwest默认支持）
        .gzip(true)
        // 启用brotli压缩（需要features支持）
        .brotli(true)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

/// 检测环境变量代理配置
///
/// # 返回
/// 返回检测到的环境变量代理信息
pub fn detect_env_proxy() -> Option<String> {
    let mut proxy_info = Vec::new();

    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY") {
        if !https_proxy.is_empty() {
            proxy_info.push(format!("HTTPS_PROXY={}", https_proxy));
        }
    }
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY") {
        if !http_proxy.is_empty() {
            proxy_info.push(format!("HTTP_PROXY={}", http_proxy));
        }
    }
    if let Ok(all_proxy) = std::env::var("ALL_PROXY") {
        if !all_proxy.is_empty() {
            proxy_info.push(format!("ALL_PROXY={}", all_proxy));
        }
    }
    if let Ok(no_proxy) = std::env::var("NO_PROXY") {
        if !no_proxy.is_empty() {
            proxy_info.push(format!("NO_PROXY={}", no_proxy));
        }
    }

    if proxy_info.is_empty() {
        None
    } else {
        Some(format!("检测到环境变量代理: {}", proxy_info.join(", ")))
    }
}

/// 创建带代理和环境变量回退的优化 HTTP 客户端（通用版本）
///
/// # 参数
/// - `proxy`: 配置文件中的代理 URL（可选）
/// - `url`: 请求 URL（用于日志）
/// - `log_prefix`: 日志前缀（例如："[缩略图缓存]" 或 "[下载任务]"）
/// - `log_level_info`: 是否使用 info 级别（否则使用 debug 级别）
///
/// # 返回
/// 返回配置好的优化 HTTP 客户端
pub fn create_client_with_env_fallback(
    proxy: Option<String>,
    url: &str,
    log_prefix: &str,
    log_level_info: bool,
) -> reqwest::Client {
    // 尝试使用配置文件代理
    if let Some(proxy_url) = proxy {
        if !proxy_url.is_empty() {
            if log_level_info {
                info!("[{}] [URL:{}] 使用配置文件代理: {}", log_prefix, url, proxy_url);
            } else {
                debug!("[{}] [URL:{}] 使用配置文件代理: {}", log_prefix, url, proxy_url);
            }
            match create_optimized_client_with_proxy(&proxy_url) {
                Ok(http_client) => {
                    if log_level_info {
                        info!("[{}] [URL:{}] 代理客户端创建成功", log_prefix, url);
                    } else {
                        debug!("[{}] [URL:{}] HTTP客户端创建成功（已优化）", log_prefix, url);
                    }
                    return http_client;
                }
                Err(e) => {
                    warn!("[{}] [URL:{}] 代理客户端创建失败: {}，尝试环境变量代理", log_prefix, url, e);
                }
            }
        }
    }

    // 尝试使用环境变量代理
    if let Some(env_proxy_url) = get_proxy_from_env() {
        if log_level_info {
            info!("[{}] [URL:{}] 使用环境变量代理: {}", log_prefix, url, env_proxy_url);
        } else {
            debug!("[{}] [URL:{}] 使用环境变量代理: {}", log_prefix, url, env_proxy_url);
        }
        match create_optimized_client_with_proxy(&env_proxy_url) {
            Ok(http_client) => {
                if log_level_info {
                    info!("[{}] [URL:{}] 环境变量代理客户端创建成功", log_prefix, url);
                } else {
                    debug!("[{}] [URL:{}] 环境变量代理客户端创建成功", log_prefix, url);
                }
                return http_client;
            }
            Err(e) => {
                warn!("[{}] [URL:{}] 环境变量代理客户端创建失败: {}，回退到无代理", log_prefix, url, e);
            }
        }
    } else {
        debug!("[{}] [URL:{}] 未检测到环境变量代理，回退到无代理", log_prefix, url);
    }

    // 回退到无代理
    if log_level_info {
        info!("[{}] [URL:{}] 使用无代理客户端（优化）", log_prefix, url);
    } else {
        debug!("[{}] [URL:{}] 使用无代理客户端（优化）", log_prefix, url);
    }
    create_optimized_client()
}
