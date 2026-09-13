// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 异步函数用于打开图片选择对话框
///
/// 支持 jpg/jpeg/png/bmp/gif/webp（与本地壁纸库支持的格式一致），
/// 使用 AsyncFileDialog：对话框存续期间内部线程负责等待，
/// 不占用调用方的 tokio worker 线程
pub async fn select_image_async() -> String {
    if let Some(file) = rfd::AsyncFileDialog::new()
        .add_filter("Images", &["jpg", "jpeg", "png", "bmp", "gif", "webp"])
        .pick_file()
        .await
    {
        file.path().to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}
