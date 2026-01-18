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
    time_range: crate::services::wallhaven::TimeRange,
    atleast: Option<String>,
    resolutions: Option<String>,
    ratios: Option<String>,
    page: usize,
    api_key: Option<String>,
    proxy: Option<String>,
    context: crate::services::request_context::RequestContext,
) -> Result<(Vec<crate::services::wallhaven::OnlineWallpaper>, bool, usize, usize), Box<dyn std::error::Error + Send + Sync>> {
    let service = crate::services::wallhaven::WallhavenService::new(api_key, proxy);
    match service.search_wallpapers(page, categories, sorting, purities, color, &query, time_range, atleast.as_deref(), resolutions.as_deref(), ratios.as_deref(), &context).await {
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
    total_size: u64,
    cache_path: String,
) -> Result<u64, String> {
    info!("[下载任务] [ID:{}] 开始下载: {}", task_id, url);
    info!("[下载任务] [ID:{}] 参数：downloaded_size = {} bytes, total_size = {} bytes", task_id, downloaded_size, total_size);

    // 步骤1: 获取缓存文件路径（带.download后缀）
    // 使用文件总大小（total_size）来生成hash，确保同一任务的缓存路径始终一致
    let temp_cache_path = if total_size > 0 {
        // 使用已知的文件总大小
        crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, total_size)
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
                    Ok(p) => reqwest::Client::builder().proxy(p).build().unwrap_or_else(|_| create_client()),
                    Err(_) => create_client(),
                }
            } else {
                create_client()
            }
        } else {
            create_client()
        };

        let head_response = client.head(&url).send().await.map_err(|e| format!("HEAD请求失败: {}", e))?;
        let file_size = head_response.content_length().unwrap_or(0);
        crate::services::download::DownloadService::get_online_image_cache_path(&cache_path, &url, file_size)
            .map_err(|e| format!("获取缓存路径失败: {}", e))?
    };

    info!("[下载任务] [ID:{}] 缓存路径: {}", task_id, temp_cache_path);

    // 步骤2: 确保缓存目录存在
    let cache_file_path = PathBuf::from(&temp_cache_path);
    if let Some(cache_dir) = cache_file_path.parent() {
        tokio::fs::create_dir_all(cache_dir).await.map_err(|e| format!("创建缓存目录失败: {}", e))?;
    }

    // 步骤3: 下载图片到缓存目录
    let actual_size = download_to_cache(&url, &temp_cache_path, proxy.clone(), task_id, cancel_token.clone(), downloaded_size).await?;

    // 步骤4: 下载完成后，移除.download后缀
    let final_cache_path = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, actual_size)
        .map_err(|e| format!("获取最终缓存路径失败: {}", e))?;

    info!("[下载任务] [ID:{}] 重命名文件: {} -> {}", task_id, temp_cache_path, final_cache_path);
    tokio::fs::rename(&temp_cache_path, &final_cache_path).await
        .map_err(|e| format!("重命名缓存文件失败: {}", e))?;

    // 步骤5: 复制文件到data_path并应用正确的文件名
    // 确保data_path目录存在
    if let Some(parent_dir) = save_path.parent() {
        tokio::fs::create_dir_all(parent_dir).await.map_err(|e| format!("创建目标目录失败: {}", e))?;
    }

    info!("[下载任务] [ID:{}] 复制文件: {} -> {}", task_id, final_cache_path, save_path.display());
    tokio::fs::copy(&final_cache_path, &save_path).await
        .map_err(|e| format!("复制文件到目标路径失败: {}", e))?;

    info!("[下载任务] [ID:{}] 下载完成，文件大小: {} bytes", task_id, actual_size);

    Ok(actual_size)
}

