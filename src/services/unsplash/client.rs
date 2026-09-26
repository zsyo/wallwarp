// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Unsplash API HTTP 客户端
//!
//! 接口：`https://api.unsplash.com`，key 免费注册（请求经
//! `Authorization: Client-ID <key>` 头认证；demo key 限流 50 次/小时）。
//! 图片资源在 images.unsplash.com，不计入 API 限流。
//! 响应类型仅反序列化本功能用到的字段

use crate::services::request_context::RequestContext;
use crate::services::source::http_log;
use serde::Deserialize;
use tracing::{debug, error, info, trace, warn};

/// Unsplash API 根地址
pub const BASE_URL: &str = "https://api.unsplash.com";
/// 每页固定返回条数（接口上限 30）
pub const PER_PAGE: usize = 24;
/// 检索请求超时秒数（对齐 Wallhaven 检索请求）
const TIMEOUT_SECS: u64 = 5;

/// search/photos 接口响应
#[derive(Debug, Clone, Deserialize)]
pub struct UnsplashResponse {
    #[serde(default)]
    pub total_pages: u64,
    #[serde(default)]
    pub results: Vec<UnsplashPhoto>,
}

/// Unsplash 图片条目（字段按官方文档 camelCase 映射）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsplashPhoto {
    pub id: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    /// 主色 hex（如 "#a3a3a3"）
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub likes: u64,
    #[serde(default)]
    pub urls: UnsplashUrls,
    #[serde(default)]
    pub links: UnsplashLinks,
}

/// 各尺寸图片直链（images.unsplash.com，不计入 API 限流）
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsplashUrls {
    /// 原图直链（带 ixid 参数，须保留以完成访问归因）
    #[serde(default)]
    pub raw: String,
    /// 1080px 宽 jpg
    #[serde(default)]
    pub regular: String,
    /// 200px 宽 jpg
    #[serde(default)]
    pub thumb: String,
}

/// 图片相关链接
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsplashLinks {
    /// 详情页
    #[serde(default)]
    pub html: String,
    /// 下载埋点地址（API 指南要求下载动作时触发）
    ///
    /// 已知取舍：暂不实现。链路为无状态 per-request 服务（每次检索新建
    /// source 实例），跨进程生命周期保存 download_location 无处存放；
    /// 该 endpoint 仅为下载量埋点，不触发不会导致封禁，后续如需要
    /// 可在下载任务链路补一次调用
    #[serde(default)]
    #[allow(dead_code)]
    pub download_location: String,
}

/// search/photos 接口查询参数（服务层负责把统一 SearchQuery 映射到此结构）
#[derive(Debug, Clone, Copy)]
pub struct SearchParams<'a> {
    pub key: &'a str,
    /// 关键词（空串时接口无实际过滤语义，调用方应保证非空或由 Unsplash 报错）
    pub query: &'a str,
    /// 页码（从 1 开始）
    pub page: usize,
    /// landscape / portrait / squarish（None 不限制方向）
    pub orientation: Option<&'a str>,
    /// relevant / latest
    pub order_by: &'a str,
    /// 官方颜色值（black_and_white/red/...，None 不限制）
    pub color: Option<&'a str>,
}

/// 按条件检索一页图片
///
/// # 参数
/// - `client`: 已配置代理的 HTTP 客户端
/// - `params`: 检索参数
/// - `context`: 请求上下文（用于取消操作）
pub async fn search(
    client: &reqwest::Client,
    params: SearchParams<'_>,
    context: &RequestContext,
) -> Result<UnsplashResponse, String> {
    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    // 获取并发控制许可（与其它图源共用全局并发上限）
    let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER
        .acquire()
        .await;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    let mut url = format!(
        "{}/search/photos?query={}&page={}&per_page={}&content_filter=high&order_by={}",
        BASE_URL,
        urlencoding::encode(params.query),
        params.page,
        PER_PAGE,
        params.order_by,
    );
    if let Some(orientation) = params.orientation {
        url.push_str(&format!("&orientation={}", orientation));
    }
    if let Some(color) = params.color {
        url.push_str(&format!("&color={}", color));
    }

    let identifier = format!("page{}", params.page);
    info!("[Unsplash API] [{}] 请求URL: {}", identifier, url);

    let response = client
        .get(&url)
        .header("Authorization", format!("Client-ID {}", params.key))
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| http_log::log_send_error("Unsplash", &identifier, &e, Some(TIMEOUT_SECS)))?;

    debug!(
        "[Unsplash API] [{}] 响应状态: {}",
        identifier,
        response.status()
    );

    if !response.status().is_success() {
        return Err(http_log::log_api_error_status(
            "Unsplash",
            &identifier,
            response.status(),
        ));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[Unsplash API] [{}] 读取响应失败: {}", identifier, e);
        format!("读取响应失败: {}", e)
    })?;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    trace!(
        "[Unsplash API] [{}] 响应体 {} 字节，摘要: {}",
        identifier,
        text.len(),
        http_log::body_excerpt(&text, 200)
    );

    let parsed: UnsplashResponse = serde_json::from_str(&text).map_err(|e| {
        error!("[Unsplash API] [{}] JSON解析失败: {}", identifier, e);
        format!("解析JSON失败: {}", e)
    })?;

    debug!(
        "[Unsplash API] [{}] 解析成功，获取到 {} 张壁纸",
        identifier,
        parsed.results.len()
    );
    Ok(parsed)
}
