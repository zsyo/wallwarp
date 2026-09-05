// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 文件复制相关的异步操作
//!
//! 大文件(整张壁纸原图)的复制必须放在 spawn_blocking 中执行，
//! 避免阻塞 tokio runtime 线程造成 UI 卡顿

use crate::services::local::LocalWallpaperService;
use crate::utils::config::WallpaperMode;
use std::error::Error;
use std::path::Path;
use tokio::task::spawn_blocking;

/// 异步复制文件到目标路径
///
/// 目标路径的父目录不存在时自动创建
pub async fn async_copy_file(source_path: String, target_path: String) -> Result<(), String> {
    spawn_blocking(move || -> Result<(), String> {
        if let Some(parent) = Path::new(&target_path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
        std::fs::copy(&source_path, &target_path)
            .map(|_| ())
            .map_err(|e| format!("复制文件失败: {}", e))
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

/// 异步复制文件到目标路径后设置壁纸
///
/// 复制与系统壁纸调用都在 spawn_blocking 中执行；
/// 目标路径的父目录不存在时自动创建
pub async fn async_copy_and_set_wallpaper(
    source_path: String,
    target_path: String,
    mode: WallpaperMode,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(parent) = Path::new(&target_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(&source_path, &target_path)?;
        LocalWallpaperService::set_wallpaper(&target_path, mode)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}
