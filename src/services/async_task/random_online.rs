// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 定时切换在线模式的子流程
//!
//! [`async_set_random_online_wallpaper`](super::async_set_random_online_wallpaper)
//! 主流程按"解析参数 → 翻页找图 → 缓存判定 → 落盘设置"拆分，
//! 本文件承载翻页找图与缓存判定两个子步骤

use crate::services::download::DownloadService;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    SearchQuery, SiteCategory, SourceColor, SourceConfig, SourceKind, WallpaperSource,
    create_source,
};
use crate::services::wallhaven;
use crate::utils::config::Config;
use rand::prelude::IndexedRandom;
use std::error::Error;
use std::path::PathBuf;
use tokio::task::spawn_blocking;
use tracing::{debug, error};

/// 翻页查找的最大页数
pub(super) const MAX_RANDOM_PAGES: usize = 5;

/// 解析定时切换在线搜索参数并创建图源实例（图源取当前配置的选择）
pub(super) fn prepare_random_online_params(
    config: &Config,
) -> (Box<dyn WallpaperSource>, SearchQuery, Option<String>) {
    let categories = wallhaven::parse_category_bitmask(&config.wallhaven.category);
    let purities = wallhaven::parse_purity_bitmask(&config.wallhaven.purity);
    let color = wallhaven::ColorOption::parse(&config.wallhaven.color);
    let source_color = SourceColor::parse(&config.wallhaven.source_color);
    // 站点分类按当前图源输出各自的 key（无该维度的图源不携带参数）
    let current_source = SourceKind::parse(&config.global.online_source);
    let site_category = SiteCategory::parse(&config.wallhaven.site_category)
        .api_value(current_source)
        .map(|value| value.to_string());
    // 使用配置文件中的排序方式和时间范围（从 config.wallpaper 读取）
    let sorting = wallhaven::Sorting::parse(&config.wallpaper.auto_change_sorting);
    let time_range = wallhaven::TimeRange::parse(&config.wallpaper.auto_change_top_range);

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

    // key 按图源取用：定时切换的图源取当前配置选择，经 create_source 分发
    let api_key = SourceKind::parse(&config.global.online_source).api_key_from_config(config);

    let proxy = config.resolved_proxy();

    // 经图源抽象层创建图源实例（支持环境变量代理回退）
    let source_impl = create_source(
        SourceKind::parse(&config.global.online_source),
        SourceConfig {
            api_key,
            proxy: proxy.clone(),
            proxy_enabled: config.global.proxy_enabled,
            use_env_fallback: true,
        },
    );

    let query = SearchQuery {
        page: 1,
        categories,
        sorting,
        purities,
        color,
        source_color,
        site_category,
        query: config.wallpaper.auto_change_query.clone(),
        time_range,
        atleast,
        resolutions,
        ratios,
    };
    (source_impl, query, proxy)
}

/// 翻页找图：最多请求 [`MAX_RANDOM_PAGES`] 页直到拿到非空数据，随机返回一张
pub(super) async fn pick_random_online_wallpaper(
    source_impl: &dyn WallpaperSource,
    base_query: &SearchQuery,
    context: &RequestContext,
) -> Result<crate::services::source::OnlineWallpaper, Box<dyn Error + Send + Sync>> {
    let mut query = base_query.clone();
    let mut wallpapers = Vec::new();

    for page in 1..=MAX_RANDOM_PAGES {
        query.page = page;
        debug!(
            "[定时切换] [在线] 请求第 {} 页壁纸，关键词: {}",
            page,
            if query.query.is_empty() {
                "(无)"
            } else {
                &query.query
            }
        );

        match source_impl.search(&query, context).await {
            Ok(result) => {
                if result.wallpapers.is_empty() {
                    debug!("[定时切换] [在线] 第 {} 页返回空数据", page);
                    if result.is_last || result.current_page >= MAX_RANDOM_PAGES {
                        break;
                    }
                    continue;
                }

                debug!(
                    "[定时切换] [在线] 第 {} 页获取到 {} 张壁纸",
                    page,
                    result.wallpapers.len()
                );
                wallpapers = result.wallpapers;
                break;
            }
            Err(e) => {
                error!("[定时切换] [在线] 第 {} 页请求失败: {}", page, e);
                return Err(Box::new(std::io::Error::other(e)) as Box<dyn Error + Send + Sync>);
            }
        }
    }

    if wallpapers.is_empty() {
        error!(
            "[定时切换] [在线] 请求 {} 页后仍无可用壁纸",
            MAX_RANDOM_PAGES
        );
        return Err("未找到可用的在线壁纸".into());
    }

    // 随机选择一张壁纸
    wallpapers
        .choose(&mut rand::rng())
        .cloned()
        .ok_or("随机选择壁纸失败".into())
}

