// Copyright (C) 2026 zsyo - GNU AGPL v3.0

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
