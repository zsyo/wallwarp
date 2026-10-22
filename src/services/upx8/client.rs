// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wp.upx8 壁纸 API 客户端
//!
//! 接口为 `GET {BASE_URL}/api.php`，免密钥，限流 60 次/分钟/IP。接口特征
//! 均经实测确认：
//! - `count` 上限 10，无分页（`page` 参数被忽略，每次随机返回）
//! - `data` 字段在 `count=1` 时是对象、`count>1` 时是数组，故按 untagged 兼容
//! - `width`/`height` 恒为 0、`title` 恒为空串；`resolution` 只是声明值，
//!   birdpaper 源按原图尺寸返回，不做强制缩放
//! - 图片直链为 `http://`，换 `https://` 同样可用（本模块统一升为 https）
//! - CDN 支持 OSS 图片处理参数 `?x-oss-process=image/resize,w_{N}`，用于按
//!   档位取缩略图（原图动辄 1MB 以上，不宜直接进列表）

use crate::services::request_context::RequestContext;
use crate::services::source::http_log;
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, error, info, trace, warn};

/// 接口根地址
pub const BASE_URL: &str = "https://wp.upx8.com";
/// 单次请求的最大条目数（接口上限 10）
pub const ITEMS_PER_PAGE: usize = 10;
/// 请求超时秒数（接口带双源兜底，失败切换耗时略长）
const TIMEOUT_SECS: u64 = 20;
/// 列表缩略图宽度（CDN OSS 处理参数）
pub const THUMB_WIDTH: u32 = 300;
/// 大图预览宽度（CDN OSS 处理参数）
pub const THUMB_LARGE_WIDTH: u32 = 800;

/// 单张壁纸
#[derive(Debug, Clone, Deserialize)]
pub struct Upx8Image {
    /// 图片直链（响应内为 http，使用时升为 https）
    pub url: String,
    /// 声明分辨率（如 `1920x1080`；birdpaper 源不代表实际尺寸）
    #[serde(default)]
    pub resolution: String,
    /// 数据源标识（birdpaper / bing）
    #[serde(default)]
    pub source: String,
}

/// JSON 响应
#[derive(Debug, Deserialize)]
struct Upx8Response {
    code: u32,
    #[serde(default)]
    message: String,
    #[serde(default)]
    data: Option<Upx8Data>,
}

/// `data` 字段的双形态（单条为对象，多条为数组）
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Upx8Data {
    One(Box<Upx8Image>),
    Many(Vec<Upx8Image>),
}

impl Upx8Data {
    fn into_vec(self) -> Vec<Upx8Image> {
        match self {
            Upx8Data::One(image) => vec![*image],
            Upx8Data::Many(images) => images,
        }
    }
}

/// 请求参数（已由图源层映射为站点取值）
#[derive(Debug, Clone, Default)]
pub struct Upx8Query {
    /// 关键词（与分类同传时接口以关键词优先）
    pub keyword: Option<String>,
    /// 分类 key（nature/anime/game/animal/city/abstract/space/car/girl/sport）
    pub category: Option<String>,
    /// 分辨率（站点六档之一；birdpaper 源仅作记录）
    pub resolution: Option<String>,
    /// 请求条数（上限 10）
    pub count: usize,
}

impl Upx8Query {
    /// 日志标识（含操作参数摘要，便于按条件追踪请求）
    fn identifier(&self) -> String {
        let scope = match (&self.keyword, &self.category) {
            (Some(keyword), _) => format!("kw:{}", keyword),
            (None, Some(category)) => format!("cat:{}", category),
            (None, None) => "random".to_string(),
        };
        match &self.resolution {
            Some(resolution) => format!("{},res:{},count:{}", scope, resolution, self.count),
            None => format!("{},count:{}", scope, self.count),
        }
    }

    /// 拼接带参查询串
    fn to_query_string(&self) -> String {
        let mut params: Vec<String> = vec!["format=json".to_string()];
        if let Some(keyword) = &self.keyword {
            params.push(format!("content={}", urlencoding::encode(keyword)));
        } else if let Some(category) = &self.category {
            params.push(format!("category={}", urlencoding::encode(category)));
        }
        if let Some(resolution) = &self.resolution {
            params.push(format!("resolution={}", urlencoding::encode(resolution)));
        }
        params.push(format!("count={}", self.count));
        params.join("&")
    }
}

/// 抓取图片列表
///
/// # 参数
/// - `client`: 已配置代理的 HTTP 客户端
/// - `query`: 请求参数
/// - `context`: 请求上下文（用于取消操作）
pub async fn fetch_images(
    client: &reqwest::Client,
    query: &Upx8Query,
    context: &RequestContext,
) -> Result<Vec<Upx8Image>, String> {
    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    // 获取并发控制许可（与其它图源共用全局并发上限）
    let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    let identifier = query.identifier();
    let url = format!("{}/api.php?{}", BASE_URL, query.to_query_string());
    info!("[wp.upx8 API] [{}] 请求URL: {}", identifier, url);

    let response = client
        .get(&url)
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| http_log::log_send_error("wp.upx8 API", &identifier, &e, Some(TIMEOUT_SECS)))?;

    debug!(
        "[wp.upx8 API] [{}] 响应状态: {}",
        identifier,
        response.status()
    );

    if !response.status().is_success() {
        return Err(http_log::log_api_error_status(
            "wp.upx8 API",
            &identifier,
            response.status(),
        ));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[wp.upx8 API] [{}] 读取响应失败: {}", identifier, e);
        format!("读取响应失败: {}", e)
    })?;

    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    trace!(
        "[wp.upx8 API] [{}] 响应体 {} 字节，摘要: {}",
        identifier,
        text.len(),
        http_log::body_excerpt(&text, 200)
    );

    let parsed: Upx8Response = serde_json::from_str(&text).map_err(|e| {
        error!("[wp.upx8 API] [{}] 解析JSON失败: {}", identifier, e);
        format!("解析JSON失败: {}", e)
    })?;

    if parsed.code != 200 {
        warn!(
            "[wp.upx8 API] [{}] 接口返回错误码 {}: {}",
            identifier, parsed.code, parsed.message
        );
        return Err(format!(
            "接口返回错误码 {}: {}",
            parsed.code, parsed.message
        ));
    }

    let images = parsed
        .data
        .map(Upx8Data::into_vec)
        .unwrap_or_default()
        .into_iter()
        .filter(|image| !image.url.is_empty())
        .collect::<Vec<Upx8Image>>();

    debug!("[wp.upx8 API] [{}] 解析成功，获取到 {} 张壁纸", identifier, images.len());
    Ok(images)
}

/// 把响应内的 http 直链升为 https（实测两种协议均可用，https 避免混合内容）
pub fn normalize_url(url: &str) -> String {
    match url.strip_prefix("http://") {
        Some(rest) => format!("https://{}", rest),
        None => url.to_string(),
    }
}

/// 按宽度拼接 CDN 缩略图直链（OSS 图片处理参数，保持原图宽高比）
pub fn thumb_url(url: &str, width: u32) -> String {
    format!(
        "{}?x-oss-process=image/resize,w_{}",
        normalize_url(url),
        width
    )
}
