// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Danbooru API HTTP 客户端
//!
//! 接口：`https://danbooru.donmai.us/posts.json`，读操作免密钥（tags 检索 +
//! 分页）。字段按官方 API 文档 snake_case 映射

use crate::services::request_context::RequestContext;
use crate::services::source::http_log;
use serde::Deserialize;
use tracing::{debug, error, info, trace, warn};

/// Danbooru API 根地址
pub const BASE_URL: &str = "https://danbooru.donmai.us";
/// 每页固定返回条数（接口上限 200）
pub const PER_PAGE: usize = 24;
/// 检索请求超时秒数（对齐 Wallhaven 检索请求）
const TIMEOUT_SECS: u64 = 5;

/// posts 接口响应（直接返回条目数组）
#[derive(Debug, Clone, Deserialize)]
pub struct DanbooruPost {
    pub id: u64,
    /// 原图直链（cdn.donmai.us）
    #[serde(default)]
    pub file_url: String,
    /// 约 1500px 缩放版直链（部分小图无此字段，回退 preview）
    #[serde(default)]
    pub large_file_url: String,
    /// 预览直链
    #[serde(default)]
    pub preview_file_url: String,
    pub image_width: u32,
    pub image_height: u32,
    #[serde(default)]
    pub file_size: u64,
    /// 文件扩展名（如 jpg/png）
    #[serde(default)]
    pub file_ext: String,
    /// 内容分级：g(一般) / s(敏感) / q(存疑) / e(暴露)
    #[serde(default)]
    pub rating: String,
    /// 评分（收藏加权热度）
    #[serde(default)]
    pub score: i64,
    /// 收藏数
    #[serde(default)]
    pub fav_count: i64,
}

/// posts 接口查询参数（服务层负责把统一 SearchQuery 映射到此结构）
#[derive(Debug, Clone, Copy)]
pub struct SearchParams<'a> {
    /// tags 查询串（空格分隔为 AND，已含 rating/order metatag）
    pub tags: &'a str,
    /// 页码（从 1 开始）
    pub page: usize,
}

/// 按 tags 检索一页图片
///
/// # 参数
/// - `client`: 已配置代理的 HTTP 客户端
/// - `params`: 检索参数
/// - `context`: 请求上下文（用于取消操作）
pub async fn search(
    client: &reqwest::Client,
    params: SearchParams<'_>,
    context: &RequestContext,
) -> Result<Vec<DanbooruPost>, String> {
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

    let url = format!(
        "{}/posts.json?tags={}&page={}&limit={}",
        BASE_URL,
        urlencoding::encode(params.tags),
        params.page,
        PER_PAGE,
    );

    let identifier = format!("page{}", params.page);
    info!("[Danbooru API] [{}] 请求URL: {}", identifier, url);

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| http_log::log_send_error("Danbooru", &identifier, &e, Some(TIMEOUT_SECS)))?;

    debug!(
        "[Danbooru API] [{}] 响应状态: {}",
        identifier,
        response.status()
    );

    if !response.status().is_success() {
        return Err(http_log::log_api_error_status(
            "Danbooru",
            &identifier,
            response.status(),
        ));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[Danbooru API] [{}] 读取响应失败: {}", identifier, e);
        format!("读取响应失败: {}", e)
    })?;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    trace!(
        "[Danbooru API] [{}] 响应体 {} 字节，摘要: {}",
        identifier,
        text.len(),
        http_log::body_excerpt(&text, 200)
    );

    let parsed: Vec<DanbooruPost> = serde_json::from_str(&text).map_err(|e| {
        error!("[Danbooru API] [{}] JSON解析失败: {}", identifier, e);
        format!("解析JSON失败: {}", e)
    })?;

    debug!(
        "[Danbooru API] [{}] 解析成功，获取到 {} 张壁纸",
        identifier,
        parsed.len()
    );
    Ok(parsed)
}
