// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Pixabay 服务层
//!
//! 实现图源抽象的 [`WallpaperSource`] trait：把统一筛选参数映射为 Pixabay
//! 接口参数（关键词/方向/排序/分类/颜色/精选，其余维度忽略），并把检索
//! 结果映射为图源无关的 [`OnlineWallpaper`]

use super::client::{self, PixabayHit};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, Sorting, SourceColor, SourceFuture, SourceKind,
    WallpaperSource,
};
use crate::utils::helpers::{format_resolution, simplified_ratio};
use tracing::{error, info, trace};

/// Pixabay 图源服务
pub struct PixabayService {
    client: reqwest::Client,
    api_key: Option<String>,
}

impl PixabayService {
    /// 创建新的 Pixabay 服务
    ///
    /// # 参数
    /// - `api_key`: Pixabay API 密钥（未配置时检索直接返回错误）
    /// - `proxy`: 可选的代理 URL
    /// - `proxy_enabled`: 代理是否启用
    /// - `use_env_fallback`: 是否使用环境变量作为回退
    pub fn new(
        api_key: Option<String>,
        proxy: Option<String>,
        proxy_enabled: bool,
        use_env_fallback: bool,
    ) -> Self {
        Self {
            client: proxy::create_proxy_client(proxy, proxy_enabled, use_env_fallback),
            api_key,
        }
    }

    /// 统一排序到 Pixabay order 参数的降级映射
    ///
    /// Pixabay 仅支持 ec/latest/popular：DateAdded->latest；
    /// Relevance/Hot->ec（配 editors_choice 过滤近似精选）；
    /// Random/Views/Favorites/TopList 无对应能力，退化 popular
    fn map_order(sorting: Sorting) -> &'static str {
        match sorting {
            Sorting::DateAdded => "latest",
            Sorting::Relevance | Sorting::Hot => "ec",
            Sorting::Random | Sorting::Views | Sorting::Favorites | Sorting::TopList => "popular",
        }
    }

    /// 统一比例参数映射横竖方向（Pixabay 无具体比例能力，仅有 horizontal/vertical）
    ///
    /// ratios 为逗号分隔串（landscape/portrait/具体比例值）；具体比例值
    /// 在 Pixabay 侧忽略，只把 landscape/portrait 归并为方向
    fn map_orientation(ratios: &Option<String>) -> Option<&'static str> {
        let ratios = ratios.as_ref()?;
        let has_landscape = ratios.split(',').any(|r| r.trim() == "landscape");
        let has_portrait = ratios.split(',').any(|r| r.trim() == "portrait");
        match (has_landscape, has_portrait) {
            (true, false) => Some("horizontal"),
            (false, true) => Some("vertical"),
            // 都选/都未选/仅具体比例均不限制方向
            _ => None,
        }
    }

    /// 统一 atleast 分辨率提取最小宽度（形如 "1920x1080"，取宽边）
    fn map_min_width(atleast: &Option<String>) -> Option<u32> {
        atleast
            .as_ref()
            .and_then(|s| s.split(['x', 'X']).next())
            .and_then(|w| w.trim().parse().ok())
    }

    /// 统一色板映射 Pixabay colors 参数
    ///
    /// Pixabay 的取值为 grayscale/transparent/red/orange/yellow/green/
    /// turquoise/blue/lilac/pink/white/gray/black/brown；灰与黑白在
    /// Pixabay 为两个独立取值，本枚举 Gray 语义为黑白（grayscale），
    /// Any 不限制
    fn map_color(source_color: SourceColor) -> Option<&'static str> {
        match source_color {
            SourceColor::Any => None,
            SourceColor::Gray => Some("grayscale"),
            SourceColor::Red => Some("red"),
            SourceColor::Orange => Some("orange"),
            SourceColor::Yellow => Some("yellow"),
            SourceColor::Green => Some("green"),
            SourceColor::Teal => Some("turquoise"),
            SourceColor::Blue => Some("blue"),
            SourceColor::Purple => Some("lilac"),
            SourceColor::Pink => Some("pink"),
            SourceColor::Brown => Some("brown"),
            SourceColor::Black => Some("black"),
            SourceColor::White => Some("white"),
        }
    }

    /// 将 Pixabay 条目映射为图源无关的在线壁纸条目
    ///
    /// 接口不返回比例与颜色信息：ratio 由宽高约分生成，colors 留空；
    /// file_type 无法从接口获取，由原图直链扩展名推断
    fn to_online_wallpaper(hit: PixabayHit) -> Option<OnlineWallpaper> {
        // 无原图直链（矢量条目）或尺寸无效时跳过
        if hit.image_url.is_empty() || hit.image_width == 0 || hit.image_height == 0 {
            return None;
        }

        let file_type = hit
            .image_url
            .rsplit('.')
            .next()
            .map(|ext| format!("image/{}", ext.to_lowercase()))
            .unwrap_or_else(|| "image/jpeg".to_string());
        let image_url = hit.image_url;

        Some(OnlineWallpaper {
            id: hit.id.to_string(),
            url: hit.page_url,
            path: image_url.clone(),
            thumb_large: hit.large_image_url,
            thumb_original: image_url,
            thumb_small: hit.webformat_url,
            width: hit.image_width,
            height: hit.image_height,
            resolution: format_resolution(hit.image_width, hit.image_height),
            ratio: simplified_ratio(hit.image_width, hit.image_height),
            file_size: hit.image_size,
            file_type,
            category: "general".to_string(),
            purity: "sfw".to_string(),
            views: hit.views as u32,
            favorites: hit.likes as u32,
            colors: Vec::new(),
            source: SourceKind::Pixabay,
            image_handle: None,
        })
    }
}

