// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线图源抽象层
//!
//! 定义多图源的统一接口：[`WallpaperSource`] trait。每个在线图源（如 Wallhaven）
//! 实现该 trait 并经 [`create_source`] 工厂创建；上层（async_task/UI）只依赖本模块
//! 的类型与 trait，不再直接绑定具体图源实现，为后续接入新图源打地基。
//!
//! 当前阶段约定：
//! - [`SearchQuery`] 的筛选字段沿用 Wallhaven 的参数形状（Sorting/ColorOption/
//!   TimeRange，见 [`filters`]），另含 Basic 档图源共用的筛选字段
//!   （[`SourceColor`] 色板、[`PixabayCategory`] 分类、editors_choice），
//!   由各图源 service 层实现自己的映射层，无能力的图源忽略对应字段
//! - 壁纸条目 [`OnlineWallpaper`] 已图源无关，携带 [`SourceKind`] 标识来源，
//!   文件命名规则经其 [`OnlineWallpaper::download_file_name`] 按来源分发

pub mod filters;
pub mod http_log;
pub mod pixabay_category;
pub mod source_color;
pub mod types;

use std::future::Future;
use std::pin::Pin;

use crate::services::bing::BingService;
use crate::services::danbooru::DanbooruService;
use crate::services::pixabay::PixabayService;
use crate::services::request_context::RequestContext;
use crate::services::unsplash::UnsplashService;
use crate::services::wallhaven::WallhavenService;

pub use filters::{ColorOption, Sorting, TimeRange};
pub use pixabay_category::PixabayCategory;
pub use source_color::SourceColor;
pub use types::OnlineWallpaper;

/// trait 方法返回的 boxed Future（保持 trait 对象安全，可用 `dyn WallpaperSource`）
pub type SourceFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// 在线图源标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Wallhaven,
    /// Bing 每日一图（免密钥，无筛选维度）
    Bing,
    /// Pixabay 图库（需密钥，支持关键词/方向/排序/分类/颜色/精选）
    Pixabay,
    /// Unsplash 摄影图库（需密钥，支持关键词/方向/排序）
    Unsplash,
    /// Danbooru 图库（免密钥，支持 tags 关键词/分级/排序）
    Danbooru,
}

/// 图源搜索能力（在线页筛选栏按能力分档渲染）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchCapability {
    /// 完整筛选：关键词 + 分类/纯净度/分辨率/比例/颜色/排序/时间范围
    Full,
    /// 基础筛选：关键词 + 排序，其余维度由图源 service 层映射或忽略
    Basic,
    /// 无搜索维度：仅翻阅固定图集
    Bare,
}

impl SourceKind {
    /// 全部可用图源（设置页图源下拉顺序）
    pub fn all() -> [SourceKind; 5] {
        [
            SourceKind::Wallhaven,
            SourceKind::Bing,
            SourceKind::Pixabay,
            SourceKind::Unsplash,
            SourceKind::Danbooru,
        ]
    }

    /// 从配置取值解析图源；未知取值回退 Wallhaven（解析不会失败）
    pub fn parse(s: &str) -> Self {
        match s {
            "bing" => SourceKind::Bing,
            "pixabay" => SourceKind::Pixabay,
            "unsplash" => SourceKind::Unsplash,
            "danbooru" => SourceKind::Danbooru,
            _ => SourceKind::Wallhaven,
        }
    }

    /// 图源唯一标识（用于日志、配置与持久化区分）
    pub fn id(self) -> &'static str {
        match self {
            SourceKind::Wallhaven => "wallhaven",
            SourceKind::Bing => "bing",
            SourceKind::Pixabay => "pixabay",
            SourceKind::Unsplash => "unsplash",
            SourceKind::Danbooru => "danbooru",
        }
    }

    /// 图源搜索能力（筛选栏渲染档位，新增图源时在此声明）
    pub fn search_capability(self) -> SearchCapability {
        match self {
            SourceKind::Wallhaven => SearchCapability::Full,
            SourceKind::Bing => SearchCapability::Bare,
            SourceKind::Pixabay | SourceKind::Unsplash | SourceKind::Danbooru => {
                SearchCapability::Basic
            }
        }
    }

    /// 按图源从配置解析 API Key（未配置返回 None）
    ///
    /// 检索入口（UI 在线搜索、定时切换）统一经本方法取 key，避免图源间
    /// 错配（如把 Wallhaven 的 key 传给 Pixabay）
    pub fn api_key_from_config(self, config: &crate::utils::config::Config) -> Option<String> {
        match self {
            SourceKind::Wallhaven => non_empty(config.wallhaven.api_key.clone()),
            SourceKind::Pixabay => non_empty(config.pixabay.api_key.clone()),
            SourceKind::Unsplash => non_empty(config.unsplash.api_key.clone()),
            SourceKind::Danbooru | SourceKind::Bing => None,
        }
    }

    /// 生成下载文件名（扩展名取自 file_type 的 mime 子类型，按图源命名规则）
    ///
    /// 各下载/收藏/落库调用点统一经由本方法获取文件名，图源命名规则变更时只需调整此处
    pub fn download_file_name(self, item_id: &str, file_type: &str) -> String {
        let ext = file_type.split('/').next_back().unwrap_or("jpg");
        match self {
            SourceKind::Wallhaven => format!("wallhaven-{}.{}", item_id, ext),
            SourceKind::Bing => format!("bing-{}.{}", item_id, ext),
            SourceKind::Pixabay => format!("pixabay-{}.{}", item_id, ext),
            SourceKind::Unsplash => format!("unsplash-{}.{}", item_id, ext),
            SourceKind::Danbooru => format!("danbooru-{}.{}", item_id, ext),
        }
    }
}

