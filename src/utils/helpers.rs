// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use std::path::Path;
use tracing::{error, info};

/// 在文件管理器中打开并选中指定文件
///
/// # 参数
/// * `path` - 文件路径（可以是相对路径或绝对路径）
///
/// # 平台支持
/// - Windows: 使用 `explorer /select,路径`
/// - macOS: 使用 `open -R 路径`
/// - Linux: 使用 `xdg-open` 打开文件所在目录
pub fn open_file_in_explorer(path: &str) {
    let full_path = if Path::new(path).is_absolute() {
        path.to_string()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(path)
            .to_string_lossy()
            .to_string()
    };

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer")
            .args(["/select,", &full_path])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").args(["-R", &full_path]).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 上使用 xdg-open 打开文件所在目录
        if let Some(parent) = Path::new(&full_path).parent() {
            let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
        }
    }
}

pub fn is_valid_image_path(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() {
        return false;
    }

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "bmp" | "webp")
}

pub fn format_resolution(width: u32, height: u32) -> String {
    format!("{}x{}", width, height)
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 获取绝对路径
pub fn get_absolute_path(path: &str) -> String {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let path_buf = std::path::PathBuf::from(path);

    if path_buf.is_absolute() {
        path.to_string()
    } else {
        current_dir.join(path_buf).to_string_lossy().to_string()
    }
}

/// 根据平台返回最通用的系统 UI 字体名称
pub fn get_system_ui_font() -> &'static str {
    if cfg!(target_os = "windows") {
        "Microsoft YaHei" // 微软雅黑
    } else if cfg!(target_os = "macos") {
        "Helvetica Neue" // 或使用 ".AppleSystemUIFont"
    } else {
        "Noto Sans CJK SC" // Linux 常用中文字体
    }
}

/// 检测运行环境
pub fn is_running_via_cargo() -> bool {
    // 只要是 cargo 启动的，这个环境变量一定存在
    std::env::var("CARGO").is_ok()
}

/// 确保目录存在，如果不存在则创建
///
/// # 参数
/// - `path`: 目录路径
/// - `dir_name`: 目录名称（用于日志记录）
///
/// # 行为
/// - 如果目录已存在，记录信息日志
/// - 如果目录不存在，创建目录并记录信息日志
/// - 如果创建失败，记录错误日志
pub fn ensure_directory_exists(path: &str, dir_name: &str) {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        if let Err(e) = std::fs::create_dir_all(dir_path) {
            error!("[{}] 创建目录失败: {}", dir_name, e);
        } else {
            info!("[{}] 目录已创建: {}", dir_name, path);
        }
    } else {
        info!("[{}] 目录已存在: {}", dir_name, path);
    }
}

/// 标准化路径，去除 Windows 扩展路径前缀
///
/// # 参数
/// - `path`: 原始路径
///
/// # 返回
/// 标准化后的路径（去除 `\\?\` 前缀）
///
/// # 说明
/// Windows API 有时会返回带有 `\\?\` 前缀的扩展长度路径，
/// 该函数去除此前缀以提高可读性。文件操作可以正常使用原始路径，
/// 但显示给用户时建议使用标准化后的路径。
pub fn normalize_path(path: &str) -> String {
    // Windows 扩展路径前缀: \\?\
    if path.starts_with("\\\\?\\") {
        path[4..].to_string()
    } else {
        path.to_string()
    }
}
