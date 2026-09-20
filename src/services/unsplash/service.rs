// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Unsplash 服务层
//!
//! 实现图源抽象的 [`WallpaperSource`] trait：把统一筛选参数映射为 Unsplash
//! 接口参数（接口仅支持关键词/方向/排序，其余维度忽略），并把检索结果映射
//! 为图源无关的 [`OnlineWallpaper`]

use super::client::{self, UnsplashPhoto};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, Sorting, SourceFuture, SourceKind, WallpaperSource,
};
use crate::utils::helpers::{format_resolution, simplified_ratio};
use tracing::{error, info, trace};

/// Unsplash 图源服务
pub struct UnsplashService {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl UnsplashService {
    /// 创建新的 Unsplash 服务
    ///
    /// # 参数
    /// - `api_key`: Unsplash API 密钥（未配置时检索直接返回错误）
    /// - `proxy`: 可选的代理 URL
    /// - `proxy_enabled`: 代理是否启用
    /// - `use_env_fallback`: 是否使用环境变量作为回退
    pub fn new(
        api_key: Option<String>,
        proxy: Option<String>,
        proxy_enabled: bool,
        use_env_fallback: bool,
    ) -> Self {
        Self {
            client: proxy::create_proxy_client(proxy, proxy_enabled, use_env_fallback),
            api_key,
        }
    }

    /// 统一排序到 Unsplash order_by 参数的降级映射
    ///
    /// Unsplash 仅支持 relevant/latest：Relevance/Hot->relevant；
    /// 其余（DateAdded/Random/Views/Favorites/TopList）无对应能力，退化 latest
    fn map_order(sorting: Sorting) -> &'static str {
        match sorting {
            Sorting::Relevance | Sorting::Hot => "relevant",
            _ => "latest",
        }
    }

    /// 统一比例参数映射方向（Unsplash 支持 landscape/portrait/squarish）
    ///
    /// ratios 为逗号分隔串（landscape/portrait/具体比例值）；具体比例值
    /// 在 Unsplash 侧忽略，只把 landscape/portrait 归并为方向
    fn map_orientation(ratios: &Option<String>) -> Option<&'static str> {
        let ratios = ratios.as_ref()?;
        let has_landscape = ratios.split(',').any(|r| r.trim() == "landscape");
        let has_portrait = ratios.split(',').any(|r| r.trim() == "portrait");
        match (has_landscape, has_portrait) {
            (true, false) => Some("landscape"),
            (false, true) => Some("portrait"),
            // 都选/都未选/仅具体比例均不限制方向
            _ => None,
        }
    }

    /// 将 Unsplash 条目映射为图源无关的在线壁纸条目
    ///
    /// 接口不返回文件字节数与浏览量：file_size 记 0（下载链路按"未知大小"
    /// 语义处理，只判存在不比对大小），views 记 0；
    /// ratio 由宽高约分生成，colors 取主色单值
    fn to_online_wallpaper(photo: UnsplashPhoto) -> Option<OnlineWallpaper> {
        // 原图缺失或尺寸无效时跳过
        if photo.urls.raw.is_empty() || photo.width == 0 || photo.height == 0 {
            return None;
        }

        let colors = if photo.color.is_empty() {
            Vec::new()
        } else {
            vec![photo.color]
        };

        Some(OnlineWallpaper {
            id: photo.id,
            url: photo.links.html,
            path: photo.urls.raw.clone(),
            thumb_large: photo.urls.regular,
            thumb_original: photo.urls.raw,
            thumb_small: photo.urls.thumb,
            width: photo.width,
            height: photo.height,
            resolution: format_resolution(photo.width, photo.height),
            ratio: simplified_ratio(photo.width, photo.height),
            file_size: 0,
            // Unsplash 原图统一为 jpg（raw 直链不带可靠扩展名）
            file_type: "image/jpeg".to_string(),
            category: "general".to_string(),
            purity: "sfw".to_string(),
            views: 0,
            favorites: photo.likes as u32,
            colors,
            source: SourceKind::Unsplash,
            image_handle: None,
        })
    }
}

impl WallpaperSource for UnsplashService {
    fn kind(&self) -> SourceKind {
        SourceKind::Unsplash
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // Unsplash 所有检索都需 API Key，未配置时直接报错
            let Some(key) = self.api_key.as_deref() else {
                error!(
                    "[Unsplash API] [page{}] 未配置 API Key，跳过检索",
                    query.page
                );
                return Err("未配置 Unsplash API Key，请在设置页填写".to_string());
            };

            let params = client::SearchParams {
                key,
                query: &query.query,
                page: query.page,
                orientation: Self::map_orientation(&query.ratios),
                order_by: Self::map_order(query.sorting),
            };

            trace!(
                "[Unsplash API] [page{}] 筛选映射: sorting={}->order_by={}, ratios={:?}->orientation={:?}",
                query.page,
                query.sorting.value(),
                Self::map_order(query.sorting),
                query.ratios,
                Self::map_orientation(&query.ratios),
            );

            let response = client::search(&self.client, params, context).await?;

            let wallpapers: Vec<OnlineWallpaper> = response
                .results
                .into_iter()
                .filter_map(Self::to_online_wallpaper)
                .collect();

            info!(
                "[Unsplash API] [page{}] 关键词: {} 返回 {} 张壁纸",
                query.page,
                if query.query.is_empty() {
                    "(无)"
                } else {
                    &query.query
                },
                wallpapers.len()
            );

            // 空页或已到总页数即视为末页（total_pages 为 0 时不作为判据）
            let is_last = wallpapers.is_empty()
                || (response.total_pages > 0 && query.page as u64 >= response.total_pages);

            Ok(SearchResult {
                wallpapers,
                is_last,
                total_pages: response.total_pages as usize,
                current_page: query.page,
            })
        })
    }
}
