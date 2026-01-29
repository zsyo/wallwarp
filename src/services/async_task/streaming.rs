// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use iced::futures::StreamExt;
use iced::widget::image::Handle;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::AsyncWriteExt;
use tracing::{debug, error, info};
use xxhash_rust::xxh3::xxh3_128;

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

    // 步骤1: 计算缓存文件路径
    let hash_input = format!("{}{}", url, file_size);
    let hash = xxh3_128(hash_input.as_bytes());
    let extension = Path::new(&url)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");
    let cache_dir = PathBuf::from(&cache_path).join("online");
    let cache_file = cache_dir.join(format!("{:x}.{}", hash, extension));

    // 步骤2: 检查缓存是否存在且大小匹配
    if cache_file.exists() {
        if let Ok(metadata) = std::fs::metadata(&cache_file) {
            let cache_size = metadata.len();
            if cache_size == file_size {
                debug!("[模态窗口图片下载] [URL:{}] 使用缓存: {}", url, cache_file.display());
                return Ok(Handle::from_path(&cache_file));
            }
        }
    }

    // 步骤3: 确保缓存目录存在
    if let Some(cache_dir_path) = cache_file.parent() {
        tokio::fs::create_dir_all(cache_dir_path).await.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 创建缓存目录失败: {}", url, e);
            Box::new(e) as Box<dyn Error + Send + Sync>
        })?;
    }

    // 步骤4: 创建优化的HTTP客户端（带代理）
    let create_optimized_client = || -> reqwest::Client {
        reqwest::Client::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(300))
            .tcp_nodelay(true)
            .http2_prior_knowledge()
            .gzip(true)
            .brotli(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    };

    let client = if let Some(proxy_url) = &proxy {
        if !proxy_url.is_empty() {
            match reqwest::Proxy::all(proxy_url) {
                Ok(p) => {
                    match reqwest::Client::builder()
                        .proxy(p)
                        .pool_max_idle_per_host(10)
                        .pool_idle_timeout(std::time::Duration::from_secs(90))
                        .connect_timeout(std::time::Duration::from_secs(30))
                        .timeout(std::time::Duration::from_secs(300))
                        .tcp_nodelay(true)
                        .http2_prior_knowledge()
                        .gzip(true)
                        .brotli(true)
                        .build()
                    {
                        Ok(http_client) => http_client,
                        Err(e) => {
                            debug!(
                                "[模态窗口图片下载] [URL:{}] HTTP客户端创建失败: {}，回退到无代理",
                                url, e
                            );
                            create_optimized_client()
                        }
                    }
                }
                Err(e) => {
                    debug!("[模态窗口图片下载] [URL:{}] Proxy::all 失败: {}，回退到无代理", url, e);
                    create_optimized_client()
                }
            }
        } else {
            create_optimized_client()
        }
    } else {
        create_optimized_client()
    };

    // 步骤5: 发送请求
    let response = client.get(&url).send().await.map_err(|e| {
        error!("[模态窗口图片下载] [URL:{}] 请求失败: {}", url, e);
        Box::new(e) as Box<dyn Error + Send + Sync>
    })?;

    if !response.status().is_success() {
        let error_msg = format!("下载失败: {}", response.status());
        error!("[模态窗口图片下载] [URL:{}] {}", url, error_msg);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)) as Box<dyn Error + Send + Sync>);
    }

    // 步骤6: 获取总大小
    let total_size = response.content_length().unwrap_or(file_size);

    // 步骤7: 流式下载图片到内存并写入文件
    let mut stream = response.bytes_stream();
    let mut file = tokio::fs::File::create(&cache_file).await.map_err(|e| {
        error!("[模态窗口图片下载] [URL:{}] 创建文件失败: {}", url, e);
        Box::new(e) as Box<dyn Error + Send + Sync>
    })?;

    let mut downloaded: u64 = 0;
    let mut progress_sent = 0i32;
    let mut buffer = Vec::with_capacity(1024 * 1024); // 1MB缓冲区

    while let Some(chunk_result) = stream.next().await {
        // 检查是否被取消
        if cancel_token.load(Ordering::Relaxed) {
            info!("[模态窗口图片下载] [URL:{}] 下载被取消", url);
            // 删除未完成的文件
            let _ = tokio::fs::remove_file(&cache_file).await;
            return Err(
                Box::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "下载已取消"))
                    as Box<dyn Error + Send + Sync>,
            );
        }

        let chunk = chunk_result.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 读取数据流失败: {}", url, e);
            Box::new(e) as Box<dyn Error + Send + Sync>
        })?;

        // 将数据添加到缓冲区
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;

        // 计算进度百分比
        if total_size > 0 {
            let progress = downloaded as f32 / total_size as f32;
            let percent = (progress * 100.0) as i32;

            // 每完成5%发送一次进度更新
            if percent >= progress_sent + 5 {
                progress_sent = percent;
            }
        }

        // 当缓冲区达到1MB时，批量写入文件
        if buffer.len() >= 1024 * 1024 {
            file.write_all(&buffer).await.map_err(|e| {
                error!("[模态窗口图片下载] [URL:{}] 写入文件失败: {}", url, e);
                Box::new(e) as Box<dyn Error + Send + Sync>
            })?;
            buffer.clear();
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer).await.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 写入文件失败: {}", url, e);
            Box::new(e) as Box<dyn Error + Send + Sync>
        })?;
    }

    // 确保数据写入磁盘
    file.flush().await.map_err(|e| {
        error!("[模态窗口图片下载] [URL:{}] 刷新文件失败: {}", url, e);
        Box::new(e) as Box<dyn Error + Send + Sync>
    })?;

    // 验证文件完整性
    if let Ok(metadata) = tokio::fs::metadata(&cache_file).await {
        let actual_size = metadata.len();
        if total_size > 0 && actual_size != total_size {
            error!(
                "[模态窗口图片下载] [URL:{}] 文件大小不匹配：期望 {} bytes，实际 {} bytes",
                url, total_size, actual_size
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("文件大小不匹配：期望 {} bytes，实际 {} bytes", total_size, actual_size),
            )) as Box<dyn Error + Send + Sync>);
        }
    }

    info!(
        "[模态窗口图片下载] [URL:{}] 下载完成，文件大小: {} bytes",
        url, downloaded
    );

    // 返回图片Handle
    Ok(Handle::from_path(&cache_file))
}
