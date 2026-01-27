/// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 生成下载文件名
pub fn generate_file_name(id: &str, file_type: &str) -> String {
    format!("wallhaven-{}.{}", id, file_type)
}
