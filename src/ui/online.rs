use super::AppMessage;
use crate::i18n::I18n;
use crate::utils::config::Config;
use iced::widget::{column, stack};
use iced::{Element, Length};

// 重新导出枚举类型，使其可以被其他模块使用
pub use crate::services::wallhaven::{AspectRatio, AspectRatioGroup, Category, ColorOption, Purity, Ratio, Resolution, Sorting, TimeRange};

// 子模块
use crate::ui::online_filter::create_filter_bar;
use crate::ui::online_list::create_wallpaper_list;
use crate::ui::online_modal::create_modal;

// 使用 services::wallhaven::OnlineWallpaper
pub use crate::services::wallhaven::OnlineWallpaper;

/// 包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableResolution {
    pub value: Resolution,
    pub display: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableRatio {
    pub value: Ratio,
    pub display: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableColorOption {
    pub value: ColorOption,
    pub display: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableTimeRange {
    pub value: TimeRange,
    pub display: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableSorting {
    pub value: Sorting,
    pub display: &'static str,
}

impl std::fmt::Display for DisplayableResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl std::fmt::Display for DisplayableRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl std::fmt::Display for DisplayableColorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl std::fmt::Display for DisplayableTimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl std::fmt::Display for DisplayableSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// 消息类型
#[derive(Debug, Clone)]
pub enum OnlineMessage {
    LoadWallpapers,
    LoadWallpapersSuccess(Vec<OnlineWallpaper>, bool, usize, usize), // wallpapers, last_page, total_pages, current_page
    LoadWallpapersFailed(String),
    LoadPage,
    LoadPageSuccess(Vec<OnlineWallpaper>, bool, usize, usize), // wallpapers, last_page, total_pages, current_page
    LoadPageFailed(String),
    WallpaperSelected(OnlineWallpaper),
    ScrollToBottom,
    CheckAndLoadNextPage, // 检查是否需要自动加载下一页
    ShowModal(usize),
    CloseModal,
    NextImage,
    PreviousImage,
    DownloadWallpaper(usize),
    DownloadFromCache(usize),
    SetAsWallpaperFromCache(usize),
    SetAsWallpaper(usize),
    ModalImageLoaded(iced::widget::image::Handle),
    ModalImageDownloaded(iced::widget::image::Handle),
    ModalImageDownloadFailed(String),
    ThumbLoaded(usize, iced::widget::image::Handle),
    // 筛选条件
    CategoryToggled(Category), // 切换分类选择状态
    SortingChanged(Sorting),
    PurityToggled(Purity), // 切换纯净度选择状态
    ResolutionChanged(Resolution),
    RatioChanged(Ratio),
    ColorChanged(ColorOption),
    ColorPickerExpanded, // 展开颜色选择器
    ColorPickerDismiss,  // 关闭颜色选择器
    TimeRangeChanged(TimeRange),
    SearchTextChanged(String),
    Search,
    Refresh, // 刷新按钮
    // 分辨率筛选器
    ResolutionPickerExpanded,              // 展开分辨率选择器
    ResolutionPickerDismiss,               // 关闭分辨率选择器
    ResolutionModeChanged(ResolutionMode), // 切换分辨率筛选模式
    ResolutionToggled(Resolution),         // 切换分辨率选择状态（Exactly模式）
    ResolutionAtLeastSelected(Resolution), // 选择分辨率（AtLeast模式）
    // 比例筛选器
    RatioPickerExpanded,       // 展开比例选择器
    RatioPickerDismiss,        // 关闭比例选择器
    RatioLandscapeToggled,     // 切换"全部横屏"选项
    RatioPortraitToggled,      // 切换"全部竖屏"选项
    RatioAllToggled,           // 切换"全部"选项
    RatioToggled(AspectRatio), // 切换比例选择状态
}

/// 壁纸加载状态
#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    Loading,
    ThumbLoaded(OnlineWallpaper, iced::widget::image::Handle),
    Loaded(OnlineWallpaper),
}

/// 分页信息，记录每页的结束索引和对应的页码
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub end_index: usize, // 该页最后一个壁纸的索引（不包含）
    pub page_num: usize,  // 该页的页码（从1开始）
}

/// 在线壁纸页面状态
#[derive(Debug)]
pub struct OnlineState {
    pub wallpapers: Vec<WallpaperLoadStatus>,
    pub wallpapers_data: Vec<OnlineWallpaper>, // 保存原始数据
    pub loading_page: bool,
    pub current_page: usize,
    pub page_size: usize,
    pub total_count: usize,
    pub total_pages: usize, // 总页数
    pub modal_visible: bool,
    pub current_image_index: usize,
    pub modal_image_handle: Option<iced::widget::image::Handle>,
    // 筛选条件
    pub categories: u32, // 位掩码：100(4)表示通用，010(2)表示动漫，001(1)表示人物
    pub sorting: Sorting,
    pub purities: u32, // 位掩码：100(4)表示安全，010(2)表示轻微，001(1)表示成人
    pub resolution: Resolution,
    pub ratio: Ratio,
    pub color: ColorOption,
    pub time_range: TimeRange,
    pub search_text: String,
    pub last_page: bool,
    pub has_loaded: bool,            // 标记是否已加载过数据
    pub page_info: Vec<PageInfo>,    // 记录每页的结束索引和页码，用于显示分页分隔线
    pub color_picker_expanded: bool, // 颜色选择器展开状态
    // 分辨率筛选器状态
    pub resolution_picker_expanded: bool,       // 分辨率选择器展开状态
    pub resolution_mode: ResolutionMode,        // 分辨率筛选模式：AtLeast 或 Exactly
    pub selected_resolutions: Vec<Resolution>,  // Exactly模式下选中的分辨率列表
    pub atleast_resolution: Option<Resolution>, // AtLeast模式下选中的分辨率
    // 比例筛选器状态
    pub ratio_picker_expanded: bool,       // 比例选择器展开状态
    pub selected_ratios: Vec<AspectRatio>, // 选中的比例列表
    pub ratio_landscape_selected: bool,    // 选中"全部横屏"
    pub ratio_portrait_selected: bool,     // 选中"全部竖屏"
    pub ratio_all_selected: bool,          // 选中"全部"
    // 请求上下文，用于取消正在进行的请求
    pub request_context: crate::services::request_context::RequestContext,
    // 待设置壁纸的文件名（用于在下载完成后自动设置壁纸）
    pub pending_set_wallpaper_filename: Option<String>,
    // 模态窗口图片下载状态
    pub modal_download_cancel_token: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    pub modal_download_progress: f32, // 0.0 到 1.0
    pub modal_downloaded_bytes: u64,
    pub modal_total_bytes: u64,
}

/// 分辨率筛选模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    All,     // 全部：不携带分辨率参数
    AtLeast, // 至少：atleast 参数
    Exactly, // 精确：resolutions 参数
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
            sorting: Sorting::DateAdded,
            purities: 0b100, // 默认只选择安全
            resolution: Resolution::Any,
            ratio: Ratio::Any,
            color: ColorOption::Any,
            time_range: TimeRange::Month,
            search_text: String::new(),
            last_page: false,
            has_loaded: false,     // 初始状态为未加载
            page_info: Vec::new(), // 初始化为空
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
            request_context: crate::services::request_context::RequestContext::new(),
            pending_set_wallpaper_filename: None,
            modal_download_cancel_token: None,
            modal_download_progress: 0.0,
            modal_downloaded_bytes: 0,
            modal_total_bytes: 0,
        }
    }
}

