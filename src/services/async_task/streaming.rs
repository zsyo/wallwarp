// Copyright (C) 2026 zsyo - GNU GPL v3.0

use iced::widget::image::Handle;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tracing::{debug, error, info};

/// 异步加载在线壁纸图片函数（流式下载，支持取消）
pub async fn async_load_online_wallpaper_image_with_streaming(
    url: String,
    file_size: u64,
    cache_path: String,
    proxy: Option<String>,
    cancel_token: Arc<AtomicBool>,
) -> Result<Handle, Box<dyn Error + Send + Sync>> {
    debug!(
        "[模态窗口图片下载] [URL:{}] 开始流式下载，文件大小: {} bytes",
        url, file_size
    );

    // 步骤1: 计算缓存文件路径（复用 DownloadService 的哈希规则）
    let cache_file = PathBuf::from(
        crate::services::download::DownloadService::get_online_image_cache_final_path(
            &cache_path,
            &url,
            file_size,
        )
        .map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 获取缓存路径失败: {}", url, e);
            e
        })?,
    );

    // 步骤2: 检查缓存是否存在且大小匹配
    if cache_file.exists()
        && let Ok(metadata) = std::fs::metadata(&cache_file)
    {
        let cache_size = metadata.len();
        if cache_size == file_size {
            debug!(
                "[模态窗口图片下载] [URL:{}] 使用缓存: {}",
                url,
                cache_file.display()
            );
            return Ok(Handle::from_path(&cache_file));
        }
    }

    // 步骤3: 确保缓存目录存在
    if let Some(cache_dir_path) = cache_file.parent() {
        tokio::fs::create_dir_all(cache_dir_path)
            .await
            .map_err(|e| {
                error!("[模态窗口图片下载] [URL:{}] 创建缓存目录失败: {}", url, e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;
    }

    // 步骤4: 创建带代理和环境变量回退的优化 HTTP 客户端
    let client = crate::services::proxy::create_client_with_env_fallback(
        proxy,
        &url,
        "模态窗口图片下载",
        true, // 使用 info 级别
    );

    // 步骤5: 流式下载落盘（边读边写，整图不进内存），每完成 5% 上报一次进度
    let mut progress_sent = 0i32;
    let downloaded = crate::services::download::download_to_file_streaming(
        client,
        &url,
        &cache_file,
        Some(cancel_token),
        "模态窗口图片下载",
        |downloaded, content_length| {
            // 响应无 Content-Length 时以 API 提供的 file_size 为分母
            let total = if content_length > 0 {
                content_length
            } else {
                file_size
            };
            if total > 0 {
                let percent = (downloaded as f32 / total as f32 * 100.0) as i32;
                if percent >= progress_sent + 5 {
                    progress_sent = percent;
                    crate::services::send_modal_image_progress(downloaded, total);
                }
            }
        },
    )
    .await?;

    let total_size = if file_size > 0 { file_size } else { downloaded };

    // 验证文件完整性
    if let Ok(metadata) = tokio::fs::metadata(&cache_file).await {
        let actual_size = metadata.len();
        if total_size > 0 && actual_size != total_size {
            error!(
                "[模态窗口图片下载] [URL:{}] 文件大小不匹配：期望 {} bytes，实际 {} bytes",
                url, total_size, actual_size
            );
            return Err(Box::new(std::io::Error::other(format!(
                "文件大小不匹配：期望 {} bytes，实际 {} bytes",
                total_size, actual_size
            ))) as Box<dyn Error + Send + Sync>);
        }
    }

    info!(
        "[模态窗口图片下载] [URL:{}] 下载完成，文件大小: {} bytes",
        url, downloaded
    );

    // 返回图片Handle
    Ok(Handle::from_path(&cache_file))
}
