use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::error;
use tracing::info;
use tracing::warn;

/// 异步加载壁纸路径列表函数
pub async fn async_load_wallpaper_paths(data_path: String) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    // 在这里调用同步的获取壁纸路径函数
    tokio::task::spawn_blocking(move || crate::services::local::LocalWallpaperService::get_wallpaper_paths(&data_path))
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

/// 异步加载单个壁纸函数（带降级处理，即使图片加载失败也能获取文件大小）
pub async fn async_load_single_wallpaper_with_fallback(
    wallpaper_path: String,
    cache_path: String,
) -> Result<crate::services::local::Wallpaper, Box<dyn std::error::Error + Send + Sync>> {
    let full_cache_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")).join(&cache_path);

    // 使用spawn_blocking在阻塞线程池中运行
    tokio::task::spawn_blocking(move || {
        // 先获取文件大小（这个操作通常不会失败）
        let file_size = std::fs::metadata(&wallpaper_path).map(|metadata| metadata.len()).unwrap_or(0);

        // 尝试加载图片
        let result = (|| -> Result<crate::services::local::Wallpaper, Box<dyn std::error::Error + Send + Sync>> {
            let thumbnail_path =
                crate::services::local::LocalWallpaperService::generate_thumbnail_for_path(&wallpaper_path, &full_cache_path.to_string_lossy())?;

            // 获取图片尺寸
            let (width, height) = image::image_dimensions(&wallpaper_path).unwrap_or((0, 0));

            Ok(crate::services::local::Wallpaper::with_thumbnail(
                wallpaper_path.clone(),
                std::path::Path::new(&wallpaper_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                thumbnail_path,
                file_size,
                width,
                height,
            ))
        })();

        match result {
            Ok(wallpaper) => Ok(wallpaper),
            Err(_) => {
                // 如果加载失败，返回一个带有文件大小的失败状态
                Ok(crate::services::local::Wallpaper::new(
                    wallpaper_path.clone(),
                    "加载失败".to_string(),
                    file_size,
                    0,
                    0,
                ))
            }
        }
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

/// 异步设置壁纸函数
pub async fn async_set_wallpaper(wallpaper_path: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || crate::services::local::LocalWallpaperService::set_wallpaper(&wallpaper_path))
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

/// 异步函数用于打开目录选择对话框
pub async fn select_folder_async() -> String {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        path.to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}

/// 异步加载在线壁纸函数
pub async fn async_load_online_wallpapers(
    categories: u32,
    sorting: crate::services::wallhaven::Sorting,
    purities: u32,
    color: crate::services::wallhaven::ColorOption,
    query: String,
    page: usize,
    api_key: Option<String>,
    proxy: Option<String>,
    context: crate::services::request_context::RequestContext,
) -> Result<(Vec<crate::services::wallhaven::OnlineWallpaper>, bool, usize, usize), Box<dyn std::error::Error + Send + Sync>> {
    let service = crate::services::wallhaven::WallhavenService::new(api_key, proxy);
    match service.search_wallpapers(page, categories, sorting, purities, color, &query, &context).await {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>),
    }
}

/// 异步加载在线壁纸图片函数
pub async fn async_load_online_wallpaper_image(
    url: String,
    proxy: Option<String>,
) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
    let client = if let Some(proxy_url) = proxy {
        if !proxy_url.is_empty() {
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                reqwest::Client::builder().proxy(proxy).build().unwrap_or_else(|_| reqwest::Client::new())
            } else {
                reqwest::Client::new()
            }
        } else {
            reqwest::Client::new()
        }
    } else {
        reqwest::Client::new()
    };

    let response = client.get(&url).send().await.map_err(|e| {
        error!("[图片加载] [URL:{}] 请求失败: {}", url, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    if !response.status().is_success() {
        let error_msg = format!("下载失败: {}", response.status());
        error!("[图片加载] [URL:{}] {}", url, error_msg);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)) as Box<dyn std::error::Error + Send + Sync>);
    }

    let bytes = response.bytes().await.map_err(|e| {
        error!("[图片加载] [URL:{}] 读取响应体失败: {}", url, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    Ok(iced::widget::image::Handle::from_bytes(bytes.to_vec()))
}

/// 异步加载在线壁纸缩略图函数（带缓存）
pub async fn async_load_online_wallpaper_thumb_with_cache(
    url: String,
    file_size: u64,
    cache_base_path: String,
    proxy: Option<String>,
) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
    // 使用DownloadService的智能缓存加载功能
    crate::services::download::DownloadService::load_thumb_with_cache(url, file_size, cache_base_path, proxy).await
}

/// 异步下载壁纸任务函数
pub async fn async_download_wallpaper_task(url: String, save_path: PathBuf, proxy: Option<String>, _task_id: usize) -> Result<u64, String> {
    // 确保父目录存在
    if let Some(parent_dir) = save_path.parent() {
        tokio::fs::create_dir_all(parent_dir).await.map_err(|e| format!("创建目录失败: {}", e))?;
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
    let mut file = tokio::fs::File::create(&save_path).await.map_err(|e| format!("创建文件失败: {}", e))?;

    use tokio::io::AsyncWriteExt;
    file.write_all(&bytes).await.map_err(|e| format!("写入文件失败: {}", e))?;

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
    cancel_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
    downloaded_size: u64,
) -> Result<u64, String> {
    // 确保父目录存在
    if let Some(parent_dir) = save_path.parent() {
        tokio::fs::create_dir_all(parent_dir).await.map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 创建优化的HTTP客户端（带代理）
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
                            warn!("[下载任务] [ID:{}] HTTP客户端创建失败: {}，回退到无代理", task_id, e);
                            create_optimized_client()
                        }
                    }
                }
                Err(e) => {
                    warn!("[下载任务] [ID:{}] Proxy::all 失败: {}，回退到无代理", task_id, e);
                    create_optimized_client()
                }
            }
        } else {
            create_optimized_client()
        }
    } else {
        create_optimized_client()
    };

    // 发送请求（支持断点续传）
    let response = if downloaded_size > 0 {
        // 断点续传：使用 Range 请求头
        let range_header = format!("bytes={}-", downloaded_size);
        let resp = client
            .get(&url)
            .header("Range", range_header)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        resp
    } else {
        // 新下载
        client.get(&url).send().await.map_err(|e| format!("请求失败: {}", e))?
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

    // 打开文件（追加模式用于断点续传）
    let mut file = if downloaded_size > 0 {
        // 断点续传：检查文件是否存在并验证大小
        if !save_path.exists() {
            return Err(format!("文件不存在，无法断点续传: {}", save_path.display()));
        }

        // 检查文件大小是否匹配
        if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
            let actual_size = metadata.len();
            if actual_size != downloaded_size {
                return Err(format!("文件大小不匹配，期望: {} bytes，实际: {} bytes", downloaded_size, actual_size));
            }
        }

        tokio::fs::OpenOptions::new()
            .write(true)
            .open(&save_path)
            .await
            .map_err(|e| format!("打开文件失败: {}", e))?
    } else {
        // 创建新文件
        tokio::fs::File::create(&save_path).await.map_err(|e| format!("创建文件失败: {}", e))?
    };

    let mut downloaded: u64 = downloaded_size;
    let mut progress_sent = 0i32;
    let start_time = std::time::Instant::now();
    let mut buffer = Vec::with_capacity(1024 * 1024);
    let mut last_log_time = start_time;

    use iced::futures::StreamExt;

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

        // 当缓冲区达到1MB时，批量写入文件
        if buffer.len() >= 1024 * 1024 {
            file.write_all(&buffer).await.map_err(|e| format!("写入文件失败: {}", e))?;
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
                let speed = if elapsed > 0.0 { (downloaded as f64 / elapsed) as u64 } else { 0 };

                // 发送进度更新到UI
                crate::services::send_download_progress(task_id, downloaded, total_size, speed);
            }
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer).await.map_err(|e| format!("写入文件失败: {}", e))?;
    }

    // 确保数据写入磁盘
    file.flush().await.map_err(|e| e.to_string())?;

    // 验证文件完整性
    if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
        let actual_size = metadata.len();
        if total_size > 0 && actual_size != total_size {
            error!(
                "[下载任务] [ID:{}] 文件大小不匹配：期望 {} bytes，实际 {} bytes",
                task_id, total_size, actual_size
            );
            return Err(format!("文件大小不匹配：期望 {} bytes，实际 {} bytes", total_size, actual_size));
        }
    }

    // 返回实际文件大小
    let actual_size = if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
        metadata.len()
    } else {
        downloaded
    };

    Ok(actual_size)
}
