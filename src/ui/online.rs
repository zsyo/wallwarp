use super::AppMessage;
use crate::i18n::I18n;
use crate::utils::config::Config;
use iced::widget::{column, stack};
use iced::{Element, Length};

// 重新导出枚举类型，使其可以被其他模块使用
pub use crate::services::wallhaven::{Category, ColorOption, Purity, Ratio, Resolution, Sorting, TimeRange};

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
    SetAsWallpaper(usize),
    ModalImageLoaded(iced::widget::image::Handle),
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
    // 请求上下文，用于取消正在进行的请求
    pub request_context: crate::services::request_context::RequestContext,
    // 待设置壁纸的文件名（用于在下载完成后自动设置壁纸）
    pub pending_set_wallpaper_filename: Option<String>,
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
            request_context: crate::services::request_context::RequestContext::new(),
            pending_set_wallpaper_filename: None,
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
}

/// 在线壁纸页面视图
pub fn online_view<'a>(
    i18n: &'a I18n,
    window_width: u32,
    online_state: &'a OnlineState,
    config: &'a Config,
) -> Element<'a, AppMessage> {
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