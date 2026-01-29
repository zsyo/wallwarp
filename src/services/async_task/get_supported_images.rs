// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::local::LocalWallpaperService;
use std::error::Error;
use tokio::task::spawn_blocking;

/// 异步获取支持的图片文件列表
pub async fn async_get_supported_images(data_path: String) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || LocalWallpaperService::get_supported_image_paths(&data_path))
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}
