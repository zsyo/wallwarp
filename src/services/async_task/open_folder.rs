// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 异步函数用于打开目录选择对话框
pub async fn select_folder_async() -> String {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        path.to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}
