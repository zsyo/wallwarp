// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::request_context::RequestContext;
use crate::services::wallhaven::{ColorOption, OnlineWallpaper, Sorting, TimeRange, WallhavenService};
use iced::widget::image::Handle;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 异步加载在线壁纸函数
pub async fn async_load_online_wallpapers(
    categories: u32,
    sorting: Sorting,
    purities: u32,
    color: ColorOption,
    query: String,
    time_range: TimeRange,
    atleast: Option<String>,
    resolutions: Option<String>,
    ratios: Option<String>,
    page: usize,
    api_key: Option<String>,
    proxy: Option<String>,
    context: RequestContext,
) -> Result<(Vec<OnlineWallpaper>, bool, usize, usize), Box<dyn Error + Send + Sync>> {
    let service = WallhavenService::new(api_key, proxy);
    match service
        .search_wallpapers(
            page,
            categories,
            sorting,
            purities,
            color,
            &query,
            time_range,
            atleast.as_deref(),
            resolutions.as_deref(),
            ratios.as_deref(),
            &context,
        )
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error + Send + Sync>),
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
        return Err(
            Box::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "下载已取消"))
                as Box<dyn Error + Send + Sync>,
        );
    }

    // 使用DownloadService的智能缓存加载功能
    DownloadService::load_thumb_with_cache_with_cancel(url, file_size, cache_base_path, proxy, cancel_token).await
}
