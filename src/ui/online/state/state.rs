// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::request_context::RequestContext;
use crate::services::wallhaven;
use iced::widget::image::Handle;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// 壁纸加载状态
#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    /// 加载中
    Loading,
    /// 缩略图已加载
    ThumbLoaded(wallhaven::OnlineWallpaper, Handle),
    /// 已加载
    Loaded(wallhaven::OnlineWallpaper),
}

/// 分页信息，记录每页的结束索引和对应的页码
#[derive(Debug, Clone)]
pub struct PageInfo {
    /// 该页最后一个壁纸的索引（不包含）
    pub end_index: usize,
    /// 该页的页码（从1开始）
    pub page_num: usize,
}

/// 分辨率筛选模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    /// 全部：不携带分辨率参数
    All,
    /// 至少：atleast 参数
    AtLeast,
    /// 精确：resolutions 参数
    Exactly,
}

/// 在线壁纸页面状态
#[derive(Debug)]
pub struct OnlineState {
    /// 壁纸列表（带加载状态）
    pub wallpapers: Vec<WallpaperLoadStatus>,
    /// 壁纸原始数据
    pub wallpapers_data: Vec<wallhaven::OnlineWallpaper>,
    /// 是否正在加载页面
    pub loading_page: bool,
    /// 当前页码
    pub current_page: usize,
    /// 每页大小
    pub page_size: usize,
    /// 总壁纸数
    pub total_count: usize,
    /// 总页数
    pub total_pages: usize,
    /// 模态窗口是否可见
    pub modal_visible: bool,
    /// 当前图片索引
    pub current_image_index: usize,
    /// 模态窗口图片句柄
    pub modal_image_handle: Option<Handle>,
    // 筛选条件
    /// 分类（位掩码：100(4)表示通用，010(2)表示动漫，001(1)表示人物）
    pub categories: u32,
    /// 排序方式
    pub sorting: wallhaven::Sorting,
    /// 纯净度（位掩码：100(4)表示安全，010(2)表示轻微，001(1)表示成人）
    pub purities: u32,
    /// 分辨率
    pub resolution: wallhaven::Resolution,
    /// 比例
    pub ratio: wallhaven::Ratio,
    /// 颜色
    pub color: wallhaven::ColorOption,
    /// 时间范围
    pub time_range: wallhaven::TimeRange,
    /// 搜索文本
    pub search_text: String,
    /// 是否是最后一页
    pub last_page: bool,
    /// 是否已加载过数据
    pub has_loaded: bool,
    /// 分页信息列表
    pub page_info: Vec<PageInfo>,
    /// 颜色选择器展开状态
    pub color_picker_expanded: bool,
    // 分辨率筛选器状态
    /// 分辨率选择器展开状态
    pub resolution_picker_expanded: bool,
    /// 分辨率筛选模式
    pub resolution_mode: ResolutionMode,
    /// Exactly模式下选中的分辨率列表
    pub selected_resolutions: Vec<wallhaven::Resolution>,
    /// AtLeast模式下选中的分辨率
    pub atleast_resolution: Option<wallhaven::Resolution>,
    // 比例筛选器状态
    /// 比例选择器展开状态
    pub ratio_picker_expanded: bool,
    /// 选中的比例列表
    pub selected_ratios: Vec<wallhaven::AspectRatio>,
    /// 选中"全部横屏"
    pub ratio_landscape_selected: bool,
    /// 选中"全部竖屏"
    pub ratio_portrait_selected: bool,
    /// 选中"全部"
    pub ratio_all_selected: bool,
    /// 排序方式选择器展开状态
    pub sorting_picker_expanded: bool,
    /// 时间范围选择器展开状态
    pub time_range_picker_expanded: bool,
    /// 请求上下文，用于取消正在进行的请求
    pub request_context: RequestContext,
    /// 待设置壁纸的文件名（用于在下载完成后自动设置壁纸）
    pub pending_set_wallpaper_filename: Option<String>,
    /// 模态窗口图片下载取消令牌
    pub modal_download_cancel_token: Option<Arc<AtomicBool>>,
    /// 模态窗口图片下载进度
    pub modal_download_progress: f32,
    /// 模态窗口已下载字节数
    pub modal_downloaded_bytes: u64,
    /// 模态窗口总字节数
    pub modal_total_bytes: u64,
    /// 缩略图加载任务的取消令牌列表
    pub thumb_load_cancel_tokens: Vec<Arc<AtomicBool>>,
}

impl Default for OnlineState {
    fn default() -> Self {
        Self {
            wallpapers: Vec::new(),
            wallpapers_data: Vec::new(),
            loading_page: false,
            current_page: 1,
            page_size: 24,
            total_count: 0,
            total_pages: 0,
            modal_visible: false,
            current_image_index: 0,
            modal_image_handle: None,
            categories: 0b100, // 默认只选择通用
            sorting: wallhaven::Sorting::DateAdded,
            purities: 0b100, // 默认只选择安全
            resolution: wallhaven::Resolution::Any,
            ratio: wallhaven::Ratio::Any,
            color: wallhaven::ColorOption::Any,
            time_range: wallhaven::TimeRange::Month,
            search_text: String::new(),
            last_page: false,
            has_loaded: false,
            page_info: Vec::new(),
            color_picker_expanded: false,
            resolution_picker_expanded: false,
            resolution_mode: ResolutionMode::All,
            selected_resolutions: Vec::new(),
            atleast_resolution: None,
            ratio_picker_expanded: false,
            selected_ratios: Vec::new(),
            ratio_landscape_selected: false,
            ratio_portrait_selected: false,
            ratio_all_selected: false,
            sorting_picker_expanded: false,
            time_range_picker_expanded: false,
            request_context: RequestContext::new(),
            pending_set_wallpaper_filename: None,
            modal_download_cancel_token: None,
            modal_download_progress: 0.0,
            modal_downloaded_bytes: 0,
            modal_total_bytes: 0,
            thumb_load_cancel_tokens: Vec::new(),
        }
    }
}
