// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::local::LocalWallpaperService;
use std::error::Error;
use tokio::task::spawn_blocking;

/// 异步加载壁纸路径列表函数
pub async fn async_load_wallpaper_paths(data_path: String) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    // 在这里调用同步的获取壁纸路径函数
    spawn_blocking(move || LocalWallpaperService::get_wallpaper_paths(&data_path))
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}
