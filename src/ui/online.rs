use super::AppMessage;
use super::common;
use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};
use serde::{Deserialize, Serialize};

// 布局常量
const IMAGE_WIDTH: f32 = 300.0;
const IMAGE_HEIGHT: f32 = 200.0;
const IMAGE_SPACING: f32 = 20.0;
const EMPTY_STATE_PADDING: u16 = 360;
const EMPTY_STATE_TEXT_SIZE: f32 = 24.0;

// 加载文本常量
const LOADING_TEXT_SIZE: f32 = 24.0;

// 按钮常量
const ALL_LOADED_TEXT_SIZE: f32 = 14.0;

// 透明遮罩常量
const OVERLAY_HEIGHT: f32 = 22.0;
const OVERLAY_TEXT_SIZE: f32 = 12.0;

// 容器样式常量
const BORDER_WIDTH: f32 = 1.0;
const BORDER_RADIUS: f32 = 4.0;

// 颜色常量
const COLOR_BG_LIGHT: Color = Color::from_rgb(0.9, 0.9, 0.9);
const COLOR_TEXT_DARK: Color = Color::from_rgb(0.3, 0.3, 0.3);
const COLOR_MODAL_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.85);
const COLOR_OVERLAY_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);
const COLOR_OVERLAY_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);

// 浅色主题颜色
const COLOR_LIGHT_BG: Color = Color::from_rgb(0.969, 0.969, 0.969); // #F8F8F8
const COLOR_LIGHT_BUTTON: Color = Color::from_rgb(0.933, 0.933, 0.933); // #EEEEEE
#[allow(dead_code)]
const COLOR_LIGHT_BUTTON_HOVER: Color = Color::from_rgb(0.878, 0.878, 0.878); // #E0E0E0
const COLOR_LIGHT_TEXT: Color = Color::from_rgb(0.2, 0.2, 0.2); // #333333
const COLOR_LIGHT_TEXT_SUB: Color = Color::from_rgb(0.533, 0.533, 0.533); // #888888
const COLOR_SELECTED_BLUE: Color = Color::from_rgb(0.098, 0.463, 0.824); // #1976D2 (蓝色)

// 纯净度颜色
const COLOR_SFW: Color = Color::from_rgb(0.298, 0.686, 0.314); // #4CAF50
const COLOR_SKETCHY: Color = Color::from_rgb(1.0, 0.757, 0.027); // #FFC107
const COLOR_NSFW: Color = Color::from_rgb(0.965, 0.263, 0.212); // #F44336

// 筛选栏常量
#[allow(dead_code)]
const FILTER_BAR_HEIGHT: f32 = 60.0;
#[allow(dead_code)]
const FILTER_BAR_PADDING: f32 = 10.0;
#[allow(dead_code)]
const FILTER_SPACING: f32 = 10.0;
#[allow(dead_code)]
const FILTER_LABEL_SIZE: f32 = 14.0;

// 分页分隔线常量
const PAGE_SEPARATOR_HEIGHT: f32 = 40.0;
const PAGE_SEPARATOR_TEXT_SIZE: f32 = 18.0;
const PAGE_SEPARATOR_TEXT_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);

// 模态窗口加载占位符常量
const MODAL_LOADING_TEXT_SIZE: f32 = 20.0;

// 分类选项（位掩码表示）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    General, // 100 (第1位)
    Anime,   // 010 (第2位)
    People,  // 001 (第3位)
}

impl Category {
    pub fn all() -> [Category; 3] {
        [Category::General, Category::Anime, Category::People]
    }

    pub fn value(&self) -> &str {
        match self {
            Category::General => "general",
            Category::Anime => "anime",
            Category::People => "people",
        }
    }

    pub fn bit_position(&self) -> u8 {
        match self {
            Category::General => 2, // 第3位（从右到左）
            Category::Anime => 1,   // 第2位
            Category::People => 0,  // 第1位
        }
    }

    pub fn bit_value(&self) -> u32 {
        1 << self.bit_position()
    }

    pub fn id(&self) -> i32 {
        match self {
            Category::General => 1,
            Category::Anime => 2,
            Category::People => 3,
        }
    }
}

