// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::local::{LocalWallpaperService, Wallpaper};
use std::error::Error;
use std::path::{Path, PathBuf};
use tokio::task::spawn_blocking;

/// 异步加载单个壁纸函数（带降级处理，即使图片加载失败也能获取文件大小）
pub async fn async_load_single_wallpaper_with_fallback(
    wallpaper_path: String,
    cache_path: String,
) -> Result<Wallpaper, Box<dyn Error + Send + Sync>> {
    let full_cache_path = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(&cache_path);

    // 使用spawn_blocking在阻塞线程池中运行
    spawn_blocking(move || {
        // 先获取文件大小（这个操作通常不会失败）
        let file_size = std::fs::metadata(&wallpaper_path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);

        // 尝试加载图片
        let result = (|| -> Result<Wallpaper, Box<dyn Error + Send + Sync>> {
            let thumbnail_path = LocalWallpaperService::generate_thumbnail_for_path(
                &wallpaper_path,
                &full_cache_path.to_string_lossy(),
            )?;

            // 获取图片尺寸
            let (width, height) = image::image_dimensions(&wallpaper_path).unwrap_or((0, 0));

            Ok(Wallpaper::with_thumbnail(
                wallpaper_path.clone(),
                Path::new(&wallpaper_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                thumbnail_path,
                file_size,
                width,
                height,
            ))
        })();

        match result {
            Ok(wallpaper) => Ok(wallpaper),
            Err(_) => {
                // 如果加载失败，返回一个带有文件大小的失败状态
                Ok(Wallpaper::new(
                    wallpaper_path.clone(),
                    "加载失败".to_string(),
                    file_size,
                    0,
                    0,
                ))
            }
        }
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}
