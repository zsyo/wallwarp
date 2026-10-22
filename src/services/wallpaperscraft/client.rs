// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wallpaperscraft 列表页抓取
//!
//! 站点为服务端渲染 HTML，无公开 JSON API，故按固定类名正则提取条目：
//! 列表项为 `<li class="wallpapers__item">`，内部含详情页链接（`/wallpaper/{slug}`，
//! 带分辨率过滤时为 `/download/{slug}/{res}`）、缩略图（`_300x168.jpg` 是唯一
//! 可用缩略图档，其它尺寸一律 404）与标签信息块。分页末页链接含总页数

use crate::services::request_context::RequestContext;
use crate::services::source::http_log;
use regex::Regex;
use std::sync::LazyLock;
use tracing::{debug, error, info, trace, warn};

/// 站点根地址（列表页与详情页同源）
pub const BASE_URL: &str = "https://wallpaperscraft.com";
/// 图片资源根地址（原图直链按 `{slug}_{W}x{H}.jpg` 拼接）
pub const IMAGE_BASE: &str = "https://images.wallpaperscraft.com/image/single";
/// 每页条目数（站点固定 15）
pub const ITEMS_PER_PAGE: usize = 15;
/// 列表页请求超时秒数（HTML 体积较大，略放宽）
const TIMEOUT_SECS: u64 = 20;

/// 列表项（抓取结果，尚未映射为图源无关壁纸）
#[derive(Debug, Clone)]
pub struct WallpaperscraftItem {
    /// 条目 slug（同时是详情页路径段与图片文件名前缀）
    pub slug: String,
    /// 缩略图直链（300x168 档）
    pub thumb: String,
    /// 标签（逗号分隔，来自列表项标签信息块）
    pub tags: String,
    /// 下载次数（抓取失败记 0）
    pub downloads: u32,
}

/// 一页列表的抓取结果
#[derive(Debug, Clone)]
pub struct WallpaperscraftPage {
    pub items: Vec<WallpaperscraftItem>,
    /// 总页数（未找到末页链接时为 0）
    pub last_page: usize,
}

/// 单个列表项（非贪婪跨行匹配到闭合 li）
static ITEM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<li class="wallpapers__item".*?</li>"#).expect("列表项正则编译失败")
});

/// 详情页链接（带分辨率过滤时为 /download/{slug}/{res}）
static LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"href="/(?:wallpaper|download)/([^/"]+)"#).expect("链接正则编译失败")
});

/// 缩略图直链
static THUMB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<img class="wallpapers__image" src="([^"]+)""#).expect("缩略图正则编译失败")
});

/// 标签信息块（第二个 wallpapers__info，内容为纯文本标签）
static TAGS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<span class="wallpapers__info">([^<]*)</span>"#).expect("标签正则编译失败")
});

/// 下载次数
static DOWNLOADS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"gui-icon_download"></span>&nbsp;(\d+)"#).expect("下载数正则编译失败")
});

/// 末页链接（取其中的 page{N} 作为总页数；链接与类名之间跨行，需开启 dot_matches_new_line）
static LAST_PAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)pager__item_last-page.{0,300}?/page(\d+)"#).expect("末页正则编译失败")
});

/// 抓取一页列表页
///
/// # 参数
/// - `client`: 已配置代理的 HTTP 客户端
/// - `path`: 站点相对路径（如 `/tag/forest/ratings/1920x1080/page2`）
/// - `context`: 请求上下文（用于取消操作）
pub async fn fetch_page(
    client: &reqwest::Client,
    path: &str,
    context: &RequestContext,
) -> Result<WallpaperscraftPage, String> {
    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    // 获取并发控制许可（与其它图源共用全局并发上限）
    let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    let url = format!("{}{}", BASE_URL, path);
    let identifier = path.to_string();
    info!("[Wallpaperscraft] [{}] 请求URL: {}", identifier, url);

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send()
        .await
        .map_err(|e| {
            http_log::log_send_error("Wallpaperscraft", &identifier, &e, Some(TIMEOUT_SECS))
        })?;

    debug!(
        "[Wallpaperscraft] [{}] 响应状态: {}",
        identifier,
        response.status()
    );

    if !response.status().is_success() {
        return Err(http_log::log_api_error_status(
            "Wallpaperscraft",
            &identifier,
            response.status(),
        ));
    }

    let text = response.text().await.map_err(|e| {
        warn!("[Wallpaperscraft] [{}] 读取响应失败: {}", identifier, e);
        format!("读取响应失败: {}", e)
    })?;

    if context.check_cancelled().is_some() {
        return Err("请求已取消".to_string());
    }

    trace!(
        "[Wallpaperscraft] [{}] 响应体 {} 字节，摘要: {}",
        identifier,
        text.len(),
        http_log::body_excerpt(&text, 200)
    );

    let page = parse_page(&text);
    debug!(
        "[Wallpaperscraft] [{}] 解析成功，获取到 {} 张壁纸，总页数 {}",
        identifier,
        page.items.len(),
        page.last_page
    );
    Ok(page)
}

/// 解析列表页 HTML
fn parse_page(html: &str) -> WallpaperscraftPage {
    let items: Vec<WallpaperscraftItem> = ITEM_RE
        .find_iter(html)
        .filter_map(|m| parse_item(m.as_str()))
        .collect();
    let last_page = LAST_PAGE_RE
        .captures(html)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<usize>().ok())
        .unwrap_or(0);
    WallpaperscraftPage { items, last_page }
}

/// 解析单个列表项
fn parse_item(block: &str) -> Option<WallpaperscraftItem> {
    let slug = LINK_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())?;
    let thumb = THUMB_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default();
    let tags = TAGS_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();
    let downloads = DOWNLOADS_RE
        .captures(block)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(0);

    if slug.is_empty() || thumb.is_empty() {
        error!("[Wallpaperscraft] [{}] 条目缺少 slug 或缩略图，已跳过", slug);
        return None;
    }

    Some(WallpaperscraftItem {
        slug,
        thumb,
        tags,
        downloads,
    })
}

/// 拼接原图直链
pub fn image_url(slug: &str, resolution: &str) -> String {
    format!("{}/{}_{}.jpg", IMAGE_BASE, slug, resolution)
}
