// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wallpaperscraft 图源服务层
//!
//! 把通用筛选参数映射为站点路径段（关键词 tag / 分类 catalog / 排序 /
//! 分辨率 / 页码），抓取列表页后映射为图源无关壁纸条目。
//!
//! 能力边界（均经实测确认）：
//! - 排序与分辨率都是**路径段**，写成 query 参数一律 404
//! - 排序只有 date / ratings / downloads 三种，由 [`sort_segment`] 映射
//! - 分辨率路径只接受单个 `{W}x{H}`；多选时取第一个站点识别的值，其余丢弃
//! - 无比例/颜色/时间范围/纯净度维度，对应参数忽略

use super::client::{self, WallpaperscraftItem};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, Sorting, SourceFuture, SourceKind, WallpaperSource,
};
use crate::utils::helpers;
use tracing::{info, trace};

/// 站点识别为路径段的分辨率（实测；项目 Resolution 枚举中其余取值站点返回 404）
const SUPPORTED_RESOLUTIONS: [&str; 13] = [
    "2560x1080",
    "2560x1440",
    "3840x2160",
    "1280x720",
    "1600x900",
    "1920x1080",
    "1280x800",
    "1920x1200",
    "2560x1600",
    "3840x2400",
    "1280x960",
    "1600x1200",
    "1280x1024",
];

/// 未指定分辨率时的原图尺寸（站点列表页默认标注 1280x720，桌面壁纸偏小；
/// 实测抽样条目均提供 Full HD，故默认取 1920x1080）
const DEFAULT_RESOLUTION: &str = "1920x1080";

/// wallpaperscraft 图源服务
pub struct WallpaperscraftService {
    client: reqwest::Client,
}

impl WallpaperscraftService {
    /// 创建新的 wallpaperscraft 服务
    ///
    /// # 参数
    /// - `proxy`: 可选的代理 URL
    /// - `proxy_enabled`: 代理是否启用
    /// - `use_env_fallback`: 是否使用环境变量作为回退
    pub fn new(proxy: Option<String>, proxy_enabled: bool, use_env_fallback: bool) -> Self {
        Self {
            client: proxy::create_proxy_client(proxy, proxy_enabled, use_env_fallback),
        }
    }

