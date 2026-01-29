// Copyright (C) 2026 zsyo - GNU AGPL v3.0

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
