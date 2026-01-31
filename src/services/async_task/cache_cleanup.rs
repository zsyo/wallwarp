// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::utils::config::Config;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tracing::{error, info, warn};

/// 异步清理缓存任务
///
/// # 功能说明
/// 1. 清理 thumbnail 目录中创建时间超过 7 天的文件
/// 2. 清理 auto_change 目录中创建时间超过 3 天的文件（但跳过当前正在使用的壁纸）
/// 3. 清理 online 目录中创建时间超过 7 天的文件以及文件结尾是 .download 的所有文件
/// 4. 清理 logs 目录中创建时间超过 3 天的文件
pub async fn async_cleanup_cache(config: Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("[缓存清理] 开始清理缓存");

    let cache_path = PathBuf::from(&config.data.cache_path);

    // 1. 清理 thumbnail 目录（超过 7 天）
    let thumbnail_dir = cache_path.join("thumbnail");
    if thumbnail_dir.exists() {
        let deleted = cleanup_directory_by_age(&thumbnail_dir, 7, None).await?;
        info!("[缓存清理] thumbnail 目录清理完成，删除了 {} 个文件", deleted);
    } else {
        info!("[缓存清理] thumbnail 目录不存在，跳过");
    }

    // 2. 清理 auto_change 目录（超过 3 天，排除当前使用的壁纸）
    let auto_change_dir = cache_path.join("auto_change");
    if auto_change_dir.exists() {
        // 获取当前正在使用的壁纸路径
        let current_wallpaper = get_current_wallpaper().await.ok();
        let deleted = cleanup_directory_by_age(&auto_change_dir, 3, current_wallpaper.as_deref()).await?;
        info!("[缓存清理] auto_change 目录清理完成，删除了 {} 个文件", deleted);
    } else {
        info!("[缓存清理] auto_change 目录不存在，跳过");
    }

    // 3. 清理 online 目录（超过 7 天以及所有 .download 文件）
    let online_dir = cache_path.join("online");
    if online_dir.exists() {
        let deleted_by_age = cleanup_directory_by_age(&online_dir, 7, None).await?;
        let deleted_downloads = cleanup_download_files(&online_dir).await?;
        info!(
            "[缓存清理] online 目录清理完成，删除了 {} 个过期文件和 {} 个下载中文件",
            deleted_by_age, deleted_downloads
        );
    } else {
        info!("[缓存清理] online 目录不存在，跳过");
    }

    // 4. 清理 logs 目录（超过 3 天）
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let logs_dir = current_dir.join("logs");
    if logs_dir.exists() {
        let deleted = cleanup_logs_directory(&logs_dir, 3).await?;
        info!("[缓存清理] logs 目录清理完成，删除了 {} 个文件", deleted);
    } else {
        info!("[缓存清理] logs 目录不存在，跳过");
    }

    info!("[缓存清理] 缓存清理任务完成");
    Ok(())
}

