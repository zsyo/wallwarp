use crate::ui::online::{OnlineWallpaper, Sorting};
use serde::Deserialize;

use crate::ui::online::SortDirection;

const BASE_URL: &str = "https://wallhaven.cc/api/v1";

#[derive(Debug, Deserialize)]
struct WallhavenResponse<T> {
    data: T,
    meta: Option<WallhavenMeta>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WallhavenMeta {
    current_page: u64,
    last_page: u64,
    per_page: serde_json::Value,
    total: u64,
}

#[derive(Debug, Deserialize)]
struct WallhavenWallpaperData {
    id: String,
    url: String,
    path: String,
    thumbs: WallhavenThumbs,
    dimension_x: u32,
    dimension_y: u32,
    resolution: String,
    ratio: String,
    file_size: u64,
    file_type: String,
    category: String,
    purity: String,
    views: u32,
    favorites: u32,
    #[serde(default)]
    colors: Vec<String>,
    #[serde(default)]
    tags: Option<Vec<WallhavenTag>>,
}

#[derive(Debug, Deserialize)]
struct WallhavenThumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Debug, Deserialize)]
struct WallhavenTag {
    name: String,
}

impl From<WallhavenWallpaperData> for OnlineWallpaper {
    fn from(data: WallhavenWallpaperData) -> Self {
        OnlineWallpaper {
            id: data.id,
            url: data.url,
            path: data.path,
            thumb_large: data.thumbs.large,
            thumb_original: data.thumbs.original,
            thumb_small: data.thumbs.small,
            width: data.dimension_x,
            height: data.dimension_y,
            resolution: data.resolution,
            ratio: data.ratio,
            file_size: data.file_size,
            file_type: data.file_type,
            category: data.category,
            purity: data.purity,
            views: data.views,
            favorites: data.favorites,
            colors: data.colors,
            tags: data.tags.unwrap_or_default().into_iter().map(|t| t.name).collect(),
        }
    }
}

pub struct WallhavenService {
    api_key: Option<String>,
    client: reqwest::Client,
}

