// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 站点内容分类筛选枚举
//!
//! wallpaperscraft 与 wp.upx8 各自提供一套分类 key，两者有重叠（nature/anime/
//! city 等）也有差异（animals/animal、cars/car），故本枚举为并集，
//! 由 [`SiteCategory::api_value`] 按图源输出各自的取值；无该维度的图源
//! （Wallhaven/Bing）忽略本字段

use crate::services::source::SourceKind;
use serde::{Deserialize, Serialize};

/// 站点内容分类（Any 为不限）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SiteCategory {
    /// 不限分类
    Any,
    /// 3D 渲染（仅 wallpaperscraft）
    C3d,
    Abstract,
    /// wallpaperscraft 为 animals，wp.upx8 为 animal
    Animals,
    Anime,
    Art,
    Black,
    BlackAndWhite,
    /// wallpaperscraft 为 cars，wp.upx8 为 car
    Cars,
    City,
    Dark,
    Fantasy,
    Flowers,
    Food,
    /// 游戏（仅 wp.upx8）
    Game,
    /// 人物写真（仅 wp.upx8）
    Girl,
    HiTech,
    Holidays,
    Love,
    Macro,
    Minimalism,
    Motorcycles,
    Music,
    Nature,
    Other,
    Space,
    Sport,
    Textures,
    Vector,
    Words,
}

/// wallpaperscraft 支持的分类（顺序即下拉面板顺序）
const WALLPAPERSCRAFT_CATEGORIES: [SiteCategory; 28] = [
    SiteCategory::Any,
    SiteCategory::C3d,
    SiteCategory::Abstract,
    SiteCategory::Animals,
    SiteCategory::Anime,
    SiteCategory::Art,
    SiteCategory::Black,
    SiteCategory::BlackAndWhite,
    SiteCategory::Cars,
    SiteCategory::City,
    SiteCategory::Dark,
    SiteCategory::Fantasy,
    SiteCategory::Flowers,
    SiteCategory::Food,
    SiteCategory::HiTech,
    SiteCategory::Holidays,
    SiteCategory::Love,
    SiteCategory::Macro,
    SiteCategory::Minimalism,
    SiteCategory::Motorcycles,
    SiteCategory::Music,
    SiteCategory::Nature,
    SiteCategory::Other,
    SiteCategory::Space,
    SiteCategory::Sport,
    SiteCategory::Textures,
    SiteCategory::Vector,
    SiteCategory::Words,
];

/// wp.upx8 支持的分类（顺序即下拉面板顺序）
const UPX8_CATEGORIES: [SiteCategory; 11] = [
    SiteCategory::Any,
    SiteCategory::Nature,
    SiteCategory::Anime,
    SiteCategory::Game,
    SiteCategory::Animals,
    SiteCategory::City,
    SiteCategory::Abstract,
    SiteCategory::Space,
    SiteCategory::Cars,
    SiteCategory::Girl,
    SiteCategory::Sport,
];