impl Category {
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::General => "online-wallpapers.category-general",
            Category::Anime => "online-wallpapers.category-anime",
            Category::People => "online-wallpapers.category-people",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 排序选项
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sorting {
    DateAdded,
    Relevance,
    Random,
    Views,
    Favorites,
    TopList,
}

impl Sorting {
    pub fn all() -> [Sorting; 6] {
        [
            Sorting::DateAdded,
            Sorting::Relevance,
            Sorting::Random,
            Sorting::Views,
            Sorting::Favorites,
            Sorting::TopList,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            Sorting::DateAdded => "date_added",
            Sorting::Relevance => "relevance",
            Sorting::Random => "random",
            Sorting::Views => "views",
            Sorting::Favorites => "favorites",
            Sorting::TopList => "toplist",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Sorting::DateAdded => "online-wallpapers.sorting-date-added",
            Sorting::Relevance => "online-wallpapers.sorting-relevance",
            Sorting::Random => "online-wallpapers.sorting-random",
            Sorting::Views => "online-wallpapers.sorting-views",
            Sorting::Favorites => "online-wallpapers.sorting-favorites",
            Sorting::TopList => "online-wallpapers.sorting-toplist",
        }
    }
}

impl std::fmt::Display for Sorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

// 排序方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,  // 正序
    Descending, // 倒序
}

impl SortDirection {
    pub fn all() -> [SortDirection; 2] {
        [SortDirection::Ascending, SortDirection::Descending]
    }

    pub fn value(&self) -> &str {
        match self {
            SortDirection::Ascending => "asc",
            SortDirection::Descending => "desc",
        }
    }

    pub fn toggle(&self) -> SortDirection {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

// 纯净度选项（位掩码表示）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purity {
    SFW,     // 100 (第1位)
    Sketchy, // 010 (第2位)
    NSFW,    // 001 (第3位)
}

impl Purity {
    pub fn all() -> [Purity; 3] {
        [Purity::SFW, Purity::Sketchy, Purity::NSFW]
    }

    pub fn value(&self) -> &str {
        match self {
            Purity::SFW => "sfw",
            Purity::Sketchy => "sketchy",
            Purity::NSFW => "nsfw",
        }
    }

    pub fn bit_position(&self) -> u8 {
        match self {
            Purity::SFW => 2,     // 第3位（从右到左）
            Purity::Sketchy => 1, // 第2位
            Purity::NSFW => 0,    // 第1位
        }
    }

    pub fn bit_value(&self) -> u32 {
        1 << self.bit_position()
    }

    pub fn id(&self) -> i32 {
        match self {
            Purity::SFW => 1,
            Purity::Sketchy => 2,
            Purity::NSFW => 3,
        }
    }
}

impl Purity {
    pub fn display_name(&self) -> &'static str {
        match self {
            Purity::SFW => "online-wallpapers.purity-sfw",
            Purity::Sketchy => "online-wallpapers.purity-sketchy",
            Purity::NSFW => "online-wallpapers.purity-nsfw",
        }
    }
}

impl std::fmt::Display for Purity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 分辨率选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Any,
    Standard,
    Wide,
    Ultrawide,
}

impl Resolution {
    pub fn all() -> [Resolution; 4] {
        [
            Resolution::Any,
            Resolution::Standard,
            Resolution::Wide,
            Resolution::Ultrawide,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            Resolution::Any => "any",
            Resolution::Standard => "standard",
            Resolution::Wide => "wide",
            Resolution::Ultrawide => "ultrawide",
        }
    }
}

impl Resolution {
    pub fn display_name(&self) -> &'static str {
        match self {
            Resolution::Any => "online-wallpapers.resolution-any",
            Resolution::Standard => "online-wallpapers.resolution-standard",
            Resolution::Wide => "online-wallpapers.resolution-wide",
            Resolution::Ultrawide => "online-wallpapers.resolution-ultrawide",
        }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 比例选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio {
    Any,
    Portrait,
    Landscape,
    Square,
}

impl Ratio {
    pub fn all() -> [Ratio; 4] {
        [Ratio::Any, Ratio::Portrait, Ratio::Landscape, Ratio::Square]
    }

