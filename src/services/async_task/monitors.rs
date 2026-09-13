// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 多显示器独立壁纸相关任务
//!
//! macOS 的 NSScreen/NSWorkspace 仅可在主线程访问，经 iced `window::run`
//! 派发到主线程执行；Windows/Linux 的平台调用无主线程要求，放阻塞线程池

use crate::platform;
use crate::services::image_process;
use crate::services::local::LocalWallpaperService;
use crate::utils::config::{WallpaperMode, WallpaperReadability};
use iced::Task;
use tokio::task::spawn_blocking;

/// 枚举系统所有显示器
///
/// # 参数
/// - `window_id`: 主窗口 Id（macOS 主线程派发所需）
pub fn enumerate_monitors_task(
    window_id: iced::window::Id,
) -> Task<Result<Vec<platform::MonitorInfo>, String>> {
    #[cfg(target_os = "macos")]
    {
        iced::window::run(window_id, |_mw| platform::enumerate_monitors())
            .map(|monitors| Ok(monitors))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = window_id;
        Task::perform(
            async move {
                spawn_blocking(platform::enumerate_monitors)
                    .await
                    .map_err(|e| e.to_string())
            },
            |result| result,
        )
    }
}

/// 为指定显示器设置壁纸
///
/// # 参数
/// - `window_id`: 主窗口 Id（macOS 主线程派发所需）
/// - `monitor_id`: 目标显示器标识（来自 [`platform::enumerate_monitors`]）
/// - `image_path`: 壁纸图片路径
/// - `mode`: 铺满模式
/// - `readability`: 可读性处理选项（处理在阻塞线程池执行，系统壁纸指向副本）
/// - `cache_path`: 缓存根目录（处理副本写入 `<cache_path>/processed/`）
pub fn set_wallpaper_for_monitor_task(
    window_id: iced::window::Id,
    monitor_id: String,
    image_path: String,
    mode: WallpaperMode,
    readability: WallpaperReadability,
    cache_path: String,
) -> Task<Result<(), String>> {
    // 先在阻塞线程池生成处理副本（macOS 也不占用主线程），再执行系统壁纸调用
    let process_task = Task::perform(
        async move {
            spawn_blocking(move || {
                image_process::process_for_wallpaper(&image_path, readability, &cache_path)
                    .map_err(|e| e.to_string())
            })
            .await
            .map_err(|e| e.to_string())?
        },
        |result| result,
    );

    #[cfg(target_os = "macos")]
    {
        process_task.then(move |processed| {
            iced::window::run(window_id, move |_mw| match processed {
                Ok(path) => {
                    LocalWallpaperService::set_wallpaper_for_monitor(&monitor_id, &path, mode)
                }
                Err(e) => Err(e),
            })
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = window_id;
        process_task.then(move |processed| {
            let monitor_id = monitor_id.clone();
            Task::perform(
                async move {
                    spawn_blocking(move || match processed {
                        Ok(path) => {
                            LocalWallpaperService::set_wallpaper_for_monitor(&monitor_id, &path, mode)
                        }
                        Err(e) => Err(e),
                    })
                    .await
                    .unwrap_or_else(|e| Err(e.to_string()))
                },
                |result| result,
            )
        })
    }
}
