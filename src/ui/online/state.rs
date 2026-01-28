// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::request_context::RequestContext;
use crate::services::wallhaven;
use crate::ui::async_tasks;
use crate::utils::config::Config;
use iced::widget::image::Handle;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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

impl OnlineState {
    /// 从配置文件加载筛选条件
    pub fn load_from_config(config: &Config) -> Self {
        let mut state = Self::default();

        // 加载分类（从字符串解析位掩码）
        state.categories = async_tasks::parse_category_bitmask(&config.wallhaven.category);

        // 加载纯净度（从字符串解析位掩码）
        state.purities = async_tasks::parse_purity_bitmask(&config.wallhaven.purity);

        // 如果 API Key 为空，移除 NSFW 选项
        if config.wallhaven.api_key.is_empty() {
            state.purities &= !wallhaven::Purity::NSFW.bit_value();
        }

        // 加载排序
        state.sorting = async_tasks::parse_sorting(&config.wallhaven.sorting);

        // 加载颜色
        state.color = async_tasks::parse_color(&config.wallhaven.color);

        // 加载时间范围
        state.time_range = async_tasks::parse_time_range(&config.wallhaven.top_range);

        // 加载分辨率模式
        state.resolution_mode = match config.wallhaven.resolution_mode.as_str() {
            "all" => ResolutionMode::All,
            "atleast" => ResolutionMode::AtLeast,
            "exactly" => ResolutionMode::Exactly,
            _ => ResolutionMode::All,
        };

        // 加载AtLeast分辨率
        state.atleast_resolution = if !config.wallhaven.atleast_resolution.is_empty() {
            match config.wallhaven.atleast_resolution.as_str() {
                "2560x1080" => Some(wallhaven::Resolution::R2560x1080),
                "2560x1440" => Some(wallhaven::Resolution::R2560x1440U),
                "3840x1600" => Some(wallhaven::Resolution::R3840x1600),
                "1280x720" => Some(wallhaven::Resolution::R1280x720),
                "1600x900" => Some(wallhaven::Resolution::R1600x900),
                "1920x1080" => Some(wallhaven::Resolution::R1920x1080),
                "3840x2160" => Some(wallhaven::Resolution::R3840x2160),
                "1280x800" => Some(wallhaven::Resolution::R1280x800),
                "1600x1000" => Some(wallhaven::Resolution::R1600x1000),
                "1920x1200" => Some(wallhaven::Resolution::R1920x1200),
                "2560x1600" => Some(wallhaven::Resolution::R2560x1600),
                "3840x2400" => Some(wallhaven::Resolution::R3840x2400),
                "1280x960" => Some(wallhaven::Resolution::R1280x960),
                "1600x1200" => Some(wallhaven::Resolution::R1600x1200_4_3),
                "1920x1440" => Some(wallhaven::Resolution::R1920x1440),
                "2560x1920" => Some(wallhaven::Resolution::R2560x1920),
                "3840x2880" => Some(wallhaven::Resolution::R3840x2880),
                "1280x1024" => Some(wallhaven::Resolution::R1280x1024),
                "1600x1280" => Some(wallhaven::Resolution::R1600x1280),
                "1920x1536" => Some(wallhaven::Resolution::R1920x1536),
                "2560x2048" => Some(wallhaven::Resolution::R2560x2048),
                "3840x3072" => Some(wallhaven::Resolution::R3840x3072),
                _ => None,
            }
        } else {
            None
        };

        // 加载Exactly分辨率列表
        state.selected_resolutions = if !config.wallhaven.resolutions.is_empty() {
            let valid_resolutions = [
                "2560x1080",
                "2560x1440",
                "3840x1600",
                "1280x720",
                "1600x900",
                "1920x1080",
                "3840x2160",
                "1280x800",
                "1600x1000",
                "1920x1200",
                "2560x1600",
                "3840x2400",
                "1280x960",
                "1600x1200",
                "1920x1440",
                "2560x1920",
                "3840x2880",
                "1280x1024",
                "1600x1280",
                "1920x1536",
                "2560x2048",
                "3840x3072",
            ];

            let res_list: Vec<wallhaven::Resolution> = config
                .wallhaven
                .resolutions
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if valid_resolutions.contains(&s) {
                        match s {
                            "2560x1080" => Some(wallhaven::Resolution::R2560x1080),
                            "2560x1440" => Some(wallhaven::Resolution::R2560x1440U),
                            "3840x1600" => Some(wallhaven::Resolution::R3840x1600),
                            "1280x720" => Some(wallhaven::Resolution::R1280x720),
                            "1600x900" => Some(wallhaven::Resolution::R1600x900),
                            "1920x1080" => Some(wallhaven::Resolution::R1920x1080),
                            "3840x2160" => Some(wallhaven::Resolution::R3840x2160),
                            "1280x800" => Some(wallhaven::Resolution::R1280x800),
                            "1600x1000" => Some(wallhaven::Resolution::R1600x1000),
                            "1920x1200" => Some(wallhaven::Resolution::R1920x1200),
                            "2560x1600" => Some(wallhaven::Resolution::R2560x1600),
                            "3840x2400" => Some(wallhaven::Resolution::R3840x2400),
                            "1280x960" => Some(wallhaven::Resolution::R1280x960),
                            "1600x1200" => Some(wallhaven::Resolution::R1600x1200_4_3),
                            "1920x1440" => Some(wallhaven::Resolution::R1920x1440),
                            "2560x1920" => Some(wallhaven::Resolution::R2560x1920),
                            "3840x2880" => Some(wallhaven::Resolution::R3840x2880),
                            "1280x1024" => Some(wallhaven::Resolution::R1280x1024),
                            "1600x1280" => Some(wallhaven::Resolution::R1600x1280),
                            "1920x1536" => Some(wallhaven::Resolution::R1920x1536),
                            "2560x2048" => Some(wallhaven::Resolution::R2560x2048),
                            "3840x3072" => Some(wallhaven::Resolution::R3840x3072),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            res_list
        } else {
            Vec::new()
        };

        // 加载比例列表和额外选项
        state.ratio_landscape_selected = false;
        state.ratio_portrait_selected = false;
        state.ratio_all_selected = config.wallhaven.ratios == "all";
        state.selected_ratios = Vec::new();

        if !state.ratio_all_selected && !config.wallhaven.ratios.is_empty() {
            let landscape_included = [
                wallhaven::AspectRatio::R16x9,
                wallhaven::AspectRatio::R16x10,
                wallhaven::AspectRatio::R21x9,
                wallhaven::AspectRatio::R32x9,
                wallhaven::AspectRatio::R48x9,
            ];
            let portrait_included = [
                wallhaven::AspectRatio::R9x16,
                wallhaven::AspectRatio::R10x16,
                wallhaven::AspectRatio::R9x18,
            ];

            let parts: Vec<&str> = config.wallhaven.ratios.split(',').collect();

            for part in parts {
                let part = part.trim();
                match part {
                    "landscape" => {
                        state.ratio_landscape_selected = true;
                    }
                    "portrait" => {
                        state.ratio_portrait_selected = true;
                    }
                    "16x9" => state.selected_ratios.push(wallhaven::AspectRatio::R16x9),
                    "16x10" => state.selected_ratios.push(wallhaven::AspectRatio::R16x10),
                    "21x9" => state.selected_ratios.push(wallhaven::AspectRatio::R21x9),
                    "32x9" => state.selected_ratios.push(wallhaven::AspectRatio::R32x9),
                    "48x9" => state.selected_ratios.push(wallhaven::AspectRatio::R48x9),
                    "9x16" => state.selected_ratios.push(wallhaven::AspectRatio::R9x16),
                    "10x16" => state.selected_ratios.push(wallhaven::AspectRatio::R10x16),
                    "9x18" => state.selected_ratios.push(wallhaven::AspectRatio::R9x18),
                    "1x1" => state.selected_ratios.push(wallhaven::AspectRatio::R1x1),
                    "3x2" => state.selected_ratios.push(wallhaven::AspectRatio::R3x2),
                    "4x3" => state.selected_ratios.push(wallhaven::AspectRatio::R4x3),
                    "5x4" => state.selected_ratios.push(wallhaven::AspectRatio::R5x4),
                    _ => {}
                }
            }

            if state.ratio_landscape_selected {
                state.selected_ratios.retain(|r| !landscape_included.contains(r));
            }
            if state.ratio_portrait_selected {
                state.selected_ratios.retain(|r| !portrait_included.contains(r));
            }
        }

        state.has_loaded = false;

        state
    }

    /// 保存当前筛选条件到配置文件
    pub fn save_to_config(&self, config: &mut Config) {
        config.wallhaven.category = format!("{:03b}", self.categories);
        config.wallhaven.purity = format!("{:03b}", self.purities);
        config.wallhaven.sorting = self.sorting.to_string();
        config.wallhaven.color = self.color.value().to_string();
        config.wallhaven.top_range = self.time_range.value().to_string();

        config.wallhaven.resolution_mode = match self.resolution_mode {
            ResolutionMode::All => "all".to_string(),
            ResolutionMode::AtLeast => "atleast".to_string(),
            ResolutionMode::Exactly => "exactly".to_string(),
        };

        config.wallhaven.atleast_resolution = if let Some(res) = self.atleast_resolution {
            res.value().to_string()
        } else {
            String::new()
        };

        config.wallhaven.resolutions = if !self.selected_resolutions.is_empty() {
            let res_list: Vec<String> = self
                .selected_resolutions
                .iter()
                .map(|r| r.value().to_string())
                .collect();
            res_list.join(",")
        } else {
            String::new()
        };

        let mut ratios_vec = Vec::new();

        if self.ratio_landscape_selected {
            ratios_vec.push("landscape".to_string());
        }

        if self.ratio_portrait_selected {
            ratios_vec.push("portrait".to_string());
        }

        for ratio in &self.selected_ratios {
            ratios_vec.push(ratio.value().to_string());
        }

        config.wallhaven.ratios = ratios_vec.join(",");

        config.save_to_file();
    }

    /// 获取分类API参数字符串
    pub fn get_categories_param(&self) -> String {
        format!("{:03b}", self.categories)
    }

    /// 获取纯净度API参数字符串
    pub fn get_purity_param(&self) -> String {
        format!("{:03b}", self.purities)
    }

    /// 检查是否需要加载下一页
    pub fn should_load_next_page(&self) -> bool {
        !self.last_page && !self.loading_page && self.has_loaded
    }

    /// 取消当前正在进行的请求，并创建一个新的请求上下文
    pub fn cancel_and_new_context(&mut self) {
        self.request_context.cancel();
        self.request_context = RequestContext::new();
    }

    /// 取消模态窗口图片下载
    pub fn cancel_modal_download(&mut self) {
        if let Some(cancel_token) = &self.modal_download_cancel_token {
            cancel_token.store(true, Ordering::Relaxed);
        }
        self.modal_download_cancel_token = None;
        self.modal_download_progress = 0.0;
        self.modal_downloaded_bytes = 0;
        self.modal_total_bytes = 0;
    }

    /// 取消所有缩略图加载任务
    pub fn cancel_thumb_loads(&mut self) {
        for cancel_token in &self.thumb_load_cancel_tokens {
            cancel_token.store(true, Ordering::Relaxed);
        }
        self.thumb_load_cancel_tokens.clear();
    }
}
