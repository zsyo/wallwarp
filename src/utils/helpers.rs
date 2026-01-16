use std::path::Path;

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
