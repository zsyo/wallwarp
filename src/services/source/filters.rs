// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线搜索筛选模型
//!
//! 从 wallhaven::model 上移的图源无关筛选枚举（原样迁移，实现未改动）：
//! 当前 `.value()` 仍输出 Wallhaven API 参数字符串，接入第二个图源时再引入
//! 各图源自己的映射层

use serde::{Deserialize, Serialize};

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

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "date_added" => Some(Sorting::DateAdded),
            "relevance" => Some(Sorting::Relevance),
            "random" => Some(Sorting::Random),
            "views" => Some(Sorting::Views),
            "favorites" => Some(Sorting::Favorites),
            "toplist" => Some(Sorting::TopList),
            "hot" => Some(Sorting::Hot),
            _ => Some(Sorting::DateAdded),
        }
    }
}

impl std::fmt::Display for Sorting {
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

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "1d" => Some(TimeRange::Day),
            "3d" => Some(TimeRange::ThreeDays),
            "1w" => Some(TimeRange::Week),
            "1M" => Some(TimeRange::Month),
            "3M" => Some(TimeRange::ThreeMonths),
            "6M" => Some(TimeRange::SixMonths),
            "1y" => Some(TimeRange::Year),
            _ => Some(TimeRange::Month),
        }
    }
}

impl std::fmt::Display for TimeRange {
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
