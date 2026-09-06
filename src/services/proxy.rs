// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 代理客户端创建模块
//!
//! 所有 HTTP 请求路径共用同一套客户端配置（[`build_client`] 单一构建入口），
//! 代理来源优先级：配置文件代理 > 环境变量代理（回退）> 直连。
//! 创建出的客户端按最终生效代理配置缓存复用（[`cached_client`]）。
//!
//! 注意：不使用 `http2_prior_knowledge`（跳过 ALPN 协商对不支持 HTTP/2 的
//! 图床与代理会导致全部请求失败），依赖 TLS ALPN 自动协商协议版本。

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::Duration;
use tracing::{debug, error, info};

/// 连接超时
const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
/// 请求总超时
const REQUEST_TIMEOUT: Duration = Duration::from_secs(300);
/// 空闲连接保活时长
const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);

/// 按最终生效代理配置缓存的 HTTP 客户端池（键：代理 URL，None=直连）
///
/// reqwest::Client 内部为 Arc，克隆廉价；高频路径（缩略图加载、流式下载等）
/// 若每次请求都新建 Client，TLS 握手与连接池会反复初始化且完全失去连接复用。
/// 缓存键为解析后的代理 URL，代理设置变更自然产生新键，无需失效逻辑
static CLIENT_CACHE: LazyLock<Mutex<HashMap<Option<String>, reqwest::Client>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 获取（或首次创建并缓存）指定代理配置的 HTTP 客户端
///
/// # 参数
/// - `resolved_proxy`: 最终生效的代理 URL（None=直连）
fn cached_client(resolved_proxy: Option<String>) -> reqwest::Client {
    // 锁中毒仅说明其他线程持锁时 panic，map 数据本身仍可用，取回继续
    let mut cache = CLIENT_CACHE.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(client) = cache.get(&resolved_proxy) {
        return client.clone();
    }
    let client = build_client(resolved_proxy.as_deref()).unwrap_or_else(|e| {
        error!(
            "[代理客户端] 客户端创建失败 (proxy: {:?}): {}，回退到默认客户端",
            resolved_proxy, e
        );
        reqwest::Client::new()
    });
    cache.insert(resolved_proxy, client.clone());
    client
}

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

/// 创建带代理的 HTTP 客户端（结果经 [`cached_client`] 缓存复用）
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
    // 解析最终生效的代理（优先级：配置文件代理 > 环境变量代理 > 直连）
    let resolved_proxy = if proxy_enabled
        && let Some(proxy_url) = proxy
        && !proxy_url.is_empty()
    {
        info!("[代理客户端] 使用配置文件代理: {}", proxy_url);
        Some(proxy_url)
    } else if use_env_fallback && let Some(env_proxy_url) = get_proxy_from_env() {
        info!("[代理客户端] 使用环境变量代理: {}", env_proxy_url);
        Some(env_proxy_url)
    } else {
        debug!("[代理客户端] 使用直连客户端");
        None
    };

    cached_client(resolved_proxy)
}

/// 创建带代理和环境变量回退的 HTTP 客户端（下载路径通用版本，
/// 结果经 [`cached_client`] 缓存复用）
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
    // 按调用方的日志级别要求输出代理选择结果
    let log_choice = |message: String| {
        if log_level_info {
            info!("[{}] [URL:{}] {}", log_prefix, url, message);
        } else {
            debug!("[{}] [URL:{}] {}", log_prefix, url, message);
        }
    };

    // 解析最终生效的代理（优先级：配置文件代理 > 环境变量代理 > 直连）
    let resolved_proxy = if let Some(proxy_url) = proxy.filter(|p| !p.is_empty()) {
        log_choice(format!("使用配置文件代理: {}", proxy_url));
        Some(proxy_url)
    } else if let Some(env_proxy_url) = get_proxy_from_env() {
        log_choice(format!("使用环境变量代理: {}", env_proxy_url));
        Some(env_proxy_url)
    } else {
        log_choice("使用直连客户端".to_string());
        None
    };

    cached_client(resolved_proxy)
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