impl OnlineState {
    /// 从配置文件加载筛选条件
    pub fn load_from_config(config: &Config) -> Self {
        let mut state = Self::default();

        // 加载分类（从字符串解析位掩码）
        state.categories = match config.wallhaven.category.as_str() {
            "100" | "general" => 0b100,
            "010" | "anime" => 0b010,
            "001" | "people" => 0b001,
            "110" => 0b110, // general + anime
            "101" => 0b101, // general + people
            "011" => 0b011, // anime + people
            "111" => 0b111, // all
            _ => 0b100,
        };

        // 加载纯净度（从字符串解析位掩码）
        state.purities = match config.wallhaven.purity.as_str() {
            "100" | "sfw" => 0b100,
            "010" | "sketchy" => 0b010,
            "001" | "nsfw" => 0b001,
            "110" => 0b110, // sfw + sketchy
            "101" => 0b101, // sfw + nsfw
            "011" => 0b011, // sketchy + nsfw
            "111" => 0b111, // all
            _ => 0b100,
        };

        // 如果 API Key 为空，移除 NSFW 选项
        if config.wallhaven.api_key.is_empty() {
            state.purities &= !Purity::NSFW.bit_value();
        }

        // 加载排序
        state.sorting = match config.wallhaven.sorting.as_str() {
            "date_added" => Sorting::DateAdded,
            "relevance" => Sorting::Relevance,
            "random" => Sorting::Random,
            "views" => Sorting::Views,
            "favorites" => Sorting::Favorites,
            "toplist" => Sorting::TopList,
            "hot" => Sorting::Hot,
            _ => Sorting::DateAdded,
        };

        // 加载颜色
        state.color = match config.wallhaven.color.as_str() {
            "660000" => ColorOption::Color660000,
            "990000" => ColorOption::Color990000,
            "cc0000" => ColorOption::ColorCC0000,
            "cc3333" => ColorOption::ColorCC3333,
            "ea4c88" => ColorOption::ColorEA4C88,
            "993399" => ColorOption::Color993399,
            "663399" => ColorOption::Color663399,
            "333399" => ColorOption::Color333399,
            "0066cc" => ColorOption::Color0066CC,
            "0099cc" => ColorOption::Color0099CC,
            "66cccc" => ColorOption::Color66CCCC,
            "77cc33" => ColorOption::Color77CC33,
            "669900" => ColorOption::Color669900,
            "336600" => ColorOption::Color336600,
            "666600" => ColorOption::Color666600,
            "999900" => ColorOption::Color999900,
            "cccc33" => ColorOption::ColorCCCC33,
            "ffff00" => ColorOption::ColorFFFF00,
            "ffcc33" => ColorOption::ColorFFCC33,
            "ff9900" => ColorOption::ColorFF9900,
            "ff6600" => ColorOption::ColorFF6600,
            "cc6633" => ColorOption::ColorCC6633,
            "996633" => ColorOption::Color996633,
            "663300" => ColorOption::Color663300,
            "000000" => ColorOption::Color000000,
            "999999" => ColorOption::Color999999,
            "cccccc" => ColorOption::ColorCCCCCC,
            "ffffff" => ColorOption::ColorFFFFFF,
            "424153" => ColorOption::Color424153,
            _ => ColorOption::Any,
        };

        // 加载时间范围
        state.time_range = match config.wallhaven.top_range.as_str() {
            "1d" => TimeRange::Day,
            "3d" => TimeRange::ThreeDays,
            "1w" => TimeRange::Week,
            "1M" => TimeRange::Month,
            "3M" => TimeRange::ThreeMonths,
            "6M" => TimeRange::SixMonths,
            "1y" => TimeRange::Year,
            _ => TimeRange::Month,
        };

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
                "2560x1080" => Some(Resolution::R2560x1080),
                "2560x1440" => Some(Resolution::R2560x1440U),
                "3840x1600" => Some(Resolution::R3840x1600),
                "1280x720" => Some(Resolution::R1280x720),
                "1600x900" => Some(Resolution::R1600x900),
                "1920x1080" => Some(Resolution::R1920x1080),
                "3840x2160" => Some(Resolution::R3840x2160),
                "1280x800" => Some(Resolution::R1280x800),
                "1600x1000" => Some(Resolution::R1600x1000),
                "1920x1200" => Some(Resolution::R1920x1200),
                "2560x1600" => Some(Resolution::R2560x1600),
                "3840x2400" => Some(Resolution::R3840x2400),
                "1280x960" => Some(Resolution::R1280x960),
                "1600x1200" => Some(Resolution::R1600x1200_4_3),
                "1920x1440" => Some(Resolution::R1920x1440),
                "2560x1920" => Some(Resolution::R2560x1920),
                "3840x2880" => Some(Resolution::R3840x2880),
                "1280x1024" => Some(Resolution::R1280x1024),
                "1600x1280" => Some(Resolution::R1600x1280),
                "1920x1536" => Some(Resolution::R1920x1536),
                "2560x2048" => Some(Resolution::R2560x2048),
                "3840x3072" => Some(Resolution::R3840x3072),
                _ => None, // 无效值，返回 None
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

            let res_list: Vec<Resolution> = config
                .wallhaven
                .resolutions
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    // 只接受有效的分辨率值
                    if valid_resolutions.contains(&s) {
                        match s {
                            "2560x1080" => Some(Resolution::R2560x1080),
                            "2560x1440" => Some(Resolution::R2560x1440U),
                            "3840x1600" => Some(Resolution::R3840x1600),
                            "1280x720" => Some(Resolution::R1280x720),
                            "1600x900" => Some(Resolution::R1600x900),
                            "1920x1080" => Some(Resolution::R1920x1080),
                            "3840x2160" => Some(Resolution::R3840x2160),
                            "1280x800" => Some(Resolution::R1280x800),
                            "1600x1000" => Some(Resolution::R1600x1000),
                            "1920x1200" => Some(Resolution::R1920x1200),
                            "2560x1600" => Some(Resolution::R2560x1600),
                            "3840x2400" => Some(Resolution::R3840x2400),
                            "1280x960" => Some(Resolution::R1280x960),
                            "1600x1200" => Some(Resolution::R1600x1200_4_3),
                            "1920x1440" => Some(Resolution::R1920x1440),
                            "2560x1920" => Some(Resolution::R2560x1920),
                            "3840x2880" => Some(Resolution::R3840x2880),
                            "1280x1024" => Some(Resolution::R1280x1024),
                            "1600x1280" => Some(Resolution::R1600x1280),
                            "1920x1536" => Some(Resolution::R1920x1536),
                            "2560x2048" => Some(Resolution::R2560x2048),
                            "3840x3072" => Some(Resolution::R3840x3072),
                            _ => None,
                        }
                    } else {
                        None // 无效值，忽略
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
            // 定义被额外选项包含的详细比例
            let landscape_included = [
                AspectRatio::R16x9,
                AspectRatio::R16x10,
                AspectRatio::R21x9,
                AspectRatio::R32x9,
                AspectRatio::R48x9,
            ];
            let portrait_included = [AspectRatio::R9x16, AspectRatio::R10x16, AspectRatio::R9x18];

            // 解析 ratios 字符串
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
                    // 尝试解析详细比例
                    "16x9" => state.selected_ratios.push(AspectRatio::R16x9),
                    "16x10" => state.selected_ratios.push(AspectRatio::R16x10),
                    "21x9" => state.selected_ratios.push(AspectRatio::R21x9),
                    "32x9" => state.selected_ratios.push(AspectRatio::R32x9),
                    "48x9" => state.selected_ratios.push(AspectRatio::R48x9),
                    "9x16" => state.selected_ratios.push(AspectRatio::R9x16),
                    "10x16" => state.selected_ratios.push(AspectRatio::R10x16),
                    "9x18" => state.selected_ratios.push(AspectRatio::R9x18),
                    "1x1" => state.selected_ratios.push(AspectRatio::R1x1),
                    "3x2" => state.selected_ratios.push(AspectRatio::R3x2),
                    "4x3" => state.selected_ratios.push(AspectRatio::R4x3),
                    "5x4" => state.selected_ratios.push(AspectRatio::R5x4),
                    // 无效值，忽略
                    _ => {}
                }
            }