/// 缓存判定与落盘：确保壁纸文件存在于 cache_path/auto_change 并返回其路径
///
/// 文件检查/移动均为阻塞操作，统一放入 spawn_blocking 避免阻塞 tokio 运行时；
/// 需要下载时先落到 cache_path/online，再移动到 auto_change（原图非主动浏览，
/// 不在 online 目录中保留缓存）
pub(super) async fn ensure_auto_change_file(
    cache_path: &str,
    selected: &crate::services::source::OnlineWallpaper,
    proxy: Option<String>,
) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let file_name = selected.download_file_name();
    let auto_change_dir = PathBuf::from(cache_path).join("auto_change");
    let target_path = auto_change_dir.join(&file_name);
    let file_size = selected.file_size;
    let source_url = selected.path.clone();
    let selected_id = selected.id.clone();

    enum AutoChangeSource {
        /// 目标文件已存在于 auto_change 且大小匹配，可直接设置壁纸
        Ready,
        /// 已从 online 缓存移动到 auto_change，可直接设置壁纸
        Moved,
        /// 需要先下载原图到 online 缓存
        NeedDownload,
    }

    let state = {
        let target_path = target_path.clone();
        let cache_path = cache_path.to_string();
        let auto_change_dir = auto_change_dir.clone();
        spawn_blocking(move || {
            // 1. 检查目标文件是否已存在于 cache_path/auto_change 中
            if let Ok(metadata) = std::fs::metadata(&target_path)
                && DownloadService::file_size_matches(metadata.len(), file_size)
            {
                debug!(
                    "[定时切换] [在线] 文件已存在于auto_change目录，直接设置: {}",
                    target_path.display()
                );
                return AutoChangeSource::Ready;
            }

            // 2. 检查缓存文件是否存在且大小匹配（cache_path/online 目录）
            if let Ok(cache_file_path) = DownloadService::get_online_image_cache_final_path(
                &cache_path,
                &source_url,
                file_size,
            ) {
                let cache_file_path_obj = PathBuf::from(&cache_file_path);
                if let Ok(metadata) = std::fs::metadata(&cache_file_path_obj)
                    && DownloadService::file_size_matches(metadata.len(), file_size)
                {
                    // 缓存文件存在且大小匹配，移动到 cache_path/auto_change
                    // 因为这是原图且非主动浏览，不需要在 online 目录中保存对应的缓存
                    debug!(
                        "[定时切换] [在线] 从online缓存移动到auto_change目录: {} -> {}",
                        cache_file_path_obj.display(),
                        target_path.display()
                    );
                    let _ = std::fs::create_dir_all(&auto_change_dir);
                    match std::fs::rename(&cache_file_path_obj, &target_path) {
                        Ok(_) => return AutoChangeSource::Moved,
                        Err(e) => {
                            error!(
                                "[定时切换] [在线] [ID:{}] 从缓存移动失败: {}",
                                selected_id, e
                            );
                            // 移动失败，继续走下载流程
                        }
                    }
                }
            }

            // 3. 文件不存在，需要先下载
            AutoChangeSource::NeedDownload
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
    };

    if matches!(state, AutoChangeSource::NeedDownload) {
        // 3. 文件不存在，下载到 cache_path/online
        let cache_file_path = DownloadService::get_online_image_cache_path(
            cache_path,
            &selected.path,
            selected.file_size,
        )?;
        let cache_file_path_obj = PathBuf::from(&cache_file_path);
        debug!(
            "[定时切换] [在线] 缓存不存在，开始下载到online缓存: {}",
            cache_file_path_obj.display()
        );
        DownloadService::download_image_to_cache(&selected.path, &cache_file_path, proxy).await?;

        // 下载完成后，移动到 cache_path/auto_change
        // 因为这是原图且非主动浏览，不需要在 online 目录中保存对应的缓存
        debug!(
            "[定时切换] [在线] 下载完成，移动到auto_change目录: {} -> {}",
            cache_file_path_obj.display(),
            target_path.display()
        );
        let rename_src = cache_file_path_obj.clone();
        let rename_target = target_path.clone();
        spawn_blocking(move || {
            let _ = std::fs::create_dir_all(&auto_change_dir);
            std::fs::rename(&rename_src, &rename_target)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)??;
    }

    Ok(target_path)
}