    pub fn value(&self) -> &str {
        match self {
            Ratio::Any => "any",
            Ratio::Portrait => "portrait",
            Ratio::Landscape => "landscape",
            Ratio::Square => "square",
        }
    }
}

impl Ratio {
    pub fn display_name(&self) -> &'static str {
        match self {
            Ratio::Any => "online-wallpapers.ratio-any",
            Ratio::Portrait => "online-wallpapers.ratio-portrait",
            Ratio::Landscape => "online-wallpapers.ratio-landscape",
            Ratio::Square => "online-wallpapers.ratio-square",
        }
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 颜色选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorOption {
    Any,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Gray,
    Black,
    White,
}

impl ColorOption {
    pub fn all() -> [ColorOption; 11] {
        [
            ColorOption::Any,
            ColorOption::Red,
            ColorOption::Orange,
            ColorOption::Yellow,
            ColorOption::Green,
            ColorOption::Blue,
            ColorOption::Purple,
            ColorOption::Pink,
            ColorOption::Gray,
            ColorOption::Black,
            ColorOption::White,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            ColorOption::Any => "any",
            ColorOption::Red => "red",
            ColorOption::Orange => "orange",
            ColorOption::Yellow => "yellow",
            ColorOption::Green => "green",
            ColorOption::Blue => "blue",
            ColorOption::Purple => "purple",
            ColorOption::Pink => "pink",
            ColorOption::Gray => "gray",
            ColorOption::Black => "black",
            ColorOption::White => "white",
        }
    }
}

impl ColorOption {
    pub fn display_name(&self) -> &'static str {
        match self {
            ColorOption::Any => "online-wallpapers.color-any",
            ColorOption::Red => "online-wallpapers.color-red",
            ColorOption::Orange => "online-wallpapers.color-orange",
            ColorOption::Yellow => "online-wallpapers.color-yellow",
            ColorOption::Green => "online-wallpapers.color-green",
            ColorOption::Blue => "online-wallpapers.color-blue",
            ColorOption::Purple => "online-wallpapers.color-purple",
            ColorOption::Pink => "online-wallpapers.color-pink",
            ColorOption::Gray => "online-wallpapers.color-gray",
            ColorOption::Black => "online-wallpapers.color-black",
            ColorOption::White => "online-wallpapers.color-white",
        }
    }
}

impl std::fmt::Display for ColorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 时间范围选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRange {
    Any,
    Day,
    Week,
    Month,
    Year,
}

impl TimeRange {
    pub fn all() -> [TimeRange; 5] {
        [
            TimeRange::Any,
            TimeRange::Day,
            TimeRange::Week,
            TimeRange::Month,
            TimeRange::Year,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            TimeRange::Any => "any",
            TimeRange::Day => "1d",
            TimeRange::Week => "1w",
            TimeRange::Month => "1M",
            TimeRange::Year => "1y",
        }
    }
}

impl TimeRange {
    pub fn display_name(&self) -> &'static str {
        match self {
            TimeRange::Any => "online-wallpapers.time-any",
            TimeRange::Day => "online-wallpapers.time-day",
            TimeRange::Week => "online-wallpapers.time-week",
            TimeRange::Month => "online-wallpapers.time-month",
            TimeRange::Year => "online-wallpapers.time-year",
        }
    }
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// 包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableCategory {
    pub value: Category,
    pub display: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayablePurity {
    pub value: Purity,
    pub display: &'static str,
}

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

#[derive(Debug, Clone)]
pub enum OnlineMessage {
    LoadWallpapers,
    LoadWallpapersSuccess(Vec<OnlineWallpaper>, bool, usize),
    LoadWallpapersFailed(String),
    LoadPage,
    LoadPageSuccess(Vec<OnlineWallpaper>, bool, usize),
    LoadPageFailed(String),
    WallpaperSelected(OnlineWallpaper),
    ScrollToBottom,
    ShowModal(usize),
    CloseModal,
    NextImage,
    PreviousImage,
    DownloadWallpaper(usize),
    ModalImageLoaded(iced::widget::image::Handle),
    ThumbLoaded(usize, iced::widget::image::Handle),
    // 筛选条件
    CategoryToggled(Category), // 切换分类选择状态
    SortingChanged(Sorting),
    ToggleSortDirection,   // 切换排序方向
    PurityToggled(Purity), // 切换纯净度选择状态
    ResolutionChanged(Resolution),
    RatioChanged(Ratio),
    ColorChanged(ColorOption),
    TimeRangeChanged(TimeRange),
    SearchTextChanged(String),
    Search,
    Refresh, // 刷新按钮
}

#[derive(Debug, Clone)]
pub struct OnlineWallpaper {
    pub id: String,
    pub url: String,
    pub path: String,
    pub thumb_large: String,
    pub thumb_original: String,
    pub thumb_small: String,
    pub width: u32,
    pub height: u32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
    pub category: String,
    pub purity: String,
    pub views: u32,
    pub favorites: u32,
    pub colors: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    Loading,
    ThumbLoaded(OnlineWallpaper, iced::widget::image::Handle),
    Loaded(OnlineWallpaper),
}

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
    pub sort_direction: SortDirection, // 排序方向
    pub purities: u32,                 // 位掩码：100(4)表示安全，010(2)表示轻微，001(1)表示成人
    pub resolution: Resolution,
    pub ratio: Ratio,
    pub color: ColorOption,
    pub time_range: TimeRange,
    pub search_text: String,
    pub last_page: bool,
    pub has_loaded: bool,            // 标记是否已加载过数据
    pub page_boundaries: Vec<usize>, // 记录每页的起始索引，用于显示分页分隔线
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
            sort_direction: SortDirection::Descending, // 默认倒序
            purities: 0b100,                           // 默认只选择安全
            resolution: Resolution::Any,
            ratio: Ratio::Any,
            color: ColorOption::Any,
            time_range: TimeRange::Any,
            search_text: String::new(),
            last_page: false,
            has_loaded: false,           // 初始状态为未加载
            page_boundaries: Vec::new(), // 初始化为空
        }
    }
}

