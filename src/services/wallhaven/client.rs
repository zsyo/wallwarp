//! Wallhaven HTTP 客户端
//!
//! 处理 HTTP 请求和重试逻辑

use crate::services::request_context::RequestContext;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;

const BASE_URL: &str = "https://wallhaven.cc/api/v1";

/// Wallhaven HTTP 客户端
pub struct WallhavenClient {
    api_key: Option<String>,
    client: reqwest::Client,
}

impl WallhavenClient {
    /// 创建新的 Wallhaven 客户端
    ///
    /// # 参数
    /// - `api_key`: 可选的 API 密钥
    /// - `proxy`: 可选的代理 URL
    pub fn new(api_key: Option<String>, proxy: Option<String>) -> Self {
        let client = if let Some(proxy_url) = proxy {
            if !proxy_url.is_empty() {
                debug!("[WallhavenClient] 尝试创建代理客户端，代理URL: {}", proxy_url);
                match reqwest::Proxy::all(&proxy_url) {
                    Ok(p) => {
                        debug!("[WallhavenClient] Proxy::all 成功");
                        match reqwest::Client::builder().proxy(p).build() {
                            Ok(http_client) => {
                                debug!("[WallhavenClient] HTTP客户端创建成功");
                                http_client
                            }
                            Err(e) => {
                                warn!("[WallhavenClient] HTTP客户端创建失败: {}，回退到无代理", e);
                                reqwest::Client::new()
                            }
                        }
                    }
                    Err(e) => {
                        warn!("[WallhavenClient] Proxy::all 失败: {}，回退到无代理", e);
                        reqwest::Client::new()
                    }
                }
            } else {
                debug!("[WallhavenClient] 代理URL为空，使用无代理客户端");
                reqwest::Client::new()
            }
        } else {
            debug!("[WallhavenClient] 无代理配置，使用无代理客户端");
            reqwest::Client::new()
        };

        Self { api_key, client }
    }

    /// 执行带重试的 HTTP 请求
    ///
    /// # 参数
    /// - `identifier`: 请求标识符（用于日志）
    /// - `operation_name`: 操作名称
    /// - `max_retries`: 最大重试次数
    /// - `operation`: 要执行的异步操作
    ///
    /// # 返回
    /// 返回操作结果或错误信息
    pub async fn retry_with_backoff<F, T, Fut>(
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
                        info!("[Wallhaven API] [{}] 重试第 {} 次成功", identifier, attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    last_error = e;
                    if attempt < max_retries {
                        warn!(
                            "[Wallhaven API] [{}] 第 {} 次尝试失败，将在1秒后重试: {}",
                            identifier,
                            attempt + 1,
                            last_error
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    } else {
                        error!(
                            "[Wallhaven API] [{}] 所有重试失败，共尝试 {} 次",
                            identifier,
                            max_retries + 1
                        );
                    }
                }
            }
        }

        Err(last_error)
    }

    /// 构建搜索 URL
    ///
    /// # 参数
    /// - `page`: 页码
    /// - `categories`: 分类位掩码
    /// - `sorting`: 排序方式
    /// - `purities`: 纯净度位掩码
    /// - `color`: 颜色选项
    /// - `query`: 搜索关键词
    /// - `top_range`: 时间范围（仅用于 toplist 排序）
    /// - `atleast`: 最小分辨率（atleast参数）
    /// - `resolutions`: 精确分辨率列表（resolutions参数，逗号分隔）
    /// - `ratios`: 比例列表（ratios参数，逗号分隔）
    ///
    /// # 返回
    /// 返回完整的搜索 URL
    pub fn build_search_url(
        &self,
        page: usize,
        categories: u32,
        sorting: &str,
        purities: u32,
        color: &str,
        query: &str,
        top_range: &str,
        atleast: Option<&str>,
        resolutions: Option<&str>,
        ratios: Option<&str>,
    ) -> String {
        let mut url = format!("{}/search?page={}", BASE_URL, page);

        // 添加分类参数（使用位掩码）
        let categories_str = format!("{:03b}", categories);
        url.push_str(&format!("&categories={}", categories_str));

        // 添加纯净度参数（使用位掩码）
        let purity_str = format!("{:03b}", purities);
        url.push_str(&format!("&purity={}", purity_str));

        // 添加排序参数
        url.push_str(&format!("&sorting={}", sorting));

        // 默认使用倒序
        url.push_str("&order=desc");

        // 添加 topRange 参数（仅当 sorting 为 toplist 时生效）
        if sorting == "toplist" && top_range != "any" {
            url.push_str(&format!("&topRange={}", top_range));
        }

        // 添加颜色参数
        if color != "any" {
            url.push_str(&format!("&colors={}", color));
        }

        // 添加分辨率参数
        if let Some(atleast_res) = atleast {
            url.push_str(&format!("&atleast={}", atleast_res));
        }

        if let Some(res_list) = resolutions {
            url.push_str(&format!("&resolutions={}", res_list));
        }

        // 添加比例参数
        if let Some(ratio_list) = ratios {
            url.push_str(&format!("&ratios={}", ratio_list));
        }

        // 添加搜索查询
        if !query.is_empty() {
            url.push_str(&format!("&q={}", urlencoding::encode(query)));
        }

        // 添加API密钥（如果提供）
        if let Some(ref key) = self.api_key {
            url.push_str(&format!("&apikey={}", key));
        }

        url
    }

    /// 执行 HTTP GET 请求
    ///
    /// # 参数
    /// - `url`: 请求 URL
    /// - `identifier`: 请求标识符（用于日志）
    /// - `context`: 请求上下文（用于取消操作）
    ///
    /// # 返回
    /// 返回响应文本或错误信息
    pub async fn get(
        &self,
        url: String,
        identifier: String,
        context: &RequestContext,
    ) -> Result<String, String> {
        Self::retry_with_backoff(&identifier, "HTTP GET", 3, || {
            let client = self.client.clone();
            let url = url.clone();
            let identifier = identifier.clone();
            let context = context.clone();
            async move {
                // 每次重试前检查取消状态
                if let Some(()) = context.check_cancelled() {
                    return Err("请求已取消".to_string());
                }

                let response = client.get(&url).send().await.map_err(|e| {
                    error!("[Wallhaven API] [{}] 请求失败: {}", identifier, e);
                    format!("请求失败: {}", e)
                })?;

                debug!("[Wallhaven API] [{}] 响应状态: {}", identifier, response.status());

                if !response.status().is_success() {
                    return Err(format!("API返回错误: {}", response.status()));
                }

                response.text().await.map_err(|e| {
                    error!("[Wallhaven API] [{}] 读取响应失败: {}", identifier, e);
                    format!("读取响应失败: {}", e)
                })
            }
        })
        .await
    }

    /// 获取客户端引用
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Default for WallhavenClient {
    fn default() -> Self {
        Self::new(None, None)
    }
}