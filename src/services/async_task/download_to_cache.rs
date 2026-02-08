// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use iced::futures::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tracing::{error, info};

/// 辅助函数：下载图片到缓存目录
pub(super) async fn download_to_cache(
    url: &str,
    cache_path: &str,
    proxy: Option<String>,
    task_id: usize,
    cancel_token: Arc<AtomicBool>,
    downloaded_size: u64,
) -> Result<u64, String> {
    // 使用统一的代理客户端创建逻辑（支持环境变量回退）
    let client = crate::services::proxy::create_client_with_env_fallback(
        proxy,
        url,
        &format!("下载任务 [ID:{}]", task_id),
        true, // 使用 info 级别
    );

    // 发送请求（支持断点续传）
    let response = if downloaded_size > 0 {
        // 断点续传：使用 Range 请求头
        let range_header = format!("bytes={}-", downloaded_size);
        info!("[下载任务] [ID:{}] 断点续传：Range = {}", task_id, range_header);
        let resp = client
            .get(url)
            .header("Range", range_header)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        resp
    } else {
        // 新下载
        info!("[下载任务] [ID:{}] 新下载：从头开始", task_id);
        client.get(url).send().await.map_err(|e| format!("请求失败: {}", e))?
    };

    // 检查响应状态
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(format!("HTTP错误: {}", response.status()));
    }

    // 获取文件大小
    let total_size = if let Some(content_length) = response.content_length() {
        if downloaded_size > 0 {
            // 断点续传：总大小 = 已下载 + 剩余部分
            downloaded_size + content_length
        } else {
            content_length
        }
    } else {
        0
    };

    // 使用流式下载，分块读取并批量写入文件
    let mut stream = response.bytes_stream();

    // 打开文件并移动文件指针到安全偏移量的位置
    let cache_path_buf = PathBuf::from(cache_path);
    let mut file = if downloaded_size > 0 {
        // 断点续传：检查文件是否存在
        if !cache_path_buf.exists() {
            return Err(format!("文件不存在，无法断点续传: {}", cache_path));
        }

        // 打开文件
        let mut f = tokio::fs::OpenOptions::new()
            .write(true)
            .open(&cache_path_buf)
            .await
            .map_err(|e| format!("打开文件失败: {}", e))?;

        // 将文件指针移动到安全偏移量的位置
        f.seek(std::io::SeekFrom::Start(downloaded_size as u64))
            .await
            .map_err(|e| format!("移动文件指针失败: {}", e))?;

        info!(
            "[下载任务] [ID:{}] 断点续传：文件指针移动到位置 {} bytes",
            task_id, downloaded_size
        );

        f
    } else {
        // 创建新文件
        tokio::fs::File::create(&cache_path_buf)
            .await
            .map_err(|e| format!("创建文件失败: {}", e))?
    };

    let mut downloaded: u64 = downloaded_size;
    let mut progress_sent = 0i32;
    let start_time = std::time::Instant::now();
    let mut buffer = Vec::with_capacity(64 * 1024); // 64KB 缓冲区
    let mut last_log_time = start_time;

    // 立即发送一次进度更新，确保 total_size 被正确设置
    if total_size > 0 {
        let speed = 0u64;
        crate::services::send_download_progress(task_id, downloaded, total_size, speed);
    }

    while let Some(chunk_result) = stream.next().await {
        // 检查是否被取消
        if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
            info!("[下载任务] [ID:{}] 下载被取消", task_id);
            return Err("下载已取消".to_string());
        }

        let chunk = chunk_result.map_err(|e| format!("读取数据流失败: {}", e))?;

        // 将数据添加到缓冲区
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;

        // 当缓冲区达到64KB时，批量写入文件
        if buffer.len() >= 64 * 1024 {
            file.write_all(&buffer)
                .await
                .map_err(|e| format!("写入文件失败: {}", e))?;
            // 立即刷新到磁盘，确保数据不会丢失
            file.flush().await.map_err(|e| format!("刷新文件失败: {}", e))?;
            buffer.clear();
        }

        // 计算进度百分比
        if total_size > 0 {
            let percent = ((downloaded as f32 / total_size as f32) * 100.0) as i32;
            let current_time = std::time::Instant::now();
            let elapsed_since_last_log = current_time.duration_since(last_log_time).as_secs_f64();

            // 每完成5%或者距离上次日志输出超过1秒才输出进度
            if percent >= progress_sent + 5 || elapsed_since_last_log >= 1.0 {
                progress_sent = percent;
                last_log_time = current_time;

                // 计算下载速度
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    (downloaded as f64 / elapsed) as u64
                } else {
                    0
                };

                // 发送进度更新到UI
                crate::services::send_download_progress(task_id, downloaded, total_size, speed);
            }
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer)
            .await
            .map_err(|e| format!("写入文件失败: {}", e))?;
    }

    // 确保数据写入磁盘
    file.flush().await.map_err(|e| e.to_string())?;

    // 验证文件完整性
    if let Ok(metadata) = tokio::fs::metadata(&cache_path_buf).await {
        let actual_size = metadata.len();
        info!(
            "[下载任务] [ID:{}] 下载完成：downloaded = {} bytes, total_size = {} bytes, actual_size = {} bytes",
            task_id, downloaded, total_size, actual_size
        );
        if total_size > 0 && actual_size != total_size {
            error!(
                "[下载任务] [ID:{}] 文件大小不匹配：期望 {} bytes，实际 {} bytes",
                task_id, total_size, actual_size
            );
            return Err(format!(
                "文件大小不匹配：期望 {} bytes，实际 {} bytes",
                total_size, actual_size
            ));
        }
    }

    // 返回实际文件大小
    let actual_size = if let Ok(metadata) = tokio::fs::metadata(&cache_path_buf).await {
        metadata.len()
    } else {
        downloaded
    };

    Ok(actual_size)
}