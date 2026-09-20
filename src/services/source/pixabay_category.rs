// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Pixabay 内容分类筛选枚举（官方 API category 参数取值表）
//!
//! 仅 Pixabay 支持该维度；其余图源无对应能力，统一 [`super::SearchQuery`]
//! 中以 Option 携带，None 表示不限

use serde::{Deserialize, Serialize};

/// Pixabay 内容分类（Any 为不限）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PixabayCategory {
    /// 不限分类
    Any,
    Backgrounds,
    Fashion,
    Nature,
    Science,
    Education,
    Feelings,
    Health,
    People,
    Religion,
    Places,
    Animals,
    Industry,
    Computer,
    Food,
    Sports,
    Transportation,
    Travel,
    Buildings,
    Business,
    Music,
}

impl PixabayCategory {
    pub fn all() -> [PixabayCategory; 21] {
        [
            PixabayCategory::Any,
            PixabayCategory::Backgrounds,
            PixabayCategory::Fashion,
            PixabayCategory::Nature,
            PixabayCategory::Science,
            PixabayCategory::Education,
            PixabayCategory::Feelings,
            PixabayCategory::Health,
            PixabayCategory::People,
            PixabayCategory::Religion,
            PixabayCategory::Places,
            PixabayCategory::Animals,
            PixabayCategory::Industry,
            PixabayCategory::Computer,
            PixabayCategory::Food,
            PixabayCategory::Sports,
            PixabayCategory::Transportation,
            PixabayCategory::Travel,
            PixabayCategory::Buildings,
            PixabayCategory::Business,
            PixabayCategory::Music,
        ]
    }

    /// Pixabay API category 参数值（None 表示不限，不携带参数）
    pub fn api_value(&self) -> Option<&'static str> {
        match self {
            PixabayCategory::Any => None,
            PixabayCategory::Backgrounds => Some("backgrounds"),
            PixabayCategory::Fashion => Some("fashion"),
            PixabayCategory::Nature => Some("nature"),
            PixabayCategory::Science => Some("science"),
            PixabayCategory::Education => Some("education"),
            PixabayCategory::Feelings => Some("feelings"),
            PixabayCategory::Health => Some("health"),
            PixabayCategory::People => Some("people"),
            PixabayCategory::Religion => Some("religion"),
            PixabayCategory::Places => Some("places"),
            PixabayCategory::Animals => Some("animals"),
            PixabayCategory::Industry => Some("industry"),
            PixabayCategory::Computer => Some("computer"),
            PixabayCategory::Food => Some("food"),
            PixabayCategory::Sports => Some("sports"),
            PixabayCategory::Transportation => Some("transportation"),
            PixabayCategory::Travel => Some("travel"),
            PixabayCategory::Buildings => Some("buildings"),
            PixabayCategory::Business => Some("business"),
            PixabayCategory::Music => Some("music"),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            PixabayCategory::Any => "online-wallpapers.pixabay-category-any",
            PixabayCategory::Backgrounds => "online-wallpapers.pixabay-category-backgrounds",
            PixabayCategory::Fashion => "online-wallpapers.pixabay-category-fashion",
            PixabayCategory::Nature => "online-wallpapers.pixabay-category-nature",
            PixabayCategory::Science => "online-wallpapers.pixabay-category-science",
            PixabayCategory::Education => "online-wallpapers.pixabay-category-education",
            PixabayCategory::Feelings => "online-wallpapers.pixabay-category-feelings",
            PixabayCategory::Health => "online-wallpapers.pixabay-category-health",
            PixabayCategory::People => "online-wallpapers.pixabay-category-people",
            PixabayCategory::Religion => "online-wallpapers.pixabay-category-religion",
            PixabayCategory::Places => "online-wallpapers.pixabay-category-places",
            PixabayCategory::Animals => "online-wallpapers.pixabay-category-animals",
            PixabayCategory::Industry => "online-wallpapers.pixabay-category-industry",
            PixabayCategory::Computer => "online-wallpapers.pixabay-category-computer",
            PixabayCategory::Food => "online-wallpapers.pixabay-category-food",
            PixabayCategory::Sports => "online-wallpapers.pixabay-category-sports",
            PixabayCategory::Transportation => "online-wallpapers.pixabay-category-transportation",
            PixabayCategory::Travel => "online-wallpapers.pixabay-category-travel",
            PixabayCategory::Buildings => "online-wallpapers.pixabay-category-buildings",
            PixabayCategory::Business => "online-wallpapers.pixabay-category-business",
            PixabayCategory::Music => "online-wallpapers.pixabay-category-music",
        }
    }

    /// 解析配置取值；未知取值回退到 Any，不会失败
    pub fn parse(s: &str) -> Self {
        match s {
            "backgrounds" => PixabayCategory::Backgrounds,
            "fashion" => PixabayCategory::Fashion,
            "nature" => PixabayCategory::Nature,
            "science" => PixabayCategory::Science,
            "education" => PixabayCategory::Education,
            "feelings" => PixabayCategory::Feelings,
            "health" => PixabayCategory::Health,
            "people" => PixabayCategory::People,
            "religion" => PixabayCategory::Religion,
            "places" => PixabayCategory::Places,
            "animals" => PixabayCategory::Animals,
            "industry" => PixabayCategory::Industry,
            "computer" => PixabayCategory::Computer,
            "food" => PixabayCategory::Food,
            "sports" => PixabayCategory::Sports,
            "transportation" => PixabayCategory::Transportation,
            "travel" => PixabayCategory::Travel,
            "buildings" => PixabayCategory::Buildings,
            "business" => PixabayCategory::Business,
            "music" => PixabayCategory::Music,
            _ => PixabayCategory::Any,
        }
    }
}
