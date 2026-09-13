// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    self, ColorOption, OnlineWallpaper, SearchQuery, Sorting, SourceConfig, SourceKind, TimeRange,
};
use iced::widget::image::Handle;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 在线壁纸搜索参数（UI 层载荷：筛选参数 + 图源标识 + 网络配置）
pub struct OnlineSearchParams {
    /// 目标图源（当前仅 Wallhaven，后续新增图源时由 UI 选择）
    pub source: SourceKind,
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
///
/// 经图源抽象层（services::source）分发到目标图源实现，不绑定具体图源
pub async fn async_load_online_wallpapers(
    params: OnlineSearchParams,
) -> Result<(Vec<OnlineWallpaper>, bool, usize, usize), Box<dyn Error + Send + Sync>> {
    let source_impl = source::create_source(
        params.source,
        SourceConfig {
            api_key: params.api_key,
            proxy: params.proxy,
            proxy_enabled: params.proxy_enabled,
            use_env_fallback: params.use_env_fallback,
        },
    );
    let query = SearchQuery {
        categories: params.categories,
        sorting: params.sorting,
        purities: params.purities,
        color: params.color,
        query: params.query,
        time_range: params.time_range,
        atleast: params.atleast,
        resolutions: params.resolutions,
        ratios: params.ratios,
        page: params.page,
    };
    match source_impl.search(&query, &params.context).await {
        Ok(result) => Ok((
            result.wallpapers,
            result.is_last,
            result.total_pages,
            result.current_page,
        )),
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
            crate::services::download::DOWNLOAD_CANCELLED,
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
