//! Wallhaven 数据模型
//!
//! 定义 Wallhaven API 的查询参数和枚举类型

use serde::{Deserialize, Serialize};

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
    Hot,
}

impl Sorting {
    pub fn all() -> [Sorting; 7] {
        [
            Sorting::DateAdded,
            Sorting::Relevance,
            Sorting::Random,
            Sorting::Views,
            Sorting::Favorites,
            Sorting::TopList,
            Sorting::Hot,
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
            Sorting::Hot => "hot",
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
            Sorting::Hot => "online-wallpapers.sorting-hot",
        }
    }
}

impl std::fmt::Display for Sorting {
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
        [Resolution::Any, Resolution::Standard, Resolution::Wide, Resolution::Ultrawide]
    }

    pub fn value(&self) -> &str {
        match self {
            Resolution::Any => "any",
            Resolution::Standard => "standard",
            Resolution::Wide => "wide",
            Resolution::Ultrawide => "ultrawide",
        }
    }

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

// 颜色选项（官方接口支持的29种颜色 + Any）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorOption {
    Any,
    Color660000, // 深红
    Color990000, // 正红
    ColorCC0000, // 亮红
    ColorCC3333, // 浅红
    ColorEA4C88, // 粉红
    Color993399, // 紫红
    Color663399, // 深紫
    Color333399, // 蓝紫
    Color0066CC, // 宝蓝
    Color0099CC, // 天蓝
    Color66CCCC, // 青绿
    Color77CC33, // 草绿
    Color669900, // 翠绿
    Color336600, // 深绿
    Color666600, // 橄榄绿
    Color999900, // 黄绿
    ColorCCCC33, // 柠檬黄
    ColorFFFF00, // 亮黄
    ColorFFCC33, // 金黄
    ColorFF9900, // 橙黄
    ColorFF6600, // 橘红
    ColorCC6633, // 砖红
    Color996633, // 棕褐
    Color663300, // 深棕
    Color000000, // 纯黑
    Color999999, // 深灰
    ColorCCCCCC, // 中灰
    ColorFFFFFF, // 纯白
    Color424153, // 深灰蓝
}

impl ColorOption {
    pub fn all() -> [ColorOption; 30] {
        [
            ColorOption::Any,
            ColorOption::Color660000,
            ColorOption::Color990000,
            ColorOption::ColorCC0000,
            ColorOption::ColorCC3333,
            ColorOption::ColorEA4C88,
            ColorOption::Color993399,
            ColorOption::Color663399,
            ColorOption::Color333399,
            ColorOption::Color0066CC,
            ColorOption::Color0099CC,
            ColorOption::Color66CCCC,
            ColorOption::Color77CC33,
            ColorOption::Color669900,
            ColorOption::Color336600,
            ColorOption::Color666600,
            ColorOption::Color999900,
            ColorOption::ColorCCCC33,
            ColorOption::ColorFFFF00,
            ColorOption::ColorFFCC33,
            ColorOption::ColorFF9900,
            ColorOption::ColorFF6600,
            ColorOption::ColorCC6633,
            ColorOption::Color996633,
            ColorOption::Color663300,
            ColorOption::Color000000,
            ColorOption::Color999999,
            ColorOption::ColorCCCCCC,
            ColorOption::ColorFFFFFF,
            ColorOption::Color424153,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            ColorOption::Any => "any",
            ColorOption::Color660000 => "660000",
            ColorOption::Color990000 => "990000",
            ColorOption::ColorCC0000 => "cc0000",
            ColorOption::ColorCC3333 => "cc3333",
            ColorOption::ColorEA4C88 => "ea4c88",
            ColorOption::Color993399 => "993399",
            ColorOption::Color663399 => "663399",
            ColorOption::Color333399 => "333399",
            ColorOption::Color0066CC => "0066cc",
            ColorOption::Color0099CC => "0099cc",
            ColorOption::Color66CCCC => "66cccc",
            ColorOption::Color77CC33 => "77cc33",
            ColorOption::Color669900 => "669900",
            ColorOption::Color336600 => "336600",
            ColorOption::Color666600 => "666600",
            ColorOption::Color999900 => "999900",
            ColorOption::ColorCCCC33 => "cccc33",
            ColorOption::ColorFFFF00 => "ffff00",
            ColorOption::ColorFFCC33 => "ffcc33",
            ColorOption::ColorFF9900 => "ff9900",
            ColorOption::ColorFF6600 => "ff6600",
            ColorOption::ColorCC6633 => "cc6633",
            ColorOption::Color996633 => "996633",
            ColorOption::Color663300 => "663300",
            ColorOption::Color000000 => "000000",
            ColorOption::Color999999 => "999999",
            ColorOption::ColorCCCCCC => "cccccc",
            ColorOption::ColorFFFFFF => "ffffff",
            ColorOption::Color424153 => "424153",
        }
    }
}

impl std::fmt::Display for ColorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

// 时间范围选项（仅用于 toplist 排序）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRange {
    Day,
    ThreeDays,
    Week,
    Month,
    ThreeMonths,
    SixMonths,
    Year,
}

impl TimeRange {
    pub fn all() -> [TimeRange; 7] {
        [
            TimeRange::Day,
            TimeRange::ThreeDays,
            TimeRange::Week,
            TimeRange::Month,
            TimeRange::ThreeMonths,
            TimeRange::SixMonths,
            TimeRange::Year,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            TimeRange::Day => "1d",
            TimeRange::ThreeDays => "3d",
            TimeRange::Week => "1w",
            TimeRange::Month => "1M",
            TimeRange::ThreeMonths => "3M",
            TimeRange::SixMonths => "6M",
            TimeRange::Year => "1y",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TimeRange::Day => "online-wallpapers.time-last-day",
            TimeRange::ThreeDays => "online-wallpapers.time-last-three-days",
            TimeRange::Week => "online-wallpapers.time-last-week",
            TimeRange::Month => "online-wallpapers.time-last-month",
            TimeRange::ThreeMonths => "online-wallpapers.time-last-three-months",
            TimeRange::SixMonths => "online-wallpapers.time-last-six-months",
            TimeRange::Year => "online-wallpapers.time-last-year",
        }
    }
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}