// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Bing 每日壁纸 HTTP 客户端
//!
//! HPImageArchive 接口：`format=js` 返回 JSON 图集，`idx` 为起始天数偏移
//! （0 = 今天，8 = 8 天前），`n` 为单次返回天数（每日一张）

use crate::services::request_context::RequestContext;
use serde::Deserialize;
use tracing::{debug, warn};

/// Bing 国内站根地址（图集接口与图片资源同源）
pub const BASE_URL: &str = "https://cn.bing.com";
/// 单次请求返回的天数（每页壁纸数）
pub const IMAGES_PER_PAGE: u32 = 8;

/// HPImageArchive 接口响应
#[derive(Debug, Clone, Deserialize)]
pub struct BingResponse {
    #[serde(default)]
    pub images: Vec<BingImage>,
}

/// Bing 每日壁纸条目（仅反序列化本功能用到的字段，其余由 serde 忽略）
#[derive(Debug, Clone, Deserialize)]
pub struct BingImage {
    /// 相对图片直链基底（无尺寸后缀，可拼接任意尺寸变体）
    #[serde(default)]
    pub urlbase: String,
    /// 唯一哈希（作为壁纸 id）
    pub hsh: String,
}

/// 拉取一页每日图集（阻塞 HTTP 由调用方经异步上下文执行）
///
/// # 参数
/// - `page`: 页码（从 1 开始，第 N 页返回第 N*8 天前至 (N-1)*8+1 天前的图集）
/// - `client`: 已配置代理的 HTTP 客户端
/// - `context`: 请求上下文（用于取消操作）
pub async fn fetch_images_page(
    client: &reqwest::Client,
    page: usize,
    context: &RequestContext,
) -> Result<BingResponse, String> {
    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    // 获取并发控制许可（与其它图源共用全局并发上限）
    let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    let idx = (page - 1) * IMAGES_PER_PAGE as usize;
    let url = format!(
        "{}/HPImageArchive.aspx?format=js&idx={}&n={}&mkt=zh-cn",
        BASE_URL, idx, IMAGES_PER_PAGE
    );

    debug!("[Bing API] [page{}] 请求URL: {}", page, url);

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| {
            warn!("[Bing API] [page{}] 请求失败: {}", page, e);
            format!("请求失败: {}", e)
        })?;

    debug!(
        "[Bing API] [page{}] 响应状态: {}",
        page,
        response.status()
    );

    if !response.status().is_success() {
        return Err(format!("API返回错误: {}", response.status()));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[Bing API] [page{}] 读取响应失败: {}", page, e);
        format!("读取响应失败: {}", e)
    })?;

    if let Some(()) = context.check_cancelled() {
        return Err("请求已取消".to_string());
    }

    let parsed: BingResponse = serde_json::from_str(&text).map_err(|e| {
        warn!("[Bing API] [page{}] JSON解析失败: {}", page, e);
        format!("解析JSON失败: {}", e)
    })?;

    debug!(
        "[Bing API] [page{}] 解析成功，获取到 {} 张壁纸",
        page,
        parsed.images.len()
    );
    Ok(parsed)
}

/// 拼接原图直链（1920x1080 版本）
pub fn full_image_url(urlbase: &str) -> String {
    format!("{}{}_1920x1080.jpg", BASE_URL, urlbase)
}

/// 拼接缩略图直链（th 服务按 w/h 参数缩放裁剪）
pub fn thumb_image_url(urlbase: &str, width: u32, height: u32) -> String {
    format!(
        "{}{}_1920x1080.jpg&w={}&h={}&c=7",
        BASE_URL, urlbase, width, height
    )
}
