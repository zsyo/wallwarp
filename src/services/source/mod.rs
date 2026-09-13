// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线图源抽象层
//!
//! 定义多图源的统一接口：[`WallpaperSource`] trait。每个在线图源（如 Wallhaven）
//! 实现该 trait 并经 [`create_source`] 工厂创建；上层（async_task/UI）只依赖本模块
//! 的类型与 trait，不再直接绑定具体图源实现，为后续接入新图源打地基。
//!
//! 当前阶段约定：
//! - [`SearchQuery`] 的筛选字段沿用 Wallhaven 的参数形状（Sorting/ColorOption/
//!   TimeRange，见 [`filters`]），接入第二个图源时再演进为图源无关的筛选模型，
//!   由各图源实现自己的映射层
//! - 壁纸条目 [`OnlineWallpaper`] 已图源无关，携带 [`SourceKind`] 标识来源，
//!   文件命名规则经其 [`OnlineWallpaper::download_file_name`] 按来源分发

pub mod filters;
pub mod types;

use std::future::Future;
use std::pin::Pin;

use crate::services::request_context::RequestContext;
use crate::services::wallhaven::WallhavenService;

pub use filters::{ColorOption, Sorting, TimeRange};
pub use types::OnlineWallpaper;

/// trait 方法返回的 boxed Future（保持 trait 对象安全，可用 `dyn WallpaperSource`）
pub type SourceFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// 在线图源标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Wallhaven,
}

impl SourceKind {
    /// 图源唯一标识（用于日志、配置与持久化区分）
    pub fn id(self) -> &'static str {
        match self {
            SourceKind::Wallhaven => "wallhaven",
        }
    }

    /// 生成下载文件名（扩展名取自 file_type 的 mime 子类型，按图源命名规则）
    ///
    /// 各下载/收藏/落库调用点统一经由本方法获取文件名，图源命名规则变更时只需调整此处
    pub fn download_file_name(self, item_id: &str, file_type: &str) -> String {
        let ext = file_type.split('/').next_back().unwrap_or("jpg");
        match self {
            SourceKind::Wallhaven => format!("wallhaven-{}.{}", item_id, ext),
        }
    }
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
    pub color: ColorOption,
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
    }
}