impl WallhavenService {
    pub fn new(api_key: Option<String>, proxy: Option<String>) -> Self {
        let client = if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                    reqwest::Client::builder()
                        .proxy(proxy)
                        .build()
                        .unwrap_or_else(|_| reqwest::Client::new())
                } else {
                    reqwest::Client::new()
                }
            } else {
                reqwest::Client::new()
            }
        } else {
            reqwest::Client::new()
        };

        Self {
            api_key,
            client,
        }
    }

    /// 执行带重试的HTTP请求
    /// max_retries: 最大重试次数
    /// operation: 要执行的异步操作，返回Result
    async fn retry_with_backoff<F, T, Fut>(
        identifier: &str,
        _operation_name: &str,
        max_retries: usize,
        mut operation: F,
    ) -> Result<T, String>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        let mut last_error = String::new();

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        println!("[Wallhaven API] [{}] 重试第 {} 次成功", identifier, attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = e;
                    if attempt < max_retries {
                        println!("[Wallhaven API] [{}] 第 {} 次尝试失败，将在1秒后重试: {}",
                            identifier, attempt + 1, last_error);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    } else {
                        println!("[Wallhaven API] [{}] 所有重试失败，共尝试 {} 次",
                            identifier, max_retries + 1);
                    }
                }
            }
        }

        Err(last_error)
    }

    pub async fn search_wallpapers(
        &self,
        page: usize,
        categories: u32,  // 位掩码
        sorting: Sorting,
        sort_direction: SortDirection,
        purities: u32,    // 位掩码
        query: &str,
    ) -> Result<(Vec<OnlineWallpaper>, bool, usize), String> {
        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        let mut url = format!("{}/search?page={}", BASE_URL, page);

        // 添加分类参数（使用位掩码）
        let categories_str = format!("{:03b}", categories);
        url.push_str(&format!("&categories={}", categories_str));

        // 添加纯净度参数（使用位掩码）
        let purity_str = format!("{:03b}", purities);
        url.push_str(&format!("&purity={}", purity_str));

        // 添加排序参数
        url.push_str(&format!("&sorting={}", sorting.value()));

        // 添加排序方向参数
        url.push_str(&format!("&order={}", sort_direction.value()));

        // 添加搜索查询
        if !query.is_empty() {
            url.push_str(&format!("&q={}", urlencoding::encode(query)));
        }

        // 添加API密钥（如果提供）
        if let Some(ref key) = self.api_key {
            url.push_str(&format!("&apikey={}", key));
        }

        // 打印请求参数
        let search_tag = format!("page{}_cat{:03b}_sort{:?}_purity{:03b}_q{}",
            page, categories, sorting, purities, if query.is_empty() { "empty" } else { &query[..query.len().min(10)] });
        println!("[Wallhaven API] [{}] 请求URL: {}", search_tag, url);

        // 使用重试机制执行请求
        let text = Self::retry_with_backoff(
            &search_tag,
            "搜索壁纸",
            3, // 最多重试3次
            || {
                let client = self.client.clone();
                let url = url.clone();
                let search_tag = search_tag.clone();
                async move {
                    let response = client.get(&url).send().await
                        .map_err(|e| {
                            println!("[Wallhaven API] [{}] 请求失败: {}", search_tag, e);
                            format!("请求失败: {}", e)
                        })?;

                    println!("[Wallhaven API] [{}] 响应状态: {}", search_tag, response.status());

                    if !response.status().is_success() {
                        return Err(format!("API返回错误: {}", response.status()));
                    }

                    response.text().await
                        .map_err(|e| {
                            println!("[Wallhaven API] [{}] 读取响应失败: {}", search_tag, e);
                            format!("读取响应失败: {}", e)
                        })
                }
            },
        ).await?;

        let wallhaven_response: WallhavenResponse<Vec<WallhavenWallpaperData>> =
            serde_json::from_str(&text).map_err(|e| {
                println!("[Wallhaven API] [{}] JSON解析失败: {}", search_tag, e);
                format!("解析JSON失败: {}", e)
            })?;

        // 打印解析结果
        println!("[Wallhaven API] [{}] 解析成功，获取到 {} 张壁纸", search_tag, wallhaven_response.data.len());

        let wallpapers: Vec<OnlineWallpaper> = wallhaven_response
            .data
            .into_iter()
            .map(OnlineWallpaper::from)
            .collect();

        let last_page = wallhaven_response
            .meta
            .as_ref()
            .map(|m| page as u64 >= m.last_page)
            .unwrap_or(false);

        let total_pages = wallhaven_response
            .meta
            .as_ref()
            .map(|m| m.last_page as usize)
            .unwrap_or(0);

        Ok((wallpapers, last_page, total_pages))
    }

    pub async fn get_wallpaper(&self, id: &str) -> Result<OnlineWallpaper, String> {
        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        let url = format!("{}/w/{}", BASE_URL, id);

        println!("[Wallhaven API] [ID:{}] 获取壁纸详情 - URL: {}", id, url);

        // 使用重试机制执行请求
        let text = Self::retry_with_backoff(
            id,
            "获取壁纸详情",
            3, // 最多重试3次
            || {
                let client = self.client.clone();
                let url = url.clone();
                let id = id.to_string();
                async move {
                    let response = client.get(&url).send().await
                        .map_err(|e| {
                            println!("[Wallhaven API] [ID:{}] 请求失败: {}", id, e);
                            format!("请求失败: {}", e)
                        })?;

                    println!("[Wallhaven API] [ID:{}] 响应状态: {}", id, response.status());

                    if !response.status().is_success() {
                        return Err(format!("API返回错误: {}", response.status()));
                    }

                    response.text().await
                        .map_err(|e| {
                            println!("[Wallhaven API] [ID:{}] 读取响应失败: {}", id, e);
                            format!("读取响应失败: {}", e)
                        })
                }
            },
        ).await?;

        let wallhaven_response: WallhavenResponse<WallhavenWallpaperData> =
            serde_json::from_str(&text).map_err(|e| {
                println!("[Wallhaven API] [ID:{}] JSON解析失败: {}", id, e);
                format!("解析JSON失败: {}", e)
            })?;

        println!("[Wallhaven API] [ID:{}] 解析成功，获取壁纸: {}",
            id, wallhaven_response.data.path);

        Ok(OnlineWallpaper::from(wallhaven_response.data))
    }
}

impl Default for WallhavenService {
    fn default() -> Self {
        Self::new(None, None)
    }
}