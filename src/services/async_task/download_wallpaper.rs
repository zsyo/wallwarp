// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::io::AsyncWriteExt;
use tracing::info;

/// 异步下载壁纸任务函数
pub async fn async_download_wallpaper_task(
    url: String,
    save_path: PathBuf,
    proxy: Option<String>,
    _task_id: usize,
) -> Result<u64, String> {
    // 确保父目录存在
    if let Some(parent_dir) = save_path.parent() {
        tokio::fs::create_dir_all(parent_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 创建HTTP客户端（带代理）
    let client = if let Some(proxy_url) = &proxy {
        if !proxy_url.is_empty() {
            if let Ok(p) = reqwest::Proxy::all(proxy_url) {
                reqwest::Client::builder().proxy(p).build().map_err(|e| e.to_string())?
            } else {
                reqwest::Client::new()
            }
        } else {
            reqwest::Client::new()
        }
    } else {
        reqwest::Client::new()
    };

    // 发送请求
    let response = client.get(&url).send().await.map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP错误: {}", response.status()));
    }

    // 读取全部数据（对于壁纸文件，使用 bytes() 更简单）
    let bytes = response.bytes().await.map_err(|e| format!("读取数据失败: {}", e))?;

    // 创建文件并写入
    let mut file = tokio::fs::File::create(&save_path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;

    file.write_all(&bytes)
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    file.flush().await.map_err(|e| e.to_string())?;

    // 返回文件大小
    Ok(bytes.len() as u64)
}

/// 带进度更新的异步下载壁纸任务函数
/// 使用 tokio::sync::mpsc 通道来发送进度更新
pub async fn async_download_wallpaper_task_with_progress(
    url: String,
    save_path: PathBuf,
    proxy: Option<String>,
    task_id: usize,
    cancel_token: Arc<AtomicBool>,
    downloaded_size: u64,
    total_size: u64,
    cache_path: String,
) -> Result<u64, String> {
    info!("[下载任务] [ID:{}] 开始下载: {}", task_id, url);
    info!(
        "[下载任务] [ID:{}] 参数：downloaded_size = {} bytes, total_size = {} bytes",
        task_id, downloaded_size, total_size
    );

    // 步骤1: 获取缓存文件路径（带.download后缀）
    // 使用文件总大小（total_size）来生成hash，确保同一任务的缓存路径始终一致
    let temp_cache_path = if total_size > 0 {
        // 使用已知的文件总大小
        DownloadService::get_online_image_cache_path(&cache_path, &url, total_size)
            .map_err(|e| format!("获取缓存路径失败: {}", e))?
    } else {
        // 新下载：先发送HEAD请求获取文件大小
        let create_client = || -> reqwest::Client {
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
                    Ok(p) => reqwest::Client::builder()
                        .proxy(p)
                        .build()
                        .unwrap_or_else(|_| create_client()),
                    Err(_) => create_client(),
                }
            } else {
                create_client()
            }
        } else {
            create_client()
        };

        let head_response = client
            .head(&url)
            .send()
            .await
            .map_err(|e| format!("HEAD请求失败: {}", e))?;
        let file_size = head_response.content_length().unwrap_or(0);
        DownloadService::get_online_image_cache_path(&cache_path, &url, file_size)
            .map_err(|e| format!("获取缓存路径失败: {}", e))?
    };

    info!("[下载任务] [ID:{}] 缓存路径: {}", task_id, temp_cache_path);

    // 步骤2: 确保缓存目录存在
    let cache_file_path = PathBuf::from(&temp_cache_path);
    if let Some(cache_dir) = cache_file_path.parent() {
        tokio::fs::create_dir_all(cache_dir)
            .await
            .map_err(|e| format!("创建缓存目录失败: {}", e))?;
    }

    // 步骤3: 下载图片到缓存目录
    let actual_size = super::download_to_cache(
        &url,
        &temp_cache_path,
        proxy.clone(),
        task_id,
        cancel_token.clone(),
        downloaded_size,
    )
    .await?;

    // 步骤4: 下载完成后，移除.download后缀
    let final_cache_path = DownloadService::get_online_image_cache_final_path(&cache_path, &url, actual_size)
        .map_err(|e| format!("获取最终缓存路径失败: {}", e))?;

    info!(
        "[下载任务] [ID:{}] 重命名文件: {} -> {}",
        task_id, temp_cache_path, final_cache_path
    );
    tokio::fs::rename(&temp_cache_path, &final_cache_path)
        .await
        .map_err(|e| format!("重命名缓存文件失败: {}", e))?;

    // 步骤5: 复制文件到data_path并应用正确的文件名
    // 确保data_path目录存在
    if let Some(parent_dir) = save_path.parent() {
        tokio::fs::create_dir_all(parent_dir)
            .await
            .map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    info!(
        "[下载任务] [ID:{}] 复制文件: {} -> {}",
        task_id,
        final_cache_path,
        save_path.display()
    );
    tokio::fs::copy(&final_cache_path, &save_path)
        .await
        .map_err(|e| format!("复制文件到目标路径失败: {}", e))?;

    info!("[下载任务] [ID:{}] 下载完成，文件大小: {} bytes", task_id, actual_size);

    Ok(actual_size)
}