/// 空串视为未配置
fn non_empty(key: String) -> Option<String> {
    if key.is_empty() { None } else { Some(key) }
}

/// 图源网络配置（构建图源实例所需，与具体筛选参数无关）
#[derive(Debug, Clone)]
pub struct SourceConfig {
    pub api_key: Option<String>,
    pub proxy: Option<String>,
    pub proxy_enabled: bool,
    pub use_env_fallback: bool,
}

/// 搜索筛选参数（当前沿用 Wallhaven 参数形状，见模块文档）
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// 分类位掩码（100=通用, 010=动漫, 001=人物）
    pub categories: u32,
    pub sorting: Sorting,
    /// 纯净度位掩码（100=SFW, 010=Sketchy, 001=NSFW）
    pub purities: u32,
    /// Wallhaven 29 色板（Full 档）
    pub color: ColorOption,
    /// Basic 档图源共用色板（Pixabay/Unsplash，各自映射；Wallhaven 忽略）
    pub source_color: SourceColor,
    /// Pixabay 内容分类（None 不限；其余图源忽略）
    pub pixabay_category: Option<String>,
    /// Pixabay 精选过滤（其余图源忽略）
    pub editors_choice: bool,
    pub query: String,
    /// 时间范围（仅 toplist 排序生效）
    pub time_range: TimeRange,
    pub atleast: Option<String>,
    pub resolutions: Option<String>,
    pub ratios: Option<String>,
    /// 页码（从 1 开始）
    pub page: usize,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub wallpapers: Vec<OnlineWallpaper>,
    /// 是否最后一页
    pub is_last: bool,
    /// 总页数（图源未返回时为 0）
    pub total_pages: usize,
    pub current_page: usize,
}

/// 在线图源统一接口
pub trait WallpaperSource: Send + Sync {
    /// 图源标识
    fn kind(&self) -> SourceKind;

    /// 搜索壁纸
    ///
    /// # 参数
    /// - `query`: 搜索筛选参数
    /// - `context`: 请求上下文（用于取消操作）
    fn search<'a>(
        &'a self,
        query: &'a SearchQuery,
        context: &'a RequestContext,
    ) -> SourceFuture<'a, Result<SearchResult, String>>;
}

/// 按图源标识创建图源实例（新图源接入时在此补充匹配分支）
pub fn create_source(kind: SourceKind, config: SourceConfig) -> Box<dyn WallpaperSource> {
    match kind {
        SourceKind::Wallhaven => Box::new(WallhavenService::new(
            config.api_key,
            config.proxy,
            config.proxy_enabled,
            config.use_env_fallback,
        )),
        // Bing 免密钥，api_key 不参与
        SourceKind::Bing => Box::new(BingService::new(
            config.proxy,
            config.proxy_enabled,
            config.use_env_fallback,
        )),
        // Pixabay 检索需 API Key（未配置时 search() 直接返回错误）
        SourceKind::Pixabay => Box::new(PixabayService::new(
            config.api_key,
            config.proxy,
            config.proxy_enabled,
            config.use_env_fallback,
        )),
        // Unsplash 检索需 API Key（未配置时 search() 直接返回错误）
        SourceKind::Unsplash => Box::new(UnsplashService::new(
            config.api_key,
            config.proxy,
            config.proxy_enabled,
            config.use_env_fallback,
        )),
        // Danbooru 免密钥，api_key 不参与
        SourceKind::Danbooru => Box::new(DanbooruService::new(
            config.proxy,
            config.proxy_enabled,
            config.use_env_fallback,
        )),
    }
}
