// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Bing 每日一图服务层
//!
//! 实现图源抽象的 [`WallpaperSource`] trait：`search()` 忽略筛选参数，
//! 按页码翻阅最近的每日精选图集（每页 8 张，翻到底返回空页结束）

use super::client::{self, BingImage};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, SourceFuture, SourceKind, WallpaperSource,
};
use tracing::{info, trace};

/// Bing 每日一图服务
pub struct BingService {
    client: reqwest::Client,
}

impl BingService {
    /// 创建新的 Bing 服务
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

    /// 将 Bing 图集条目映射为图源无关的在线壁纸条目
    ///
    /// 接口不提供文件大小，`file_size` 记 0（缓存/下载链路按"未知大小"
    /// 语义处理：只判存在不比对大小）
    fn to_online_wallpaper(image: BingImage) -> Option<OnlineWallpaper> {
        if image.hsh.is_empty() || image.urlbase.is_empty() {
            return None;
        }

        Some(OnlineWallpaper {
            id: image.hsh.clone(),
            // 详情页为 Bing 搜索页；无版权链接时回退图片直链
            url: format!("{}/th?id=OHR.{}", client::BASE_URL, image.hsh),
            path: client::full_image_url(&image.urlbase),
            thumb_large: client::thumb_image_url(&image.urlbase, 640, 360),
            thumb_original: client::full_image_url(&image.urlbase),
            thumb_small: client::thumb_image_url(&image.urlbase, 320, 180),
            width: 1920,
            height: 1080,
            resolution: "1920x1080".to_string(),
            ratio: "16:9".to_string(),
            file_size: 0,
            file_type: "image/jpeg".to_string(),
            category: "general".to_string(),
            purity: "sfw".to_string(),
            views: 0,
            favorites: 0,
            colors: Vec::new(),
            source: SourceKind::Bing,
            image_handle: None,
        })
    }
}

impl WallpaperSource for BingService {
    fn kind(&self) -> SourceKind {
        SourceKind::Bing
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // 每日图集无筛选维度，忽略 Sorting/颜色/时间范围/纯净度等参数
            trace!(
                "[Bing API] [page{}] 每日图集无筛选维度，忽略 sorting/color/time_range/purities/categories 等参数",
                query.page
            );
            let _ = query;

            let response = client::fetch_images_page(&self.client, query.page, context).await?;
            let wallpapers: Vec<OnlineWallpaper> = response
                .images
                .into_iter()
                .filter_map(Self::to_online_wallpaper)
                .collect();

            info!(
                "[Bing API] [page{}] 返回 {} 张每日壁纸",
                query.page,
                wallpapers.len()
            );

            // 图集不足一页说明已翻到档案底部
            let is_last = wallpapers.len() < client::IMAGES_PER_PAGE as usize;

            Ok(SearchResult {
                wallpapers,
                is_last,
                total_pages: 0,
                current_page: query.page,
            })
        })
    }
}