/// 根据文件年龄清理目录中的文件
///
/// # 参数
/// - dir: 要清理的目录路径
/// - max_age_days: 最大保留天数（超过此天数的文件将被删除）
/// - exclude_path: 要排除的文件路径（不会被删除），通常用于排除当前使用的壁纸
///
/// # 返回
/// 返回删除的文件数量
async fn cleanup_directory_by_age(
    dir: &Path,
    max_age_days: u64,
    exclude_path: Option<&str>,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut deleted_count = 0;
    let max_age_seconds = max_age_days * 24 * 60 * 60;

    // 读取目录中的所有文件
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("[缓存清理] 无法读取目录 {}: {}", dir.display(), e);
            return Ok(0);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("[缓存清理] 读取目录项失败: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // 只处理文件，跳过子目录
        if path.is_dir() {
            continue;
        }

        // 检查是否在排除列表中
        if let Some(exclude) = exclude_path {
            if path.to_string_lossy().to_lowercase() == exclude.to_lowercase() {
                info!("[缓存清理] 跳过当前使用的壁纸: {}", path.display());
                continue;
            }
        }

        // 获取文件创建时间（或修改时间）
        let file_age_seconds = match get_file_age_seconds(&path) {
            Ok(age) => age,
            Err(e) => {
                warn!("[缓存清理] 无法获取文件年龄 {}: {}", path.display(), e);
                continue;
            }
        };

        // 如果文件年龄超过最大保留天数，则删除
        if file_age_seconds > max_age_seconds {
            match fs::remove_file(&path) {
                Ok(_) => {
                    info!(
                        "[缓存清理] 删除过期文件: {} (年龄: {:.1} 天)",
                        path.display(),
                        file_age_seconds as f64 / (24.0 * 60.0 * 60.0)
                    );
                    deleted_count += 1;
                }
                Err(e) => {
                    error!("[缓存清理] 删除文件失败 {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(deleted_count)
}

/// 清理目录中所有 .download 结尾的文件
///
/// # 参数
/// - dir: 要清理的目录路径
///
/// # 返回
/// 返回删除的文件数量
async fn cleanup_download_files(dir: &Path) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut deleted_count = 0;

    // 读取目录中的所有文件
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("[缓存清理] 无法读取目录 {}: {}", dir.display(), e);
            return Ok(0);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("[缓存清理] 读取目录项失败: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // 只处理文件，跳过子目录
        if path.is_dir() {
            continue;
        }

        // 检查文件名是否以 .download 结尾
        if path
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("download"))
        {
            match fs::remove_file(&path) {
                Ok(_) => {
                    info!("[缓存清理] 删除下载中文件: {}", path.display());
                    deleted_count += 1;
                }
                Err(e) => {
                    error!("[缓存清理] 删除文件失败 {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(deleted_count)
}

/// 获取文件的年龄（秒数）
///
/// # 参数
/// - path: 文件路径
///
/// # 返回
/// 返回文件创建时间（或修改时间）距今的秒数
fn get_file_age_seconds(path: &Path) -> Result<u64, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;

    // 优先使用创建时间（Windows 系统），如果创建时间不可用则使用修改时间
    let file_time = metadata.created().unwrap_or_else(|_| metadata.modified().unwrap());

    let now = SystemTime::now();
    let duration = now.duration_since(file_time)?;

    Ok(duration.as_secs())
}

/// 获取当前正在使用的壁纸路径
///
/// # 返回
/// 返回当前壁纸的绝对路径，如果获取失败则返回 None
async fn get_current_wallpaper() -> Result<String, Box<dyn Error + Send + Sync>> {
    // 使用 tokio::task::spawn_blocking 在阻塞线程中执行
    tokio::task::spawn_blocking(|| wallpaper::get().map_err(|e| format!("获取当前壁纸失败: {}", e).into())).await?
}

/// 清理 logs 目录中的日志文件
///
/// # 参数
/// - dir: logs 目录路径
/// - max_age_days: 最大保留天数（超过此天数的日志文件将被删除）
///
/// # 返回
/// 返回删除的文件数量
///
/// # 注意事项
/// - 不会删除 latest.log（当前正在使用的日志文件）
/// - 只删除 .log 后缀的文件
async fn cleanup_logs_directory(dir: &Path, max_age_days: u64) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let mut deleted_count = 0;
    let max_age_seconds = max_age_days * 24 * 60 * 60;

    // 读取目录中的所有文件
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("[缓存清理] 无法读取 logs 目录 {}: {}", dir.display(), e);
            return Ok(0);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("[缓存清理] 读取目录项失败: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // 只处理文件，跳过子目录
        if path.is_dir() {
            continue;
        }

        // 获取文件名
        let file_name = match path.file_name() {
            Some(name) => name,
            None => continue,
        };

        // 跳过 latest.log（当前正在使用的日志文件）
        if file_name == "latest.log" {
            continue;
        }

        // 只处理 .log 后缀的文件
        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("log")) {
            // 获取文件年龄
            let file_age_seconds = match get_file_age_seconds(&path) {
                Ok(age) => age,
                Err(e) => {
                    warn!("[缓存清理] 无法获取日志文件年龄 {}: {}", path.display(), e);
                    continue;
                }
            };

            // 如果文件年龄超过最大保留天数，则删除
            if file_age_seconds > max_age_seconds {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        info!(
                            "[缓存清理] 删除过期日志文件: {} (年龄: {:.1} 天)",
                            path.display(),
                            file_age_seconds as f64 / (24.0 * 60.0 * 60.0)
                        );
                        deleted_count += 1;
                    }
                    Err(e) => {
                        error!("[缓存清理] 删除日志文件失败 {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    Ok(deleted_count)
}