            // 移除被额外选项包含的详细比例（避免冗余）
            if state.ratio_landscape_selected {
                state.selected_ratios.retain(|r| !landscape_included.contains(r));
            }
            if state.ratio_portrait_selected {
                state.selected_ratios.retain(|r| !portrait_included.contains(r));
            }
        }

        state.has_loaded = false; // 从配置加载时重置为未加载状态

        state
    }

    /// 保存当前筛选条件到配置文件
    pub fn save_to_config(&self, config: &mut Config) {
        // 将位掩码转换为字符串
        config.wallhaven.category = format!("{:03b}", self.categories);
        config.wallhaven.purity = format!("{:03b}", self.purities);
        config.wallhaven.sorting = self.sorting.to_string();
        config.wallhaven.color = self.color.value().to_string();
        config.wallhaven.top_range = self.time_range.value().to_string();

        // 保存分辨率模式
        config.wallhaven.resolution_mode = match self.resolution_mode {
            ResolutionMode::All => "all".to_string(),
            ResolutionMode::AtLeast => "atleast".to_string(),
            ResolutionMode::Exactly => "exactly".to_string(),
        };

        // 保存AtLeast分辨率
        config.wallhaven.atleast_resolution = if let Some(res) = self.atleast_resolution {
            res.value().to_string()
        } else {
            String::new()
        };

        // 保存Exactly分辨率列表
        config.wallhaven.resolutions = if !self.selected_resolutions.is_empty() {
            let res_list: Vec<String> = self.selected_resolutions.iter().map(|r| r.value().to_string()).collect();
            res_list.join(",")
        } else {
            String::new()
        };

        // 保存比例列表和额外选项
        let mut ratios_vec = Vec::new();

        // 如果选中"全部横屏"，添加 landscape
        if self.ratio_landscape_selected {
            ratios_vec.push("landscape".to_string());
        }

        // 如果选中"全部竖屏"，添加 portrait
        if self.ratio_portrait_selected {
            ratios_vec.push("portrait".to_string());
        }

        // 添加详细模式的 ratios
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
    /// 根据当前页数和总页数判断
    pub fn should_load_next_page(&self) -> bool {
        !self.last_page && !self.loading_page && self.has_loaded
    }

    /// 取消当前正在进行的请求，并创建一个新的请求上下文
    /// 当切换页面时调用此方法可以取消正在进行的网络请求
    pub fn cancel_and_new_context(&mut self) {
        // 取消当前请求
        self.request_context.cancel();
        // 创建新的请求上下文
        self.request_context = crate::services::request_context::RequestContext::new();
    }

    /// 取消模态窗口图片下载
    pub fn cancel_modal_download(&mut self) {
        if let Some(cancel_token) = &self.modal_download_cancel_token {
            cancel_token.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        self.modal_download_cancel_token = None;
        self.modal_download_progress = 0.0;
        self.modal_downloaded_bytes = 0;
        self.modal_total_bytes = 0;
    }
}

/// 在线壁纸页面视图
pub fn online_view<'a>(i18n: &'a I18n, window_width: u32, online_state: &'a OnlineState, config: &'a Config) -> Element<'a, AppMessage> {
    // 创建筛选栏
    let filter_bar = create_filter_bar(i18n, online_state, config);

    // 创建壁纸列表
    let wallpaper_list = create_wallpaper_list(i18n, window_width, online_state);

    let main_content = column![filter_bar, wallpaper_list].width(Length::Fill).height(Length::Fill);

    let mut layers = vec![main_content.into()];

    // 图片预览模态窗口
    if online_state.modal_visible && !online_state.wallpapers.is_empty() {
        layers.push(create_modal(i18n, online_state));
    }

    stack(layers).width(Length::Fill).height(Length::Fill).into()
}

impl From<OnlineMessage> for AppMessage {
    fn from(online_message: OnlineMessage) -> AppMessage {
        AppMessage::Online(online_message)
    }
}