impl OnlineState {
    /// 从配置文件加载筛选条件
    pub fn load_from_config(config: &crate::utils::config::Config) -> Self {
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

        state.has_loaded = false; // 从配置加载时重置为未加载状态

        state
    }

    /// 保存当前筛选条件到配置文件
    pub fn save_to_config(&self, config: &mut crate::utils::config::Config) {
        // 将位掩码转换为字符串
        config.wallhaven.category = format!("{:03b}", self.categories);
        config.wallhaven.sorting = self.sorting.to_string();
        config.wallhaven.purity = format!("{:03b}", self.purities);
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
}

pub fn online_view<'a>(
    i18n: &'a crate::i18n::I18n,
    window_width: u32,
    online_state: &'a OnlineState,
) -> Element<'a, AppMessage> {
    // 创建筛选栏
    let filter_bar = create_filter_bar(i18n, online_state);

    let content = if !online_state.has_loaded && !online_state.loading_page {
        // 初始状态，还未开始加载
        column![text(i18n.t("online-wallpapers.loading")).size(LOADING_TEXT_SIZE)]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(EMPTY_STATE_PADDING)
    } else if online_state.wallpapers.is_empty() && online_state.loading_page {
        // 正在加载中
        column![text(i18n.t("online-wallpapers.loading")).size(LOADING_TEXT_SIZE)]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .padding(EMPTY_STATE_PADDING)
    } else if online_state.wallpapers.is_empty() && online_state.has_loaded {
        // 已加载但无数据
        column![
            text(i18n.t("online-wallpapers.no-data")).size(EMPTY_STATE_TEXT_SIZE),
            text(i18n.t("online-wallpapers.no-data-hint"))
                .size(14)
                .style(|_theme: &iced::Theme| text::Style {
                    color: Some(COLOR_LIGHT_TEXT_SUB),
                }),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(EMPTY_STATE_PADDING)
        .spacing(10)
    } else {
        let available_width = (window_width as f32 - IMAGE_SPACING).max(IMAGE_WIDTH);
        let unit_width = IMAGE_WIDTH + IMAGE_SPACING;
        let items_per_row = (available_width / unit_width).floor() as usize;
        let items_per_row = items_per_row.max(1);

        let mut content = column![]
            .spacing(IMAGE_SPACING)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        // 遍历所有壁纸，按行渲染，在每页数据的下面添加分页分隔线
        let mut current_page_num = 1;
        let mut boundary_iter = online_state.page_boundaries.iter().peekable();

        for (row_index, chunk) in online_state.wallpapers.chunks(items_per_row).enumerate() {
            // 创建当前行的壁纸
            let mut row_container = row![].spacing(IMAGE_SPACING).align_y(Alignment::Center);

            for wallpaper_status in chunk {
                let image_element = match wallpaper_status {
                    WallpaperLoadStatus::Loading => create_loading_placeholder(i18n),
                    WallpaperLoadStatus::ThumbLoaded(wallpaper, handle) => {
                        let wallpaper_index = online_state
                            .wallpapers
                            .iter()
                            .position(|w| matches!(w, WallpaperLoadStatus::ThumbLoaded(wp, _) if wp.id == wallpaper.id))
                            .unwrap_or(0);
                        create_loaded_wallpaper_with_thumb(i18n, wallpaper, Some(handle.clone()), wallpaper_index)
                    }
                    WallpaperLoadStatus::Loaded(wallpaper) => {
                        let wallpaper_index = online_state
                            .wallpapers
                            .iter()
                            .position(|w| matches!(w, WallpaperLoadStatus::Loaded(wp) if wp.id == wallpaper.id))
                            .unwrap_or(0);
                        create_loaded_wallpaper_with_thumb(i18n, wallpaper, None, wallpaper_index)
                    }
                };

                row_container = row_container.push(image_element);
            }

            let centered_row = container(row_container).width(Length::Fill).center_x(Length::Fill);
            content = content.push(centered_row);

            // 检查是否需要添加分页分隔线（在当前行之后）
            // 计算当前行最后一个壁纸的索引
            let current_end_index = (row_index + 1) * items_per_row.min(chunk.len());

            // 如果当前行的结束索引等于某个分页边界，则添加分页标识
            if boundary_iter
                .peek()
                .map_or(false, |&boundary| current_end_index == *boundary)
            {
                // 添加分页分隔线
                content = content.push(create_page_separator(i18n, current_page_num, online_state.total_pages));
                boundary_iter.next();
                current_page_num += 1;
            }
        }

        // 如果是最后一页，显示"已加载全部"
        if online_state.last_page {
            let all_loaded_text = text(i18n.t("online-wallpapers.all-loaded")).size(ALL_LOADED_TEXT_SIZE);
            content = content.push(all_loaded_text)
        }

        column![
            iced::widget::Space::new().height(IMAGE_SPACING),
            content,
            iced::widget::Space::new().height(IMAGE_SPACING)
        ]
    };

    let scrollable_content = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(iced::widget::Id::new("online_wallpapers"))
        .on_scroll(|viewport| {
            // 检查是否滚动到底部
            // 使用 offset 和 content_size 来判断滚动位置
            let content_height = viewport.content_bounds().height;
            let view_height = viewport.bounds().height;
            let scroll_position = viewport.absolute_offset().y;

            // 计算可滚动的总距离
            let scrollable_height = content_height - view_height;

            // 只有当有足够的可滚动内容时才检测（避免内容不足时误触发）
            if scrollable_height > 0.0 {
                // 计算当前滚动百分比（0.0 到 1.0）
                let scroll_percentage = scroll_position / scrollable_height;

                // 当滚动到 95% 以上时触发加载
                let is_near_bottom = scroll_percentage >= 0.95;

                if is_near_bottom {
                    super::AppMessage::Online(OnlineMessage::ScrollToBottom)
                } else {
                    super::AppMessage::None
                }
            } else {
                super::AppMessage::None
            }
        });

    let main_content = column![filter_bar, scrollable_content]
        .width(Length::Fill)
        .height(Length::Fill);

    let mut layers = vec![main_content.into()];

    // 图片预览模态窗口
    if online_state.modal_visible && !online_state.wallpapers.is_empty() {
        let wallpaper_index = online_state.current_image_index;

        // 创建背景加载文字
        let loading_text = create_modal_loading_placeholder(i18n);

        // 创建图片层（加载完成后显示）
        let image_layer: Element<_> = if let Some(ref handle) = online_state.modal_image_handle {
            let modal_image = iced::widget::image(handle.clone())
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill);
            modal_image.into()
        } else {
            container(iced::widget::Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

        // 创建底部工具栏按钮
        let prev_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F12E}",
                common::BUTTON_COLOR_BLUE,
                AppMessage::Online(OnlineMessage::PreviousImage),
            ),
            i18n.t("online-wallpapers.tooltip-prev"),
        );

        let next_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F137}",
                common::BUTTON_COLOR_BLUE,
                AppMessage::Online(OnlineMessage::NextImage),
            ),
            i18n.t("online-wallpapers.tooltip-next"),
        );

        let download_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F1E5}",
                common::BUTTON_COLOR_GREEN,
                AppMessage::Online(OnlineMessage::DownloadWallpaper(wallpaper_index)),
            ),
            i18n.t("online-wallpapers.tooltip-download"),
        );

        let close_button = common::create_button_with_tooltip(
            common::create_icon_button(
                "\u{F659}",
                common::BUTTON_COLOR_RED,
                AppMessage::Online(OnlineMessage::CloseModal),
            ),
            i18n.t("online-wallpapers.tooltip-close"),
        );

        // 底部工具栏
        let toolbar = container(
            row![
                container(iced::widget::Space::new()).width(Length::Fill),
                prev_button,
                next_button,
                download_button,
                close_button,
                container(iced::widget::Space::new()).width(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .spacing(50.0),
        )
        .height(Length::Fixed(30.0))
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.7,
            })),
            ..Default::default()
        });

        let modal_content = container(
            column![
                container(modal_image_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(20),
                toolbar,
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_MODAL_BG)),
            ..Default::default()
        });

        layers.push(container(iced::widget::opaque(modal_content)).into());
    }

    iced::widget::stack(layers)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn create_filter_bar<'a>(i18n: &'a crate::i18n::I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
    // 搜索框（放在最前面）
    let search_input = iced::widget::text_input(&i18n.t("online-wallpapers.search-placeholder"), &state.search_text)
        .on_input(|text| AppMessage::Online(OnlineMessage::SearchTextChanged(text)))
        .on_submit(AppMessage::Online(OnlineMessage::Search))
        .padding(6)
        .size(14)
        .width(Length::Fixed(200.0))
        .style(|_theme: &iced::Theme, _status| iced::widget::text_input::Style {
            background: iced::Background::Color(COLOR_LIGHT_BUTTON),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            icon: COLOR_LIGHT_TEXT_SUB,
            placeholder: COLOR_LIGHT_TEXT_SUB,
            value: COLOR_LIGHT_TEXT,
            selection: Color::from_rgba(0.098, 0.463, 0.824, 0.3),
        });

    let search_button = button(text("🔍").size(16))
        .on_press(AppMessage::Online(OnlineMessage::Search))
        .padding(6)
        .style(|_theme: &iced::Theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            text_color: COLOR_LIGHT_TEXT,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    let search_container = row![search_input, search_button].spacing(4).align_y(Alignment::Center);

    // 下拉筛选器 - 使用包装类型以支持 i18n
    let resolution_options: Vec<DisplayableResolution> = Resolution::all()
        .iter()
        .map(|r| DisplayableResolution {
            value: *r,
            display: i18n.t(r.display_name()).leak(),
        })
        .collect();
    let current_resolution = DisplayableResolution {
        value: state.resolution,
        display: i18n.t(state.resolution.display_name()).leak(),
    };

    let resolution_picker = pick_list(resolution_options.clone(), Some(current_resolution), |res| {
        AppMessage::Online(OnlineMessage::ResolutionChanged(res.value))
    })
    .padding(6)
    .width(Length::Fixed(90.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    let ratio_options: Vec<DisplayableRatio> = Ratio::all()
        .iter()
        .map(|r| DisplayableRatio {
            value: *r,
            display: i18n.t(r.display_name()).leak(),
        })
        .collect();
    let current_ratio = DisplayableRatio {
        value: state.ratio,
        display: i18n.t(state.ratio.display_name()).leak(),
    };

    let ratio_picker = pick_list(ratio_options.clone(), Some(current_ratio), |rat| {
        AppMessage::Online(OnlineMessage::RatioChanged(rat.value))
    })
    .padding(6)
    .width(Length::Fixed(90.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    let color_options: Vec<DisplayableColorOption> = ColorOption::all()
        .iter()
        .map(|c| DisplayableColorOption {
            value: *c,
            display: i18n.t(c.display_name()).leak(),
        })
        .collect();
    let current_color = DisplayableColorOption {
        value: state.color,
        display: i18n.t(state.color.display_name()).leak(),
    };

    let color_picker = pick_list(color_options.clone(), Some(current_color), |col| {
        AppMessage::Online(OnlineMessage::ColorChanged(col.value))
    })
    .padding(6)
    .width(Length::Fixed(90.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    let sorting_options: Vec<DisplayableSorting> = Sorting::all()
        .iter()
        .map(|s| DisplayableSorting {
            value: *s,
            display: i18n.t(s.display_name()).leak(),
        })
        .collect();
    let current_sorting = DisplayableSorting {
        value: state.sorting,
        display: i18n.t(state.sorting.display_name()).leak(),
    };

    let sorting_picker = pick_list(sorting_options.clone(), Some(current_sorting), |sort| {
        AppMessage::Online(OnlineMessage::SortingChanged(sort.value))
    })
    .padding(6)
    .width(Length::Fixed(90.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    let time_range_options: Vec<DisplayableTimeRange> = TimeRange::all()
        .iter()
        .map(|t| DisplayableTimeRange {
            value: *t,
            display: i18n.t(t.display_name()).leak(),
        })
        .collect();
    let current_time_range = DisplayableTimeRange {
        value: state.time_range,
        display: i18n.t(state.time_range.display_name()).leak(),
    };

    let time_range_picker = pick_list(time_range_options.clone(), Some(current_time_range), |time| {
        AppMessage::Online(OnlineMessage::TimeRangeChanged(time.value))
    })
    .padding(6)
    .width(Length::Fixed(90.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    // 功能按钮
    let sort_direction_icon = match state.sort_direction {
        SortDirection::Ascending => "▲",
        SortDirection::Descending => "▼",
    };
    let sort_direction_button = button(text(sort_direction_icon).size(12))
        .on_press(AppMessage::Online(OnlineMessage::ToggleSortDirection))
        .padding(6)
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            text_color: COLOR_LIGHT_TEXT,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    let refresh_button = button(text("🔄").size(14))
        .on_press(AppMessage::Online(OnlineMessage::Refresh))
        .padding(6)
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            text_color: COLOR_LIGHT_TEXT,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 组合所有元素
    let filter_row = row![
        search_container,
        iced::widget::Space::new().width(8),
        // 分类按钮（选中状态为蓝色）
        button(text(i18n.t("online-wallpapers.category-general")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::General)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::General.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.category-anime")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::Anime)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::Anime.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.category-people")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::People)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::People.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        iced::widget::Space::new().width(8),
        // 纯净度按钮（带颜色）
        button(text(i18n.t("online-wallpapers.purity-sfw")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::SFW)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::SFW.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SFW, Color::WHITE)
                } else {
                    (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.purity-sketchy")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::Sketchy)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::Sketchy.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SKETCHY, Color::BLACK)
                } else {
                    (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.purity-nsfw")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::NSFW)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::NSFW.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_NSFW, Color::WHITE)
                } else {
                    (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        iced::widget::Space::new().width(8),
        resolution_picker,
        iced::widget::Space::new().width(4),
        ratio_picker,
        iced::widget::Space::new().width(4),
        color_picker,
        iced::widget::Space::new().width(4),
        time_range_picker,
        iced::widget::Space::new().width(8),
        sorting_picker,
        iced::widget::Space::new().width(4),
        sort_direction_button,
        iced::widget::Space::new().width(4),
        refresh_button,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    container(filter_row)
        .width(Length::Fill)
        .height(Length::Fixed(50.0))
        .padding(8)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BG)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..Default::default()
        })
        .into()
}

fn create_loading_placeholder<'a>(i18n: &'a crate::i18n::I18n) -> Element<'a, AppMessage> {
    let loading_text = text(i18n.t("online-wallpapers.image-loading"))
        .size(LOADING_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_TEXT_DARK),
        });

    let placeholder_content = container(loading_text)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(create_bordered_container_style);

    button(placeholder_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .into()
}

fn create_loaded_wallpaper_with_thumb<'a>(
    i18n: &'a crate::i18n::I18n,
    wallpaper: &'a OnlineWallpaper,
    thumb_handle: Option<iced::widget::image::Handle>,
    index: usize,
) -> Element<'a, AppMessage> {
    // 使用缩略图创建图片
    let image = if let Some(handle) = thumb_handle {
        iced::widget::image(handle)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .content_fit(iced::ContentFit::Fill)
    } else {
        // 如果没有缩略图，使用占位符
        let placeholder = text(i18n.t("online-wallpapers.loading-placeholder"))
            .size(LOADING_TEXT_SIZE)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(COLOR_TEXT_DARK),
            });

        return container(placeholder)
            .width(Length::Fixed(IMAGE_WIDTH))
            .height(Length::Fixed(IMAGE_HEIGHT))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(create_bordered_container_style)
            .into();
    };

    let styled_image = container(image)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .style(create_bordered_container_style);

    // 创建透明遮罩内容
    let file_size_text = text(crate::utils::helpers::format_file_size(wallpaper.file_size))
        .size(OVERLAY_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let resolution_text = text(&wallpaper.resolution)
        .size(OVERLAY_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    let download_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F1E5}",
            common::BUTTON_COLOR_GREEN,
            super::AppMessage::Online(OnlineMessage::DownloadWallpaper(index)),
        ),
        i18n.t("online-wallpapers.tooltip-download"),
    );

    // 左侧区域：文件大小
    let left_area = container(file_size_text).align_y(Alignment::Center);

    // 右侧区域：下载按钮
    let right_area = download_button;

    // 使用 stack 确保分辨率永远居中，不受两侧内容影响
    let overlay_content = iced::widget::stack(vec![
        // 底层：左中右三部分布局
        container(
            row![
                left_area,
                // 中间占位，让分辨率在顶层居中
                container(iced::widget::Space::new()).width(Length::Fill),
                right_area,
            ]
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .padding([0, 8])
        .into(),
        // 顶层：分辨率居中显示
        container(resolution_text)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]);

    // 创建遮罩层
    let overlay = container(overlay_content)
        .width(Length::Fill)
        .height(Length::Fixed(OVERLAY_HEIGHT))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_OVERLAY_BG)),
            ..Default::default()
        });

    // 使用 stack 将遮罩覆盖在图片内部下方
    let card_content = iced::widget::stack(vec![
        styled_image.into(),
        container(overlay)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::End)
            .into(),
    ]);

    button(card_content)
        .padding(0)
        .width(Length::Fixed(IMAGE_WIDTH))
        .height(Length::Fixed(IMAGE_HEIGHT))
        .on_press(super::AppMessage::Online(OnlineMessage::ShowModal(index)))
        .into()
}

