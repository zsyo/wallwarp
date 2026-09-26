// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Pixabay API HTTP 客户端
//!
//! 接口：`https://pixabay.com/api/`，key 免费注册（以查询参数附加，限流与
//! key 绑定而非 IP）。响应类型仅反序列化本功能用到的字段

use crate::services::request_context::RequestContext;
use crate::services::source::http_log;
use serde::Deserialize;
use tracing::{debug, error, info, trace, warn};

/// Pixabay API 根地址
pub const BASE_URL: &str = "https://pixabay.com/api";
/// 每页固定返回条数（API 接受 3-200）
pub const PER_PAGE: usize = 24;
/// 单关键词最大命中数（接口上限，超过的页数返回空结果）
const MAX_HITS: u64 = 500;
/// 检索请求超时秒数（对齐 Wallhaven 检索请求）
const TIMEOUT_SECS: u64 = 5;

/// Pixabay 接口响应
#[derive(Debug, Clone, Deserialize)]
pub struct PixabayResponse {
    #[serde(default)]
    pub total_hits: u64,
    #[serde(default)]
    pub hits: Vec<PixabayHit>,
}

/// Pixabay 图片条目（字段按官方文档 camelCase 映射）
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixabayHit {
    pub id: u64,
    #[serde(default)]
    pub page_url: String,
    #[serde(default)]
    pub webformat_url: String,
    #[serde(default)]
    pub large_image_url: String,
    /// 原图直链；矢量等无原图的条目为空串，映射时跳过
    #[serde(default)]
    pub image_url: String,
    pub image_width: u32,
    pub image_height: u32,
    #[serde(default)]
    pub image_size: u64,
    #[serde(default)]
    pub views: u64,
    #[serde(default)]
    pub likes: u64,
}

/// search 接口查询参数（服务层负责把统一 SearchQuery 映射到此结构）
#[derive(Debug, Clone, Copy)]
pub struct SearchParams<'a> {
    pub key: &'a str,
    /// 关键词（空串时接口按热门排序返回全量结果）
    pub query: &'a str,
    /// 页码（从 1 开始）
    pub page: usize,
    /// horizontal / vertical（None 不限制方向）
    pub orientation: Option<&'a str>,
    /// 原图最小宽度（None 不限制）
    pub min_width: Option<u32>,
    /// ec / latest / popular
    pub order: &'a str,
    /// 官方分类值（backgrounds/fashion/...，None 不限制）
    pub category: Option<&'a str>,
    /// 官方颜色值（grayscale/red/...，None 不限制）
    pub color: Option<&'a str>,
    pub editors_choice: bool,
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
) -> Result<PixabayResponse, String> {
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
        "{}/?key={}&q={}&page={}&per_page={}&image_type=photo&safesearch=true&order={}",
        BASE_URL,
        params.key,
        urlencoding::encode(params.query),
        params.page,
        PER_PAGE,
        params.order,
    );
    if let Some(orientation) = params.orientation {
        url.push_str(&format!("&orientation={}", orientation));
    }
    if let Some(min_width) = params.min_width {
        url.push_str(&format!("&min_width={}", min_width));
    }
    if let Some(category) = params.category {
        url.push_str(&format!("&category={}", category));
    }
    if let Some(color) = params.color {
        url.push_str(&format!("&colors={}", color));
    }
    if params.editors_choice {
        url.push_str("&editors_choice=true");
    }

    let identifier = format!("page{}", params.page);
    info!(
        "[Pixabay API] [{}] 请求URL: {}",
        identifier,
        mask_api_key_in_url(&url)
    );

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| http_log::log_send_error("Pixabay", &identifier, &e, Some(TIMEOUT_SECS)))?;

    debug!(
        "[Pixabay API] [{}] 响应状态: {}",
        identifier,
        response.status()
    );

    if !response.status().is_success() {
        return Err(http_log::log_api_error_status(
            "Pixabay",
            &identifier,
            response.status(),
        ));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[Pixabay API] [{}] 读取响应失败: {}", identifier, e);
        format!("读取响应失败: {}", e)
    })?;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    trace!(
        "[Pixabay API] [{}] 响应体 {} 字节，摘要: {}",
        identifier,
        text.len(),
        http_log::body_excerpt(&text, 200)
    );

    let parsed: PixabayResponse = serde_json::from_str(&text).map_err(|e| {
        error!("[Pixabay API] [{}] JSON解析失败: {}", identifier, e);
        format!("解析JSON失败: {}", e)
    })?;

    debug!(
        "[Pixabay API] [{}] 解析成功，获取到 {} 张壁纸",
        identifier,
        parsed.hits.len()
    );
    Ok(parsed)
}

/// 日志脱敏：URL 中 key 参数替换为掩码（掩码规则复用 Wallhaven 的 key 脱敏）
pub fn mask_api_key_in_url(url: &str) -> String {
    let Some(start) = url.find("key=") else {
        return url.to_string();
    };
    let key_start = start + 4; // "key=" 的长度
    let key_end = url[key_start..]
        .find('&')
        .map(|end| key_start + end)
        .unwrap_or(url.len());
    format!(
        "{}{}{}",
        &url[..key_start],
        crate::services::wallhaven::mask_api_key(&url[key_start..key_end]),
        &url[key_end..]
    )
}

/// 按总命中数与页大小计算总页数（Pixabay 单关键词上限 MAX_HITS）
pub fn total_pages(total_hits: u64) -> usize {
    let pages = total_hits.div_ceil(PER_PAGE as u64);
    pages.min(MAX_HITS.div_ceil(PER_PAGE as u64)) as usize
}
