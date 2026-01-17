use std::fs;
use std::path::{Path, PathBuf};
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;
use xxhash_rust::xxh3::xxh3_128;

/// 下载服务，处理在线壁纸的缓存和下载
pub struct DownloadService;

impl DownloadService {
    /// 执行带重试的异步操作
    /// max_retries: 最大重试次数
    /// operation: 要执行的异步操作，返回Result
    async fn retry_with_backoff<F, T, E, Fut>(
        url: &str,
        operation_name: &str,
        max_retries: usize,
        mut operation: F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        info!("[{}] [URL:{}] 重试第 {} 次成功", operation_name, url, attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(Box::new(e) as Box<dyn std::error::Error + Send + Sync>);
                    if attempt < max_retries {
                        warn!(
                            "[{}] [URL:{}] 第 {} 次尝试失败，将在1秒后重试: {}",
                            operation_name,
                            url,
                            attempt + 1,
                            last_error.as_ref().unwrap()
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    } else {
                        error!("[{}] [URL:{}] 所有重试失败，共尝试 {} 次", operation_name, url, max_retries + 1);
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }
    /// 获取在线缩略图缓存路径
    /// 根据URL和文件大小生成hash值，用于缓存文件命名
    pub fn get_online_thumb_cache_path(cache_base_path: &str, url: &str, file_size: u64) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 计算hash值：使用URL + file_size
        let hash_input = format!("{}{}", url, file_size);
        let hash = xxh3_128(hash_input.as_bytes());

        // 从 URL 中提取文件后缀
        let extension = Path::new(url).extension().and_then(|ext| ext.to_str()).unwrap_or("jpg"); // 默认使用 jpg

        // 创建缓存目录路径
        let cache_dir = PathBuf::from(cache_base_path).join("online");

        // 生成缓存文件路径
        let cache_file = cache_dir.join(format!("{:x}.{}", hash, extension));

        Ok(cache_file.to_string_lossy().to_string())
    }

    /// 检查缩略图缓存是否存在
    pub fn check_thumb_cache_exists(cache_path: &str) -> bool {
        Path::new(cache_path).exists()
    }

    /// 获取在线原图缓存路径
    /// 根据URL和文件大小生成hash值，用于缓存文件命名
    pub fn get_online_image_cache_path(cache_base_path: &str, url: &str, file_size: u64) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 计算hash值：使用URL + file_size
        let hash_input = format!("{}{}", url, file_size);
        let hash = xxh3_128(hash_input.as_bytes());

        // 从 URL 中提取文件后缀
        let extension = Path::new(url).extension().and_then(|ext| ext.to_str()).unwrap_or("jpg"); // 默认使用 jpg

        // 创建缓存目录路径
        let cache_dir = PathBuf::from(cache_base_path).join("online");

        // 生成缓存文件路径（带.download后缀表示下载中）
        let cache_file = cache_dir.join(format!("{:x}.{}.download", hash, extension));

        Ok(cache_file.to_string_lossy().to_string())
    }

    /// 获取在线原图缓存路径（下载完成后的最终路径，不带.download后缀）
    pub fn get_online_image_cache_final_path(cache_base_path: &str, url: &str, file_size: u64) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 计算hash值：使用URL + file_size
        let hash_input = format!("{}{}", url, file_size);
        let hash = xxh3_128(hash_input.as_bytes());

        // 从 URL 中提取文件后缀
        let extension = Path::new(url).extension().and_then(|ext| ext.to_str()).unwrap_or("jpg"); // 默认使用 jpg

        // 创建缓存目录路径
        let cache_dir = PathBuf::from(cache_base_path).join("online");

        // 生成缓存文件路径（不带.download后缀）
        let cache_file = cache_dir.join(format!("{:x}.{}", hash, extension));

        Ok(cache_file.to_string_lossy().to_string())
    }