fn create_bordered_container_style(theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(COLOR_BG_LIGHT)),
        border: iced::border::Border {
            color: theme.extended_palette().primary.weak.color,
            width: BORDER_WIDTH,
            radius: iced::border::Radius::from(BORDER_RADIUS),
        },
        ..Default::default()
    }
}

fn create_page_separator<'a>(
    i18n: &'a crate::i18n::I18n,
    current_page: usize,
    total_pages: usize,
) -> Element<'a, AppMessage> {
    let page_text = i18n
        .t("online-wallpapers.page-separator")
        .replace("{current}", &current_page.to_string())
        .replace("{total}", &total_pages.to_string());

    let separator = container(
        text(page_text)
            .size(PAGE_SEPARATOR_TEXT_SIZE)
            .style(|_theme: &iced::Theme| text::Style {
                color: Some(PAGE_SEPARATOR_TEXT_COLOR),
            }),
    )
    .width(Length::Fill)
    .height(Length::Fixed(PAGE_SEPARATOR_HEIGHT))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    container(separator).width(Length::Fill).padding([10, 20]).into()
}

fn create_modal_loading_placeholder<'a>(i18n: &'a crate::i18n::I18n) -> Element<'a, AppMessage> {
    let loading_text = text(i18n.t("online-wallpapers.image-loading"))
        .size(MODAL_LOADING_TEXT_SIZE)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    container(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

impl From<OnlineMessage> for AppMessage {
    fn from(online_message: OnlineMessage) -> AppMessage {
        AppMessage::Online(online_message)
    }
}
