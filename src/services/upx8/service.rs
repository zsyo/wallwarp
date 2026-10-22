// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wp.upx8 图源服务层
//!
//! 把通用筛选参数映射为接口参数（关键词 / 分类 / 分辨率 / 条数），抓取后
//! 映射为图源无关壁纸条目。
//!
//! 能力边界（均经实测确认）：
//! - 只有六档分辨率（1280x720 / 1366x768 / 1600x900 / 1920x1080 /
//!   2560x1440 / 3840x2160），项目其余分辨率按面积就近向上映射
//! - 无分页（`page` 参数被忽略，每次随机返回），结果恒标记为最后一页
//! - 无排序、无比例/颜色/时间范围/纯净度维度，对应参数忽略
//! - 返回的 `width`/`height` 恒为 0，宽高取自声明分辨率

use super::client::{self, Upx8Image};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, SourceFuture, SourceKind, WallpaperSource,
};
use crate::utils::helpers;
use tracing::{error, info, trace};

/// 站点支持的分辨率档位（按面积升序，用于就近映射）
const SUPPORTED_RESOLUTIONS: [&str; 6] = [
    "1280x720",
    "1366x768",
    "1600x900",
    "1920x1080",
    "2560x1440",
    "3840x2160",
];

/// 未指定分辨率时的请求档位（接口默认值）
const DEFAULT_RESOLUTION: &str = "1920x1080";

/// wp.upx8 图源服务
pub struct Upx8Service {
    client: reqwest::Client,
}

impl Upx8Service {
    /// 创建新的 wp.upx8 服务
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

    /// 关键词参数（接口上限 100 字符）
    fn keyword_param(query: &SearchQuery) -> Option<String> {
        let trimmed = query.query.trim();
        if trimmed.is_empty() {
            return None;
        }
        let limited: String = trimmed.chars().take(100).collect();
        if limited.chars().count() < trimmed.chars().count() {
            trace!(
                "[wp.upx8] [{}] 关键词超过 100 字符，已截断",
                trimmed.chars().take(20).collect::<String>()
            );
        }
        Some(limited)
    }

    /// 分辨率参数（None 表示使用接口默认档）
    fn resolution_param(query: &SearchQuery) -> Option<&'static str> {
        // 「精确分辨率」取多选列表首项，「最小分辨率」取单值
        let target = query
            .resolutions
            .as_deref()
            .and_then(|list| list.split(',').next())
            .or(query.atleast.as_deref())?;