/// 异步加载在线壁纸图片函数（流式下载，支持取消）
pub async fn async_load_online_wallpaper_image_with_streaming(
    url: String,
    file_size: u64,
    cache_path: String,
    proxy: Option<String>,
    cancel_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
    use iced::futures::StreamExt;
    use std::path::Path;
    use tracing::debug;
    use tracing::error;
    use tracing::info;
    use xxhash_rust::xxh3::xxh3_128;

    debug!("[模态窗口图片下载] [URL:{}] 开始流式下载，文件大小: {} bytes", url, file_size);

    // 步骤1: 计算缓存文件路径
    let hash_input = format!("{}{}", url, file_size);
    let hash = xxh3_128(hash_input.as_bytes());
    let extension = Path::new(&url).extension().and_then(|ext| ext.to_str()).unwrap_or("jpg");
    let cache_dir = std::path::PathBuf::from(&cache_path).join("online");
    let cache_file = cache_dir.join(format!("{:x}.{}", hash, extension));

    // 步骤2: 检查缓存是否存在且大小匹配
    if cache_file.exists() {
        if let Ok(metadata) = std::fs::metadata(&cache_file) {
            let cache_size = metadata.len();
            if cache_size == file_size {
                debug!("[模态窗口图片下载] [URL:{}] 使用缓存: {}", url, cache_file.display());
                return Ok(iced::widget::image::Handle::from_path(&cache_file));
            }
        }
    }

    // 步骤3: 确保缓存目录存在
    if let Some(cache_dir_path) = cache_file.parent() {
        tokio::fs::create_dir_all(cache_dir_path).await.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 创建缓存目录失败: {}", url, e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
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
                            debug!("[模态窗口图片下载] [URL:{}] HTTP客户端创建失败: {}，回退到无代理", url, e);
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
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    if !response.status().is_success() {
        let error_msg = format!("下载失败: {}", response.status());
        error!("[模态窗口图片下载] [URL:{}] {}", url, error_msg);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)) as Box<dyn std::error::Error + Send + Sync>);
    }

    // 步骤6: 获取总大小
    let total_size = response.content_length().unwrap_or(file_size);

    // 步骤7: 流式下载图片到内存并写入文件
    let mut stream = response.bytes_stream();
    let mut file = tokio::fs::File::create(&cache_file).await.map_err(|e| {
        error!("[模态窗口图片下载] [URL:{}] 创建文件失败: {}", url, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
    })?;

    let mut downloaded: u64 = 0;
    let mut progress_sent = 0i32;
    let mut buffer = Vec::with_capacity(1024 * 1024); // 1MB缓冲区

    while let Some(chunk_result) = stream.next().await {
        // 检查是否被取消
        if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
            info!("[模态窗口图片下载] [URL:{}] 下载被取消", url);
            // 删除未完成的文件
            let _ = tokio::fs::remove_file(&cache_file).await;
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "下载已取消")) as Box<dyn std::error::Error + Send + Sync>);
        }

        let chunk = chunk_result.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 读取数据流失败: {}", url, e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
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
                Box::new(e) as Box<dyn std::error::Error + Send + Sync>
            })?;
            buffer.clear();
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer).await.map_err(|e| {
            error!("[模态窗口图片下载] [URL:{}] 写入文件失败: {}", url, e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;
    }

    // 确保数据写入磁盘
    file.flush().await.map_err(|e| {
        error!("[模态窗口图片下载] [URL:{}] 刷新文件失败: {}", url, e);
        Box::new(e) as Box<dyn std::error::Error + Send + Sync>
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
                format!("文件大小不匹配：期望 {} bytes，实际 {} bytes", total_size, actual_size)
            )) as Box<dyn std::error::Error + Send + Sync>);
        }
    }

    info!("[模态窗口图片下载] [URL:{}] 下载完成，文件大小: {} bytes", url, downloaded);

    // 返回图片Handle
    Ok(iced::widget::image::Handle::from_path(&cache_file))
}

/// 辅助函数：下载图片到缓存目录
async fn download_to_cache(
    url: &str,
    cache_path: &str,
    proxy: Option<String>,
    task_id: usize,
    cancel_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
    downloaded_size: u64,
) -> Result<u64, String> {
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
        use tokio::io::AsyncSeekExt;
        f.seek(std::io::SeekFrom::Start(downloaded_size as u64))
            .await
            .map_err(|e| format!("移动文件指针失败: {}", e))?;

        info!("[下载任务] [ID:{}] 断点续传：文件指针移动到位置 {} bytes", task_id, downloaded_size);

        f
    } else {
        // 创建新文件
        tokio::fs::File::create(&cache_path_buf).await.map_err(|e| format!("创建文件失败: {}", e))?
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

        // 当缓冲区达到64KB时，批量写入文件
        if buffer.len() >= 64 * 1024 {
            file.write_all(&buffer).await.map_err(|e| format!("写入文件失败: {}", e))?;
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
    if let Ok(metadata) = tokio::fs::metadata(&cache_path_buf).await {
        let actual_size = metadata.len();
        info!("[下载任务] [ID:{}] 下载完成：downloaded = {} bytes, total_size = {} bytes, actual_size = {} bytes", task_id, downloaded, total_size, actual_size);
        if total_size > 0 && actual_size != total_size {
            error!(
                "[下载任务] [ID:{}] 文件大小不匹配：期望 {} bytes，实际 {} bytes",
                task_id, total_size, actual_size
            );
            return Err(format!("文件大小不匹配：期望 {} bytes，实际 {} bytes", total_size, actual_size));
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