    /// 下载缩略图到缓存目录（带重试机制，最多重试3次）
    pub async fn download_thumb_to_cache(url: &str, cache_path: &str, proxy: Option<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        debug!("[缩略图缓存] [URL:{}] 开始下载到: {}", url, cache_path);

        // 确保缓存目录存在
        let cache_file_path = Path::new(cache_path);
        if let Some(cache_dir) = cache_file_path.parent() {
            fs::create_dir_all(cache_dir).map_err(|e| {
                error!("[缩略图缓存] [URL:{}] 创建缓存目录失败: {}", url, e);
                Box::new(e) as Box<dyn std::error::Error + Send + Sync>
            })?;
        }

        // 创建优化的HTTP客户端
        let create_optimized_client = || -> reqwest::Client {
            reqwest::Client::builder()
                // 连接池配置：最大100个连接，每个主机最多10个连接
                .pool_max_idle_per_host(10)
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                // 超时配置
                .connect_timeout(std::time::Duration::from_secs(30))
                .timeout(std::time::Duration::from_secs(300))
                // TCP配置：启用TCP_NODELAY减少延迟
                .tcp_nodelay(true)
                // 启用HTTP/2
                .http2_prior_knowledge()
                // 启用gzip压缩（reqwest默认支持）
                .gzip(true)
                // 启用brotli压缩（需要features支持）
                .brotli(true)
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        };

        let client = if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                debug!("[缩略图缓存] [URL:{}] 尝试创建代理客户端，代理URL: {}", url, proxy_url);
                match reqwest::Proxy::all(&proxy_url) {
                    Ok(p) => {
                        debug!("[缩略图缓存] [URL:{}] Proxy::all 成功", url);
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
                            Ok(http_client) => {
                                debug!("[缩略图缓存] [URL:{}] HTTP客户端创建成功（已优化）", url);
                                http_client
                            }
                            Err(e) => {
                                warn!("[缩略图缓存] [URL:{}] HTTP客户端创建失败: {}，回退到无代理", url, e);
                                create_optimized_client()
                            }
                        }
                    }
                    Err(e) => {
                        warn!("[缩略图缓存] [URL:{}] Proxy::all 失败: {}，回退到无代理", url, e);
                        create_optimized_client()
                    }
                }
            } else {
                debug!("[缩略图缓存] [URL:{}] 代理URL为空，使用无代理客户端", url);
                create_optimized_client()
            }
        } else {
            debug!("[缩略图缓存] [URL:{}] 无代理配置，使用无代理客户端", url);
            create_optimized_client()
        };

        // 使用重试机制下载图片
        let bytes = Self::retry_with_backoff(
            url,
            "缩略图缓存",
            3, // 最多重试3次
            || {
                let client = client.clone();
                let url = url.to_string();
                async move {
                    let response = client.get(&url).send().await.map_err(|e| {
                        error!("[缩略图缓存] [URL:{}] 请求失败: {}", url, e);
                        e
                    })?;

                    debug!("[缩略图缓存] [URL:{}] 响应状态: {}", url, response.status());

                    if !response.status().is_success() {
                        let status = response.status();
                        let error_msg = format!("下载失败: {}", status);
                        error!("[缩略图缓存] [URL:{}] {}", url, error_msg);
                        // 手动构造错误，使用 response.error_for_status_ref()
                        let _ = response.error_for_status_ref()?;
                        unreachable!();
                    }

                    response.bytes().await
                }
            },
        )
        .await?;

        debug!("[缩略图缓存] [URL:{}] 下载成功，数据大小: {} bytes", url, bytes.len());

        // 保存到缓存
        fs::write(cache_path, bytes).map_err(|e| {
            error!("[缩略图缓存] [URL:{}] 保存文件失败: {}", url, e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;

        debug!("[缩略图缓存] [URL:{}] 文件保存成功: {}", url, cache_path);

        Ok(())
    }

    /// 获取缓存图片的Handle
    /// 如果缓存存在则直接加载，否则返回None
    pub fn get_cached_thumb_handle(cache_path: &str) -> Option<iced::widget::image::Handle> {
        if Self::check_thumb_cache_exists(cache_path) {
            debug!("[缩略图缓存] 使用缓存: {}", cache_path);
            Some(iced::widget::image::Handle::from_path(Path::new(cache_path)))
        } else {
            debug!("[缩略图缓存] 缓存不存在: {}", cache_path);
            None
        }
    }

    /// 下载图片并返回Handle（不带缓存，带重试机制，最多重试3次）
    pub async fn download_image_handle(url: String, proxy: Option<String>) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        debug!("[图片下载] [URL:{}] 开始下载", url);

        let create_optimized_client = || -> reqwest::Client {
            reqwest::Client::builder()
                // 连接池配置：最大100个连接，每个主机最多10个连接
                .pool_max_idle_per_host(10)
                .pool_idle_timeout(std::time::Duration::from_secs(90))
                // 超时配置
                .connect_timeout(std::time::Duration::from_secs(30))
                .timeout(std::time::Duration::from_secs(300))
                // TCP配置：启用TCP_NODELAY减少延迟
                .tcp_nodelay(true)
                // 启用HTTP/2
                .http2_prior_knowledge()
                // 启用gzip压缩（reqwest默认支持）
                .gzip(true)
                // 启用brotli压缩（需要features支持）
                .brotli(true)
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        };

        let client = if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                debug!("[图片下载] [URL:{}] 尝试创建代理客户端，代理URL: {}", url, proxy_url);
                match reqwest::Proxy::all(&proxy_url) {
                    Ok(p) => {
                        debug!("[图片下载] [URL:{}] Proxy::all 成功", url);
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
                            Ok(http_client) => {
                                debug!("[图片下载] [URL:{}] HTTP客户端创建成功（已优化）", url);
                                http_client
                            }
                            Err(e) => {
                                warn!("[图片下载] [URL:{}] HTTP客户端创建失败: {}，回退到无代理", url, e);
                                create_optimized_client()
                            }
                        }
                    }
                    Err(e) => {
                        warn!("[图片下载] [URL:{}] Proxy::all 失败: {}，回退到无代理", url, e);
                        create_optimized_client()
                    }
                }
            } else {
                debug!("[图片下载] [URL:{}] 代理URL为空，使用无代理客户端", url);
                create_optimized_client()
            }
        } else {
            debug!("[图片下载] [URL:{}] 无代理配置，使用无代理客户端", url);
            create_optimized_client()
        };

        // 使用重试机制下载图片
        let bytes = Self::retry_with_backoff(
            &url,
            "图片下载",
            3, // 最多重试3次
            || {
                let client = client.clone();
                let url = url.clone();
                async move {
                    let response = client.get(&url).send().await.map_err(|e| {
                        error!("[图片下载] [URL:{}] 请求失败: {}", url, e);
                        e
                    })?;

                    debug!("[图片下载] [URL:{}] 响应状态: {}", url, response.status());

                    if !response.status().is_success() {
                        let status = response.status();
                        let error_msg = format!("下载失败: {}", status);
                        error!("[图片下载] [URL:{}] {}", url, error_msg);
                        // 手动构造错误，使用 response.error_for_status_ref()
                        let _ = response.error_for_status_ref()?;
                        unreachable!();
                    }

                    response.bytes().await
                }
            },
        )
        .await?;

        info!("[图片下载] [URL:{}] 下载成功，数据大小: {} bytes", url, bytes.len());

        Ok(iced::widget::image::Handle::from_bytes(bytes.to_vec()))
    }

    /// 智能加载缩略图：优先使用缓存，缓存不存在时下载并缓存
    pub async fn load_thumb_with_cache(
        url: String,
        file_size: u64,
        cache_base_path: String,
        proxy: Option<String>,
    ) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
        // 计算缓存路径
        let cache_path = Self::get_online_thumb_cache_path(&cache_base_path, &url, file_size)?;

        // 检查缓存是否存在
        if let Some(cached_handle) = Self::get_cached_thumb_handle(&cache_path) {
            return Ok(cached_handle);
        }

        // 缓存不存在，下载图片
        debug!("[缩略图缓存] [URL:{}] 缓存不存在，开始下载", url);

        // 下载并保存到缓存
        Self::download_thumb_to_cache(&url, &cache_path, proxy).await?;

        // 返回缓存的图片Handle
        Ok(iced::widget::image::Handle::from_path(Path::new(&cache_path)))
    }
}
