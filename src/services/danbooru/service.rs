// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Danbooru 服务层
//!
//! 实现图源抽象的 [`WallpaperSource`] trait：把统一筛选参数映射为 Danbooru
//! tags 查询（关键词 + rating 分级 + order metatag），并把检索结果映射为
//! 图源无关的 [`OnlineWallpaper`]

use super::client::{self, DanbooruPost};
use crate::services::proxy;
use crate::services::request_context::RequestContext;
use crate::services::source::{
    OnlineWallpaper, SearchQuery, SearchResult, Sorting, SourceFuture, SourceKind, WallpaperSource,
};
use crate::utils::helpers::{format_resolution, simplified_ratio};
use tracing::{info, trace};

/// Danbooru 图源服务
pub struct DanbooruService {
    client: reqwest::Client,
}

impl DanbooruService {
    /// 创建新的 Danbooru 服务
    ///
    /// # 参数
    /// - `proxy`: 可选的代理 URL
    /// - `proxy_enabled`: 代理是否启用
    /// - `use_env_fallback`: 是否使用环境变量作为回退
    pub fn new(proxy: Option<String>, proxy_enabled: bool, use_env_fallback: bool) -> Self {
        Self {
            client: proxy::create_proxy_client(proxy, proxy_enabled, use_env_fallback),
        }
    }

    /// 统一 purity 位掩码映射 Danbooru rating metatag
    ///
    /// 位语义（与 Wallhaven 一致）：0b100=SFW, 0b010=Sketchy, 0b001=NSFW。
    /// Danbooru 分级为 g(一般)/s(敏感)/q(存疑)/e(暴露)，sketchy 对应 s，
    /// nsfw 对应 q+e；任选了 NSFW 则不限制分级（返回全部）
    fn rating_tags(purities: u32) -> Option<&'static str> {
        let sfw = purities & 0b100 != 0;
        let sketchy = purities & 0b010 != 0;
        let nsfw = purities & 0b001 != 0;
        if nsfw {
            None
        } else if sfw && sketchy {
            Some("rating:g,s")
        } else if sketchy {
            Some("rating:s")
        } else {
            Some("rating:g")
        }
    }

    /// 统一排序映射 Danbooru order metatag
    ///
    /// DateAdded 不加 order（接口默认按上传时间新→旧）；
    /// Relevance/Views/TopList/Hot/Random 无精确对应，退化为 order:score
    fn order_tag(sorting: Sorting) -> Option<&'static str> {
        match sorting {
            Sorting::DateAdded => None,
            Sorting::Favorites => Some("order:favcount"),
            Sorting::Relevance
            | Sorting::Views
            | Sorting::TopList
            | Sorting::Hot
            | Sorting::Random => Some("order:score"),
        }
    }

    /// 将 Danbooru 条目映射为图源无关的在线壁纸条目
    ///
    /// 接口无原生 resolution/ratio：分别由宽高格式化与约分生成
    fn to_online_wallpaper(post: DanbooruPost) -> Option<OnlineWallpaper> {
        // 原图缺失或尺寸无效时跳过（接口偶发返回待审核删除条目）
        if post.file_url.is_empty() || post.image_width == 0 || post.image_height == 0 {
            return None;
        }

        let thumb_large = if post.large_file_url.is_empty() {
            post.preview_file_url.clone()
        } else {
            post.large_file_url.clone()
        };

        let file_type = if post.file_ext.is_empty() {
            "image/jpeg".to_string()
        } else {
            format!("image/{}", post.file_ext)
        };

        let purity = match post.rating.as_str() {
            "s" => "sketchy",
            "q" | "e" => "nsfw",
            _ => "sfw",
        };

        let file_url = post.file_url;

        Some(OnlineWallpaper {
            id: post.id.to_string(),
            url: format!("{}/posts/{}", client::BASE_URL, post.id),
            path: file_url.clone(),
            thumb_large,
            thumb_original: file_url,
            thumb_small: post.preview_file_url,
            width: post.image_width,
            height: post.image_height,
            resolution: format_resolution(post.image_width, post.image_height),
            ratio: simplified_ratio(post.image_width, post.image_height),
            file_size: post.file_size,
            file_type,
            category: "general".to_string(),
            purity: purity.to_string(),
            views: post.score.max(0) as u32,
            favorites: post.fav_count.max(0) as u32,
            colors: Vec::new(),
            source: SourceKind::Danbooru,
            image_handle: None,
        })
    }
}

impl WallpaperSource for DanbooruService {
    fn kind(&self) -> SourceKind {
        SourceKind::Danbooru
    }

    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>> {
        Box::pin(async move {
            // tags 组装：关键词 + rating 分级 + 排序 metatag（空格分隔均为 AND）
            let mut tags: Vec<&str> = Vec::new();
            if !query.query.is_empty() {
                tags.push(&query.query);
            }
            if let Some(rating) = Self::rating_tags(query.purities) {
                tags.push(rating);
            }
            if let Some(order) = Self::order_tag(query.sorting) {
                tags.push(order);
            }
            let tags = tags.join(" ");

            trace!(
                "[Danbooru API] [page{}] 筛选映射: purities={:03b}->{}, sorting={}->{}",
                query.page,
                query.purities,
                Self::rating_tags(query.purities).unwrap_or("(不限)"),
                query.sorting.value(),
                Self::order_tag(query.sorting).unwrap_or("(默认)"),
            );

            let params = client::SearchParams {
                tags: &tags,
                page: query.page,
            };

            let posts = client::search(&self.client, params, context).await?;

            let wallpapers: Vec<OnlineWallpaper> = posts
                .into_iter()
                .filter_map(Self::to_online_wallpaper)
                .collect();

            info!(
                "[Danbooru API] [page{}] tags: {} 返回 {} 张壁纸",
                query.page,
                if tags.is_empty() { "(无)" } else { &tags },
                wallpapers.len()
            );

            // 接口无分页元信息，空页即末页
            let is_last = wallpapers.is_empty();

            Ok(SearchResult {
                wallpapers,
                is_last,
                total_pages: 0,
                current_page: query.page,
            })
        })
    }
}