impl SiteCategory {
    /// 指定图源可选的分类列表（不支持该维度的图源返回仅 Any）
    pub fn all_for(source: SourceKind) -> &'static [SiteCategory] {
        match source {
            SourceKind::Wallpaperscraft => &WALLPAPERSCRAFT_CATEGORIES,
            SourceKind::Upx8 => &UPX8_CATEGORIES,
            SourceKind::Wallhaven | SourceKind::Bing => &WALLPAPERSCRAFT_CATEGORIES[..1],
        }
    }

    /// 指定图源的分类参数值（None 表示不限，不携带参数）
    ///
    /// 同一枚举在两个站点上的 key 不完全一致，故需传入图源
    pub fn api_value(&self, source: SourceKind) -> Option<&'static str> {
        if *self == SiteCategory::Any {
            return None;
        }
        match (self, source) {
            (SiteCategory::Animals, SourceKind::Upx8) => Some("animal"),
            (SiteCategory::Cars, SourceKind::Upx8) => Some("car"),
            (SiteCategory::C3d, _) => Some("3d"),
            (SiteCategory::Abstract, _) => Some("abstract"),
            (SiteCategory::Animals, _) => Some("animals"),
            (SiteCategory::Anime, _) => Some("anime"),
            (SiteCategory::Art, _) => Some("art"),
            (SiteCategory::Black, _) => Some("black"),
            (SiteCategory::BlackAndWhite, _) => Some("black_and_white"),
            (SiteCategory::Cars, _) => Some("cars"),
            (SiteCategory::City, _) => Some("city"),
            (SiteCategory::Dark, _) => Some("dark"),
            (SiteCategory::Fantasy, _) => Some("fantasy"),
            (SiteCategory::Flowers, _) => Some("flowers"),
            (SiteCategory::Food, _) => Some("food"),
            (SiteCategory::Game, _) => Some("game"),
            (SiteCategory::Girl, _) => Some("girl"),
            (SiteCategory::HiTech, _) => Some("hi-tech"),
            (SiteCategory::Holidays, _) => Some("holidays"),
            (SiteCategory::Love, _) => Some("love"),
            (SiteCategory::Macro, _) => Some("macro"),
            (SiteCategory::Minimalism, _) => Some("minimalism"),
            (SiteCategory::Motorcycles, _) => Some("motorcycles"),
            (SiteCategory::Music, _) => Some("music"),
            (SiteCategory::Nature, _) => Some("nature"),
            (SiteCategory::Other, _) => Some("other"),
            (SiteCategory::Space, _) => Some("space"),
            (SiteCategory::Sport, _) => Some("sport"),
            (SiteCategory::Textures, _) => Some("textures"),
            (SiteCategory::Vector, _) => Some("vector"),
            (SiteCategory::Words, _) => Some("words"),
            (SiteCategory::Any, _) => None,
        }
    }

    /// 显示名对应的 i18n 词条键
    pub fn display_name(&self) -> &'static str {
        match self {
            SiteCategory::Any => "online-wallpapers.site-category-any",
            SiteCategory::C3d => "online-wallpapers.site-category-3d",
            SiteCategory::Abstract => "online-wallpapers.site-category-abstract",
            SiteCategory::Animals => "online-wallpapers.site-category-animals",
            SiteCategory::Anime => "online-wallpapers.site-category-anime",
            SiteCategory::Art => "online-wallpapers.site-category-art",
            SiteCategory::Black => "online-wallpapers.site-category-black",
            SiteCategory::BlackAndWhite => "online-wallpapers.site-category-black-and-white",
            SiteCategory::Cars => "online-wallpapers.site-category-cars",
            SiteCategory::City => "online-wallpapers.site-category-city",
            SiteCategory::Dark => "online-wallpapers.site-category-dark",
            SiteCategory::Fantasy => "online-wallpapers.site-category-fantasy",
            SiteCategory::Flowers => "online-wallpapers.site-category-flowers",
            SiteCategory::Food => "online-wallpapers.site-category-food",
            SiteCategory::Game => "online-wallpapers.site-category-game",
            SiteCategory::Girl => "online-wallpapers.site-category-girl",
            SiteCategory::HiTech => "online-wallpapers.site-category-hi-tech",
            SiteCategory::Holidays => "online-wallpapers.site-category-holidays",
            SiteCategory::Love => "online-wallpapers.site-category-love",
            SiteCategory::Macro => "online-wallpapers.site-category-macro",
            SiteCategory::Minimalism => "online-wallpapers.site-category-minimalism",
            SiteCategory::Motorcycles => "online-wallpapers.site-category-motorcycles",
            SiteCategory::Music => "online-wallpapers.site-category-music",
            SiteCategory::Nature => "online-wallpapers.site-category-nature",
            SiteCategory::Other => "online-wallpapers.site-category-other",
            SiteCategory::Space => "online-wallpapers.site-category-space",
            SiteCategory::Sport => "online-wallpapers.site-category-sport",
            SiteCategory::Textures => "online-wallpapers.site-category-textures",
            SiteCategory::Vector => "online-wallpapers.site-category-vector",
            SiteCategory::Words => "online-wallpapers.site-category-words",
        }
    }

    /// 解析配置取值；未知取值回退到 Any，不会失败
    pub fn parse(s: &str) -> Self {
        match s {
            "3d" => SiteCategory::C3d,
            "abstract" => SiteCategory::Abstract,
            "animals" | "animal" => SiteCategory::Animals,
            "anime" => SiteCategory::Anime,
            "art" => SiteCategory::Art,
            "black" => SiteCategory::Black,
            "black_and_white" => SiteCategory::BlackAndWhite,
            "cars" | "car" => SiteCategory::Cars,
            "city" => SiteCategory::City,
            "dark" => SiteCategory::Dark,
            "fantasy" => SiteCategory::Fantasy,
            "flowers" => SiteCategory::Flowers,
            "food" => SiteCategory::Food,
            "game" => SiteCategory::Game,
            "girl" => SiteCategory::Girl,
            "hi-tech" => SiteCategory::HiTech,
            "holidays" => SiteCategory::Holidays,
            "love" => SiteCategory::Love,
            "macro" => SiteCategory::Macro,
            "minimalism" => SiteCategory::Minimalism,
            "motorcycles" => SiteCategory::Motorcycles,
            "music" => SiteCategory::Music,
            "nature" => SiteCategory::Nature,
            "other" => SiteCategory::Other,
            "space" => SiteCategory::Space,
            "sport" => SiteCategory::Sport,
            "textures" => SiteCategory::Textures,
            "vector" => SiteCategory::Vector,
            "words" => SiteCategory::Words,
            _ => SiteCategory::Any,
        }
    }
}
