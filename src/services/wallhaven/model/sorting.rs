// Copyright (C) 2026 zsyo - GNU AGPL v3.0

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

    pub fn from_str(s: &str) -> Option<Self> {
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