impl WallpaperSource for PixabayService {
    fn kind(&self) -> SourceKind {
        SourceKind::Pixabay
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // Pixabay 所有检索都需 API Key，未配置时直接报错
            let Some(key) = self.api_key.as_deref() else {
                error!(
                    "[Pixabay API] [page{}] 未配置 API Key，跳过检索",
                    query.page
                );
                return Err("未配置 Pixabay API Key，请在设置页填写".to_string());
            };

            // 精选过滤：显式勾选，或排序本身即精选语义（Relevance/Hot 映射 order=ec）
            let editors_choice =
                query.editors_choice || matches!(query.sorting, Sorting::Relevance | Sorting::Hot);
            let params = client::SearchParams {
                key,
                query: &query.query,
                page: query.page,
                orientation: Self::map_orientation(&query.ratios),
                min_width: Self::map_min_width(&query.atleast),
                order: Self::map_order(query.sorting),
                category: query.pixabay_category.as_deref(),
                color: Self::map_color(query.source_color),
                editors_choice,
            };

            trace!(
                "[Pixabay API] [page{}] 筛选映射: sorting={}->order={}, ratios={:?}->orientation={:?}, atleast={:?}->min_width={:?}, category={:?}, source_color={}->colors={:?}, editors_choice={}",
                query.page,
                query.sorting.value(),
                Self::map_order(query.sorting),
                query.ratios,
                Self::map_orientation(&query.ratios),
                query.atleast,
                Self::map_min_width(&query.atleast),
                query.pixabay_category,
                query.source_color.value(),
                Self::map_color(query.source_color),
                editors_choice,
            );

            let response = client::search(&self.client, params, context).await?;

            let wallpapers: Vec<OnlineWallpaper> = response
                .hits
                .into_iter()
                .filter_map(Self::to_online_wallpaper)
                .collect();

            info!(
                "[Pixabay API] [page{}] 关键词: {} 返回 {} 张壁纸",
                query.page,
                if query.query.is_empty() {
                    "(无)"
                } else {
                    &query.query
                },
                wallpapers.len()
            );

            // 空页或已到总页数/接口命中上限即视为末页
            let total_pages = client::total_pages(response.total_hits);
            let is_last = wallpapers.is_empty() || (total_pages > 0 && query.page >= total_pages);

            Ok(SearchResult {
                wallpapers,
                is_last,
                total_pages,
                current_page: query.page,
            })
        })
    }
}
