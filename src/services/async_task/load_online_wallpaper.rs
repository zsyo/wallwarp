// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::request_context::RequestContext;
use crate::services::wallhaven::{
    ColorOption, OnlineWallpaper, SearchParams, Sorting, TimeRange, WallhavenService,
};
use iced::widget::image::Handle;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 在线壁纸搜索参数
pub struct OnlineSearchParams {
    pub categories: u32,
    pub sorting: Sorting,
    pub purities: u32,
    pub color: ColorOption,
    pub query: String,
    pub time_range: TimeRange,
    pub atleast: Option<String>,
    pub resolutions: Option<String>,
    pub ratios: Option<String>,
    pub page: usize,
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub proxy_enabled: bool,
    pub use_env_fallback: bool,
    pub context: RequestContext,
}

/// 异步加载在线壁纸函数
pub async fn async_load_online_wallpapers(
    params: OnlineSearchParams,
) -> Result<(Vec<OnlineWallpaper>, bool, usize, usize), Box<dyn Error + Send + Sync>> {
    let service = WallhavenService::new(
        params.api_key,
        params.proxy,
        params.proxy_enabled,
        params.use_env_fallback,
    );
    let search_params = SearchParams {
        page: params.page,
        categories: params.categories,
        sorting: params.sorting.value(),
        purities: params.purities,
        color: params.color.value(),
        query: &params.query,
        top_range: params.time_range.value(),
        atleast: params.atleast.as_deref(),
        resolutions: params.resolutions.as_deref(),
        ratios: params.ratios.as_deref(),
    };
    match service
        .search_wallpapers(&search_params, &params.context)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(std::io::Error::other(e)) as Box<dyn Error + Send + Sync>),
    }
}

/// 异步加载在线壁纸缩略图函数（带缓存）
pub async fn async_load_online_wallpaper_thumb_with_cache(
    url: String,
    file_size: u64,
    cache_base_path: String,
    proxy: Option<String>,
) -> Result<Handle, Box<dyn Error + Send + Sync>> {
    // 使用DownloadService的智能缓存加载功能
    DownloadService::load_thumb_with_cache(url, file_size, cache_base_path, proxy).await
}

/// 异步加载在线壁纸缩略图函数（带缓存和取消支持）
pub async fn async_load_online_wallpaper_thumb_with_cache_with_cancel(
    url: String,
    file_size: u64,
    cache_base_path: String,
    proxy: Option<String>,
    cancel_token: Arc<AtomicBool>,
) -> Result<Handle, Box<dyn Error + Send + Sync>> {
    // 在下载前检查取消状态
    if cancel_token.load(Ordering::Relaxed) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "下载已取消",
        )) as Box<dyn Error + Send + Sync>);
    }

    // 使用DownloadService的智能缓存加载功能
    DownloadService::load_thumb_with_cache_with_cancel(
        url,
        file_size,
        cache_base_path,
        proxy,
        cancel_token,
    )
    .await
}
