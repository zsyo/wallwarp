// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 开机自启动：随平台拆分的实现
//!
//! - Windows: `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` 注册表值
//! - macOS:   `~/Library/LaunchAgents/top.aico.wallwarp.plist`（RunAtLoad）
//! - Linux:   `~/.config/autostart/wallwarp.desktop`（XDG autostart）
//!
//! 三平台启动参数统一为 `<可执行文件> --hidden`，`main.rs` 的解析不变。

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use linux as imp;
#[cfg(target_os = "macos")]
use macos as imp;
#[cfg(target_os = "windows")]
use windows as imp;

/// 开机自启动是否已开启
pub fn is_auto_startup_enabled() -> bool {
    imp::is_enabled().unwrap_or(false)
}

/// 开启/关闭开机自启动
pub fn set_auto_startup(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    imp::set(enabled)
}

/// 当前实例的可执行路径
///
/// Linux AppImage 场景下 `current_exe()` 指向临时挂载点
/// `/tmp/.mount_XXXX/...`（每次启动都会变化），必须优先取 `APPIMAGE`
/// 环境变量指向的真实文件路径
pub(crate) fn executable_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    if let Some(appimage) = std::env::var_os("APPIMAGE") {
        return Ok(std::path::PathBuf::from(appimage));
    }
    std::env::current_exe().map_err(Into::into)
}

/// 未安装直接运行时注册的路径是易失的（重启后失效），仅记日志提示
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) fn warn_if_volatile(path: &std::path::Path) {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let text = path.to_string_lossy();
        #[cfg(target_os = "macos")]
        if text.starts_with("/Volumes/") || text.contains("/.mount_") {
            tracing::warn!(
                "[自启动] [路径] 应用未安装（{}），注册的自启动路径重启后可能失效",
                text
            );
        }
        #[cfg(target_os = "linux")]
        if text.contains("/.mount_") || text.starts_with("/tmp/") {
            tracing::warn!(
                "[自启动] [路径] 应用未安装（{}），注册的自启动路径重启后可能失效",
                text
            );
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let _ = path;
}