        match target {
            "any" => None,
            // 档位语义取站点六档中的代表值
            "standard" => Some("1920x1080"),
            "wide" => Some("2560x1440"),
            "ultrawide" => Some("2560x1440"),
            other => Some(Self::nearest_resolution(other)),
        }
    }

    /// 把任意分辨率映射为站点档位（取面积不小于目标的最小档，超出则取最大档）
    fn nearest_resolution(spec: &str) -> &'static str {
        let (width, height) = Self::parse_dimensions(spec);
        if width == 0 || height == 0 {
            trace!(
                "[wp.upx8] [{}] 分辨率取值无法解析，使用默认档 {}",
                spec,
                DEFAULT_RESOLUTION
            );
            return DEFAULT_RESOLUTION;
        }
        let target_pixels = u64::from(width) * u64::from(height);
        for candidate in SUPPORTED_RESOLUTIONS {
            let (candidate_width, candidate_height) = Self::parse_dimensions(candidate);
            let pixels = u64::from(candidate_width) * u64::from(candidate_height);
            if pixels >= target_pixels {
                if candidate != spec {
                    trace!("[wp.upx8] [{}] 就近映射为站点档位 {}", spec, candidate);
                }
                return candidate;
            }
        }
        // 全部档位都小于目标面积，取站点最大档
        let max_resolution = SUPPORTED_RESOLUTIONS[SUPPORTED_RESOLUTIONS.len() - 1];
        trace!(
            "[wp.upx8] [{}] 超出站点最大档位，映射为 {}",
            spec,
            max_resolution
        );
        max_resolution
    }

    /// 从 `{W}x{H}` 解析宽高（解析失败记 0）
    fn parse_dimensions(spec: &str) -> (u32, u32) {
        let mut parts = spec.split('x');
        let width = parts.next().and_then(|v| v.trim().parse().ok()).unwrap_or(0);
        let height = parts.next().and_then(|v| v.trim().parse().ok()).unwrap_or(0);
        (width, height)
    }

    /// 取图片直链的文件名主干（作为壁纸 id，用于下载文件命名）
    fn file_stem(url: &str) -> String {
        let trimmed = url.split('?').next().unwrap_or(url);
        let last = trimmed.rsplit('/').next().unwrap_or("");
        match last.rsplit_once('.') {
            Some((stem, _)) => stem.to_string(),
            None => last.to_string(),
        }
    }

    /// 将接口返回的图片映射为图源无关的在线壁纸条目
    ///
    /// `fallback_category` 为请求使用的站点分类，用于数据源未标明来源时占位
    fn to_online_wallpaper(image: Upx8Image, fallback_category: &str) -> Option<OnlineWallpaper> {
        let id = Self::file_stem(&image.url);
        if id.is_empty() {
            error!("[wp.upx8] [{}] 图片直链缺少文件名，已跳过", image.url);
            return None;
        }

        let url = client::normalize_url(&image.url);
        let (width, height) = Self::parse_dimensions(&image.resolution);
        let resolution = if width == 0 || height == 0 {
            DEFAULT_RESOLUTION.to_string()
        } else {
            image.resolution.clone()
        };

        let category = if image.source.is_empty() {
            fallback_category.to_string()
        } else {
            image.source.clone()
        };

        Some(OnlineWallpaper {
            id,
            // 接口不提供详情页，用图片直链自身作为来源标识
            url: url.clone(),
            path: url.clone(),
            thumb_large: client::thumb_url(&url, client::THUMB_LARGE_WIDTH),
            thumb_original: url.clone(),
            thumb_small: client::thumb_url(&url, client::THUMB_WIDTH),
            width,
            height,
            ratio: helpers::simplified_ratio(width, height),
            resolution,
            // 接口不提供文件大小，按「未知」记 0
            file_size: 0,
            file_type: "image/jpeg".to_string(),
            category,
            purity: "sfw".to_string(),
            views: 0,
            favorites: 0,
            colors: Vec::new(),
            source: SourceKind::Upx8,
            image_handle: None,
        })
    }
}

impl WallpaperSource for Upx8Service {
    fn kind(&self) -> SourceKind {
        SourceKind::Upx8
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // 站点无排序/比例/颜色/时间范围/纯净度维度，这些参数直接忽略
            trace!(
                "[wp.upx8] [page{}] 忽略 sorting/ratios/color/source_color/time_range/purities/categories 参数",
                query.page
            );
            if query.page > 1 {
                trace!(
                    "[wp.upx8] [page{}] 接口无分页，页码仅用于触发重新随机取图",
                    query.page
                );
            }

            let api_query = client::Upx8Query {
                keyword: Self::keyword_param(query),
                category: query.site_category.clone(),
                resolution: Self::resolution_param(query).map(|value| value.to_string()),
                count: client::ITEMS_PER_PAGE,
            };

            let images = client::fetch_images(&self.client, &api_query, context).await?;
            let fallback_category = query.site_category.clone().unwrap_or_default();
            let wallpapers: Vec<OnlineWallpaper> = images
                .into_iter()
                .filter_map(|image| Self::to_online_wallpaper(image, &fallback_category))
                .collect();

            info!(
                "[wp.upx8] [page{}] 返回 {} 张壁纸",
                query.page,
                wallpapers.len()
            );

            Ok(SearchResult {
                wallpapers,
                // 接口无分页，单次请求即为全部结果
                is_last: true,
                total_pages: 0,
                current_page: query.page,
            })
        })
    }
}
