// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 多显示器独立壁纸相关任务
//!
//! macOS 的 NSScreen/NSWorkspace 仅可在主线程访问，经 iced `window::run`
//! 派发到主线程执行；Windows/Linux 的平台调用无主线程要求，放阻塞线程池

use crate::platform;
use crate::services::local::LocalWallpaperService;
use crate::utils::config::WallpaperMode;
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
pub fn set_wallpaper_for_monitor_task(
    window_id: iced::window::Id,
    monitor_id: String,
    image_path: String,
    mode: WallpaperMode,
) -> Task<Result<(), String>> {
    #[cfg(target_os = "macos")]
    {
        iced::window::run(window_id, move |_mw| {
            LocalWallpaperService::set_wallpaper_for_monitor(&monitor_id, &image_path, mode)
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = window_id;
        Task::perform(
            async move {
                spawn_blocking(move || {
                    LocalWallpaperService::set_wallpaper_for_monitor(&monitor_id, &image_path, mode)
                })
                .await
                .unwrap_or_else(|e| Err(e.to_string()))
            },
            |result| result,
        )
    }
}
