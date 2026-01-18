//! Wallhaven 服务层
//!
//! 提供 Wallhaven API 的高级接口

use super::client::WallhavenClient;
use super::models::{ColorOption, Sorting, TimeRange};
use super::types::{OnlineWallpaper, WallhavenResponse, WallpaperData};
use crate::services::request_context::RequestContext;
use tracing::debug;
use tracing::error;
use tracing::info;

/// Wallhaven 服务
pub struct WallhavenService {
    client: WallhavenClient,
}

impl WallhavenService {
    /// 创建新的 Wallhaven 服务
    ///
    /// # 参数
    /// - `api_key`: 可选的 API 密钥
    /// - `proxy`: 可选的代理 URL
    pub fn new(api_key: Option<String>, proxy: Option<String>) -> Self {
        Self {
            client: WallhavenClient::new(api_key, proxy),
        }
    }

    /// 搜索壁纸
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `categories`: 分类位掩码（100=通用, 010=动漫, 001=人物）
    /// - `sorting`: 排序方式
    /// - `purities`: 纯净度位掩码（100=SFW, 010=Sketchy, 001=NSFW）
    /// - `color`: 颜色选项
    /// - `query`: 搜索关键词
    /// - `time_range`: 时间范围（仅用于 toplist 排序）
    /// - `context`: 请求上下文（用于取消操作）
    ///
    /// # 返回
    /// 返回元组：(壁纸列表, 是否最后一页, 总页数, 当前页码)
    pub async fn search_wallpapers(
        &self,
        page: usize,
        categories: u32,
        sorting: Sorting,
        purities: u32,
        color: ColorOption,
        query: &str,
        time_range: TimeRange,
        context: &RequestContext,
    ) -> Result<(Vec<OnlineWallpaper>, bool, usize, usize), String> {
        // 检查是否已取消
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        // 再次检查是否已取消
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        // 构建搜索 URL
        let url = self.client.build_search_url(
            page,
            categories,
            sorting.value(),
            purities,
            color.value(),
            query,
            time_range.value(),
        );

        // 打印请求参数
        let search_tag = format!(
            "page{}_cat{:03b}_sort{:?}_purity{:03b}_color{}_tr{}_q{}",
            page,
            categories,
            sorting,
            purities,
            color.value(),
            time_range.value(),
            if query.is_empty() {
                "empty"
            } else {
                &query[..query.len().min(10)]
            }
        );
        info!("[Wallhaven API] [{}] 请求URL: {}", search_tag, url);

        // 执行请求
        let text = self.client.get(url, search_tag.clone(), context).await?;

        // 解析前检查取消状态
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        // 解析响应
        let wallhaven_response: WallhavenResponse<Vec<WallpaperData>> =
            serde_json::from_str(&text).map_err(|e| {
                error!("[Wallhaven API] [{}] JSON解析失败: {}", search_tag, e);
                format!("解析JSON失败: {}", e)
            })?;

        // 打印解析结果
        info!(
            "[Wallhaven API] [{}] 解析成功，获取到 {} 张壁纸",
            search_tag,
            wallhaven_response.data.len()
        );

        let wallpapers: Vec<OnlineWallpaper> =
            wallhaven_response.data.into_iter().map(OnlineWallpaper::from).collect();

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

        let current_page = wallhaven_response
            .meta
            .as_ref()
            .map(|m| m.current_page as usize)
            .unwrap_or(page);

        Ok((wallpapers, last_page, total_pages, current_page))
    }

    /// 获取单张壁纸详情
    ///
    /// # 参数
    /// - `id`: 壁纸 ID
    /// - `context`: 请求上下文（用于取消操作）
    ///
    /// # 返回
    /// 返回壁纸详情
    pub async fn get_wallpaper(
        &self,
        id: &str,
        context: &RequestContext,
    ) -> Result<OnlineWallpaper, String> {
        // 检查是否已取消
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        // 获取并发控制许可
        let _permit = crate::services::GLOBAL_CONCURRENCY_CONTROLLER.acquire().await;

        // 再次检查是否已取消
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        let url = format!("{}/w/{}", "https://wallhaven.cc/api/v1", id);

        debug!("[Wallhaven API] [ID:{}] 获取壁纸详情 - URL: {}", id, url);

        // 执行请求
        let text = self.client.get(url, id.to_string(), context).await?;

        // 解析前检查取消状态
        if let Some(()) = context.check_cancelled() {
            return Err("请求已取消".to_string());
        }

        // 解析响应
        let wallhaven_response: WallhavenResponse<WallpaperData> =
            serde_json::from_str(&text).map_err(|e| {
                error!("[Wallhaven API] [ID:{}] JSON解析失败: {}", id, e);
                format!("解析JSON失败: {}", e)
            })?;

        info!(
            "[Wallhaven API] [ID:{}] 解析成功，获取壁纸: {}",
            id, wallhaven_response.data.path
        );

        Ok(OnlineWallpaper::from(wallhaven_response.data))
    }

    /// 获取客户端引用
    pub fn client(&self) -> &WallhavenClient {
        &self.client
    }
}

impl Default for WallhavenService {
    fn default() -> Self {
        Self::new(None, None)
    }
}