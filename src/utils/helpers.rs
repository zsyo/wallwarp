use std::path::Path;

pub fn is_valid_image_path(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() {
        return false;
    }
    
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();
        
    matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "bmp" | "gif" | "webp")
}

pub fn format_resolution(width: u32, height: u32) -> String {
    format!("{}x{}", width, height)
}