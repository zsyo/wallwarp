// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::random_online;
use crate::services::image_process;
use crate::services::local::LocalWallpaperService;
use crate::utils::config::{Config, WallpaperMode, WallpaperReadability};
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::spawn_blocking;
use tracing::{debug, info};

/// 异步设置壁纸函数
///
/// 启用可读性处理时，先在阻塞线程上生成处理副本，系统壁纸指向副本
pub async fn async_set_wallpaper(
    wallpaper_path: String,
    mode: WallpaperMode,
    readability: WallpaperReadability,
    cache_path: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || {
        let path =
            image_process::process_for_wallpaper(&wallpaper_path, readability, &cache_path)?;
        LocalWallpaperService::set_wallpaper(&path, mode)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}

/// 异步随机设置壁纸函数
pub async fn async_set_random_wallpaper(
    image_paths: Vec<String>,
    mode: WallpaperMode,
    readability: WallpaperReadability,
    cache_path: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || {
        LocalWallpaperService::set_random_wallpaper(&image_paths, mode, readability, &cache_path)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}

/// 异步随机设置在线壁纸函数（用于定时切换）
///
/// 流程分四步（子流程见 [`super::random_online`]）：
/// 1. 解析参数：按当前配置图源创建实例与搜索参数
/// 2. 翻页找图：最多请求 5 页直到拿到非空数据，随机选择一张
/// 3. 缓存判定：确保壁纸文件存在于 cache_path/auto_change（命中/移动/下载）
/// 4. 落盘设置：经 async_set_wallpaper 设置系统壁纸（含可读性处理副本）
///
/// # 注意事项
/// 定时切换的壁纸统一存储在 cache_path/auto_change 目录中，避免污染本地壁纸库
pub async fn async_set_random_online_wallpaper(
    config: Config,
    auto_change_running: Arc<AtomicBool>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // 尝试设置执行标志，防止定时任务并行执行
    if !auto_change_running
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        debug!("[定时切换] [在线] 已有任务在执行，跳过本次切换");
        return Err("已有任务在执行".into());
    }

    // 确保在函数结束时清除执行标志
    let _guard = scopeguard::guard((), |_| {
        auto_change_running.store(false, Ordering::SeqCst);
    });

    // 1. 解析参数：创建图源实例与搜索参数
    let (source_impl, base_query, proxy) = random_online::prepare_random_online_params(&config);
    let context = crate::services::request_context::RequestContext::new();

    // 2. 翻页找图并随机选择一张
    let selected =
        random_online::pick_random_online_wallpaper(source_impl.as_ref(), &base_query, &context)
            .await?;
    info!(
        "[定时切换] [在线] 已选择壁纸: ID={}, URL={}",
        selected.id, selected.path
    );

    // 3. 缓存判定与落盘：确保文件存在于 cache_path/auto_change
    let target_path =
        random_online::ensure_auto_change_file(&config.data.cache_path, &selected, proxy).await?;

    // 4. 落盘设置（系统调用，经 async_set_wallpaper 放入阻塞线程池执行；
    // 启用可读性处理时系统壁纸指向处理副本）
    async_set_wallpaper(
        target_path.to_string_lossy().to_string(),
        config.wallpaper.mode,
        config.wallpaper.readability,
        config.data.cache_path.clone(),
    )
    .await?;

    info!("[定时切换] [在线] 壁纸设置成功: {}", target_path.display());
    Ok(target_path.to_string_lossy().to_string())
}
