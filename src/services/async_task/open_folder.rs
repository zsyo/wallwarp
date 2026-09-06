// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 异步函数用于打开目录选择对话框
///
/// 使用 AsyncFileDialog：对话框存续期间内部线程负责等待，
/// 不占用调用方的 tokio worker 线程
pub async fn select_folder_async() -> String {
    if let Some(folder) = rfd::AsyncFileDialog::new().pick_folder().await {
        folder.path().to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}