    /// 站点排序路径段
    fn sort_segment(sorting: Sorting) -> &'static str {
        match sorting {
            // 站点无「相关度/随机」排序，回退到最新
            Sorting::DateAdded | Sorting::Relevance | Sorting::Random => "date",
            // 站点以下载量代表热度
            Sorting::Views => "downloads",
            Sorting::Favorites | Sorting::TopList | Sorting::Hot => "ratings",
        }
    }

    /// 关键词路径段（站点 tag 为单个小写单词，多词只取第一个）
    fn keyword_segment(query: &SearchQuery) -> Option<String> {
        let trimmed = query.query.trim();
        let first = trimmed.split_whitespace().next()?;
        if trimmed.split_whitespace().count() > 1 {
            trace!(
                "[Wallpaperscraft] [{}] 站点标签仅支持单个单词，只取第一个词",
                trimmed
            );
        }
        Some(first.to_lowercase())
    }

    /// 分辨率路径段（None 表示不限）
    fn resolution_segment(query: &SearchQuery) -> Option<String> {
        // 「精确分辨率」模式下 resolutions 为逗号分隔的多选值
        if let Some(list) = &query.resolutions {
            let picked = list
                .split(',')
                .filter(|item| SUPPORTED_RESOLUTIONS.contains(item))
                .map(|item| item.to_string())
                .next();
            if picked.is_none() {
                trace!(
                    "[Wallpaperscraft] [{}] 所选分辨率站点不支持，已忽略分辨率筛选",
                    list
                );
            }
            return picked;
        }

        // 「最小分辨率」模式：站点无 >= 语义，按档位取代表值（近似）
        query.atleast.as_deref().and_then(|value| match value {
            "standard" => Some("1920x1080".to_string()),
            "wide" => Some("2560x1440".to_string()),
            "ultrawide" => Some("2560x1080".to_string()),
            other if SUPPORTED_RESOLUTIONS.contains(&other) => Some(other.to_string()),
            other => {
                trace!(
                    "[Wallpaperscraft] [{}] 最小分辨率站点不支持，已忽略分辨率筛选",
                    other
                );
                None
            }
        })
    }

    /// 组装列表页路径
    fn build_path(query: &SearchQuery) -> String {
        let mut path = String::new();
        if let Some(keyword) = Self::keyword_segment(query) {
            path.push_str("/tag/");
            path.push_str(&keyword);
        } else if let Some(category) = &query.site_category {
            path.push_str("/catalog/");
            path.push_str(category);
        } else {
            // 裸 /all 返回 404，必须带排序或分辨率段
            path.push_str("/all");
        }

        path.push('/');
        path.push_str(Self::sort_segment(query.sorting));

        if let Some(resolution) = Self::resolution_segment(query) {
            path.push('/');
            path.push_str(&resolution);
        }

        if query.page > 1 {
            path.push_str(&format!("/page{}", query.page));
        }
        path
    }

    /// 将抓取条目映射为图源无关的在线壁纸条目
    ///
    /// `resolution` 为本次请求实际使用的尺寸档，原图直链按它拼接
    fn to_online_wallpaper(item: WallpaperscraftItem, resolution: &str) -> OnlineWallpaper {
        let (width, height) = Self::parse_dimensions(resolution);
        OnlineWallpaper {
            id: item.slug.clone(),
            // 详情页（站点无独立 API，用页面链接标识来源）
            url: format!("{}/wallpaper/{}", client::BASE_URL, item.slug),
            path: client::image_url(&item.slug, resolution),
            thumb_large: item.thumb.clone(),
            thumb_original: item.thumb.clone(),
            thumb_small: item.thumb,
            width,
            height,
            resolution: resolution.to_string(),
            ratio: helpers::simplified_ratio(width, height),
            // 列表页不提供文件大小，按「未知」记 0
            file_size: 0,
            file_type: "image/jpeg".to_string(),
            category: item.tags,
            purity: "sfw".to_string(),
            views: item.downloads,
            favorites: 0,
            colors: Vec::new(),
            source: SourceKind::Wallpaperscraft,
            image_handle: None,
        }
    }

    /// 从 `{W}x{H}` 解析宽高（解析失败记 0）
    fn parse_dimensions(resolution: &str) -> (u32, u32) {
        let mut parts = resolution.split('x');
        let width = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        let height = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
        (width, height)
    }
}

impl WallpaperSource for WallpaperscraftService {
    fn kind(&self) -> SourceKind {
        SourceKind::Wallpaperscraft
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // 站点无比例/颜色/时间范围/纯净度维度，这些参数直接忽略
            trace!(
                "[Wallpaperscraft] [page{}] 忽略 ratios/color/source_color/time_range/purities/categories 参数",
                query.page
            );

            let path = Self::build_path(query);
            let resolution = Self::resolution_segment(query)
                .unwrap_or_else(|| DEFAULT_RESOLUTION.to_string());

            let page = client::fetch_page(&self.client, &path, context).await?;
            let wallpapers: Vec<OnlineWallpaper> = page
                .items
                .into_iter()
                .map(|item| Self::to_online_wallpaper(item, &resolution))
                .collect();

            info!(
                "[Wallpaperscraft] [page{}] 返回 {} 张壁纸（总页数 {}）",
                query.page,
                wallpapers.len(),
                page.last_page
            );

            let is_last = if page.last_page > 0 {
                query.page >= page.last_page
            } else {
                wallpapers.len() < client::ITEMS_PER_PAGE
            };

            Ok(SearchResult {
                wallpapers,
                is_last,
                total_pages: page.last_page,
                current_page: query.page,
            })
        })
    }
}
