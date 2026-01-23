use std::path::Path;

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
        let _ = std::process::Command::new("open")
            .args(["-R", &full_path])
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 上使用 xdg-open 打开文件所在目录
        if let Some(parent) = Path::new(&full_path).parent() {
            let _ = std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn();
        }
    }
}

pub fn is_valid_image_path(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() {
        return false;
    }

    let extension = path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase()).unwrap_or_default();

    matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp")
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
