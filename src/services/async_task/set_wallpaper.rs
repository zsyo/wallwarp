// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::local::LocalWallpaperService;
use crate::services::request_context::RequestContext;
use crate::services::wallhaven;
use crate::utils::config::{Config, WallpaperMode};
use rand::prelude::IndexedRandom;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::spawn_blocking;
use tracing::{error, info};

/// 异步设置壁纸函数
pub async fn async_set_wallpaper(
    wallpaper_path: String,
    mode: WallpaperMode,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || LocalWallpaperService::set_wallpaper(&wallpaper_path, mode))
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}

/// 异步随机设置壁纸函数
pub async fn async_set_random_wallpaper(
    image_paths: Vec<String>,
    mode: WallpaperMode,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    spawn_blocking(move || LocalWallpaperService::set_random_wallpaper(&image_paths, mode))
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
}

/// 异步随机设置在线壁纸函数（用于定时切换）
///
/// # 功能说明
/// 1. 从Wallhaven API获取壁纸列表
/// 2. 如果返回data为空数组则继续请求下一页，最多请求5页
/// 3. 直到返回的data不是空数组或者current_page=last_page
/// 4. 从返回的列表中随机选择一张图片
/// 5. 按照在线壁纸列表项的设置壁纸逻辑来设置壁纸：
///    - 先判断壁纸是否在config.data.cache_path/auto_change中，如果有则直接设置壁纸
///    - 否则判断壁纸是否在config.data.cache_path/online中，如果有则将该缓存图复制到config.data.cache_path/auto_change中
///    - 否则下载壁纸到缓存，然后复制到cache_path/auto_change中并设置壁纸
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
        info!("[定时切换] [在线] 已有任务在执行，跳过本次切换");
        return Err("已有任务在执行".into());
    }

    // 确保在函数结束时清除执行标志
    let _guard = scopeguard::guard((), |_| {
        auto_change_running.store(false, Ordering::SeqCst);
    });

    // 解析配置参数
    let categories = wallhaven::parse_category_bitmask(&config.wallhaven.category);
    // let sorting = wallhaven::parse_sorting(&config.wallhaven.sorting);
    let sorting = crate::services::wallhaven::Sorting::Random; // 将排序方式固定为随机
    let purities = wallhaven::parse_purity_bitmask(&config.wallhaven.purity);
    let color = wallhaven::parse_color(&config.wallhaven.color);
    let time_range = wallhaven::parse_time_range(&config.wallhaven.top_range);

    let atleast = if config.wallhaven.atleast_resolution.is_empty() {
        None
    } else {
        Some(config.wallhaven.atleast_resolution.clone())
    };

    let resolutions = if config.wallhaven.resolutions.is_empty() {
        None
    } else {
        Some(config.wallhaven.resolutions.clone())
    };

    let ratios = if config.wallhaven.ratios.is_empty() {
        None
    } else {
        Some(config.wallhaven.ratios.clone())
    };

    let api_key = if config.wallhaven.api_key.is_empty() {
        None
    } else {
        Some(config.wallhaven.api_key.clone())
    };

    let proxy = if config.global.proxy_enabled && !config.global.proxy.is_empty() {
                Some(config.global.proxy.clone())
            } else {
                None
            };
    // 创建请求上下文
    let context = RequestContext::new();

    // 创建Wallhaven服务
    let service = wallhaven::WallhavenService::new(api_key.clone(), proxy.clone());

    // 获取搜索关键词
    let query = config.wallpaper.auto_change_query.clone();

    // 最多请求5页
    let max_pages = 5;
    let mut wallpapers = Vec::new();

    for page in 1..=max_pages {
        info!(
            "[定时切换] [在线] 请求第 {} 页壁纸，关键词: {}",
            page,
            if query.is_empty() { "(无)" } else { &query }
        );

        match service
            .search_wallpapers(
                page,
                categories,
                sorting,
                purities,
                color,
                &query, // 使用配置中的关键词
                time_range,
                atleast.as_deref(),
                resolutions.as_deref(),
                ratios.as_deref(),
                &context,
            )
            .await
        {
            Ok((data, is_last_page, _total_pages, current_page)) => {
                if data.is_empty() {
                    info!("[定时切换] [在线] 第 {} 页返回空数据", page);
                    if is_last_page || current_page >= max_pages {
                        break;
                    }
                    continue;
                }

                info!("[定时切换] [在线] 第 {} 页获取到 {} 张壁纸", page, data.len());
                wallpapers = data;
                break;
            }
            Err(e) => {
                error!("[定时切换] [在线] 第 {} 页请求失败: {}", page, e);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn Error + Send + Sync>);
            }
        }
    }

    if wallpapers.is_empty() {
        error!("[定时切换] [在线] 请求 {} 页后仍无可用壁纸", max_pages);
        return Err("未找到可用的在线壁纸".into());
    }

    // 随机选择一张壁纸
    let selected = wallpapers.choose(&mut rand::rng()).ok_or("随机选择壁纸失败")?;

    info!(
        "[定时切换] [在线] 已选择壁纸: ID={}, URL={}",
        selected.id, selected.path
    );

    // 生成目标文件路径（使用原文件名，存储在 cache_path/auto_change 目录中）
    let file_name = wallhaven::generate_file_name(&selected.id, selected.file_type.split('/').last().unwrap_or("jpg"));
    let cache_path = config.data.cache_path.clone();
    let auto_change_dir = PathBuf::from(&cache_path).join("auto_change");
    let target_path = auto_change_dir.join(&file_name);

    // 1. 检查目标文件是否已存在于 cache_path/auto_change 中
    if let Ok(metadata) = std::fs::metadata(&target_path) {
        let actual_size = metadata.len();
        if actual_size == selected.file_size {
            // 文件已存在且大小匹配，直接设置壁纸
            info!(
                "[定时切换] [在线] 文件已存在于auto_change目录，直接设置: {}",
                target_path.display()
            );
            let wallpaper_mode = config.wallpaper.mode;
            LocalWallpaperService::set_wallpaper(&target_path.to_string_lossy().to_string(), wallpaper_mode)?;
            return Ok(target_path.to_string_lossy().to_string());
        }
    }

    // 2. 检查缓存文件是否存在且大小匹配（cache_path/online 目录）
    if let Ok(cache_file_path) =
        DownloadService::get_online_image_cache_final_path(&cache_path, &selected.path, selected.file_size)
    {
        let cache_file_path_obj = PathBuf::from(&cache_file_path);
        if let Ok(metadata) = std::fs::metadata(&cache_file_path_obj) {
            let cache_size = metadata.len();
            if cache_size == selected.file_size {
                // 缓存文件存在且大小匹配，移动到 cache_path/auto_change
                // 因为这是原图且非主动浏览，不需要在 online 目录中保存对应的缓存
                info!(
                    "[定时切换] [在线] 从online缓存移动到auto_change目录: {} -> {}",
                    cache_file_path_obj.display(),
                    target_path.display()
                );
                let _ = std::fs::create_dir_all(&auto_change_dir);
                match std::fs::rename(&cache_file_path_obj, &target_path) {
                    Ok(_) => {
                        // 移动成功，设置壁纸
                        let wallpaper_mode = config.wallpaper.mode;
                        LocalWallpaperService::set_wallpaper(
                            &target_path.to_string_lossy().to_string(),
                            wallpaper_mode,
                        )?;
                        return Ok(target_path.to_string_lossy().to_string());
                    }
                    Err(e) => {
                        error!("[定时切换] [在线] [ID:{}] 从缓存移动失败: {}", selected.id, e);
                        // 移动失败，继续走下载流程
                    }
                }
            }
        }
    }

    // 3. 文件不存在，下载到 cache_path/online
    let cache_file_path =
        DownloadService::get_online_image_cache_path(&cache_path, &selected.path, selected.file_size)?;
    let cache_file_path_obj = PathBuf::from(&cache_file_path);
    info!(
        "[定时切换] [在线] 缓存不存在，开始下载到online缓存: {}",
        cache_file_path_obj.display()
    );
    DownloadService::download_thumb_to_cache(&selected.path, &cache_file_path, proxy).await?;

    // 下载完成后，移动到 cache_path/auto_change
    // 因为这是原图且非主动浏览，不需要在 online 目录中保存对应的缓存
    info!(
        "[定时切换] [在线] 下载完成，移动到auto_change目录: {} -> {}",
        cache_file_path_obj.display(),
        target_path.display()
    );
    let _ = std::fs::create_dir_all(&auto_change_dir);
    std::fs::rename(&cache_file_path_obj, &target_path)?;

    // 设置壁纸
    let wallpaper_mode = config.wallpaper.mode;
    LocalWallpaperService::set_wallpaper(&target_path.to_string_lossy().to_string(), wallpaper_mode)?;

    info!("[定时切换] [在线] 壁纸设置成功: {}", target_path.display());
    Ok(target_path.to_string_lossy().to_string())
}
