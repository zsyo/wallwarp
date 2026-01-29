// Copyright (C) 2026 zsyo - GNU AGPL v3.0

// 分辨率选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Any,
    Standard,
    Wide,
    Ultrawide,
    // Ultrawide 分辨率
    R2560x1080,
    R3440x1440,
    R3840x1600,
    R2560x1440U,
    R3840x2160U,
    // 16:9 分辨率
    R1280x720,
    R1600x900,
    R1920x1080,
    R2560x1440,
    R3840x2160,
    // 16:10 分辨率
    R1280x800,
    R1600x1000,
    R1920x1200,
    R2560x1600,
    R3840x2400,
    // 4:3 分辨率
    R1280x960,
    R1600x1200_4_3,
    R1920x1440,
    R2560x1920,
    R3840x2880,
    // 5:4 分辨率
    R1280x1024,
    R1600x1280,
    R1920x1536,
    R2560x2048,
    R3840x3072,
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

    pub fn all_detailed() -> [Resolution; 25] {
        [
            Resolution::R2560x1080,
            Resolution::R3440x1440,
            Resolution::R3840x1600,
            Resolution::R2560x1440U,
            Resolution::R3840x2160U,
            Resolution::R1280x720,
            Resolution::R1600x900,
            Resolution::R1920x1080,
            Resolution::R2560x1440,
            Resolution::R3840x2160,
            Resolution::R1280x800,
            Resolution::R1600x1000,
            Resolution::R1920x1200,
            Resolution::R2560x1600,
            Resolution::R3840x2400,
            Resolution::R1280x960,
            Resolution::R1600x1200_4_3,
            Resolution::R1920x1440,
            Resolution::R2560x1920,
            Resolution::R3840x2880,
            Resolution::R1280x1024,
            Resolution::R1600x1280,
            Resolution::R1920x1536,
            Resolution::R2560x2048,
            Resolution::R3840x3072,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            Resolution::Any => "any",
            Resolution::Standard => "standard",
            Resolution::Wide => "wide",
            Resolution::Ultrawide => "ultrawide",
            Resolution::R2560x1080 => "2560x1080",
            Resolution::R3440x1440 => "3440x1440",
            Resolution::R3840x1600 => "3840x1600",
            Resolution::R2560x1440U => "2560x1440",
            Resolution::R3840x2160U => "3840x2160",
            Resolution::R1280x720 => "1280x720",
            Resolution::R1600x900 => "1600x900",
            Resolution::R1920x1080 => "1920x1080",
            Resolution::R2560x1440 => "2560x1440",
            Resolution::R3840x2160 => "3840x2160",
            Resolution::R1280x800 => "1280x800",
            Resolution::R1600x1000 => "1600x1000",
            Resolution::R1920x1200 => "1920x1200",
            Resolution::R2560x1600 => "2560x1600",
            Resolution::R3840x2400 => "3840x2400",
            Resolution::R1280x960 => "1280x960",
            Resolution::R1600x1200_4_3 => "1600x1200",
            Resolution::R1920x1440 => "1920x1440",
            Resolution::R2560x1920 => "2560x1920",
            Resolution::R3840x2880 => "3840x2880",
            Resolution::R1280x1024 => "1280x1024",
            Resolution::R1600x1280 => "1600x1280",
            Resolution::R1920x1536 => "1920x1536",
            Resolution::R2560x2048 => "2560x2048",
            Resolution::R3840x3072 => "3840x3072",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Resolution::Any => "online-wallpapers.resolution-any",
            Resolution::Standard => "online-wallpapers.resolution-standard",
            Resolution::Wide => "online-wallpapers.resolution-wide",
            Resolution::Ultrawide => "online-wallpapers.resolution-ultrawide",
            Resolution::R2560x1080 => "2560x1080",
            Resolution::R3440x1440 => "3440x1440",
            Resolution::R3840x1600 => "3840x1600",
            Resolution::R2560x1440U => "2560x1440",
            Resolution::R3840x2160U => "3840x2160",
            Resolution::R1280x720 => "1280x720",
            Resolution::R1600x900 => "1600x900",
            Resolution::R1920x1080 => "1920x1080",
            Resolution::R2560x1440 => "2560x1440",
            Resolution::R3840x2160 => "3840x2160",
            Resolution::R1280x800 => "1280x800",
            Resolution::R1600x1000 => "1600x1000",
            Resolution::R1920x1200 => "1920x1200",
            Resolution::R2560x1600 => "2560x1600",
            Resolution::R3840x2400 => "3840x2400",
            Resolution::R1280x960 => "1280x960",
            Resolution::R1600x1200_4_3 => "1600x1200",
            Resolution::R1920x1440 => "1920x1440",
            Resolution::R2560x1920 => "2560x1920",
            Resolution::R3840x2880 => "3840x2880",
            Resolution::R1280x1024 => "1280x1024",
            Resolution::R1600x1280 => "1600x1280",
            Resolution::R1920x1536 => "1920x1536",
            Resolution::R2560x2048 => "2560x2048",
            Resolution::R3840x3072 => "3840x3072",
        }
    }

    pub fn get_pixel_count(&self) -> u64 {
        match self {
            Resolution::Any => 0,
            Resolution::Standard => 0,
            Resolution::Wide => 0,
            Resolution::Ultrawide => 0,
            Resolution::R2560x1080 => 2560 * 1080,
            Resolution::R3440x1440 => 3440 * 1440,
            Resolution::R3840x1600 => 3840 * 1600,
            Resolution::R2560x1440U => 2560 * 1440,
            Resolution::R3840x2160U => 3840 * 2160,
            Resolution::R1280x720 => 1280 * 720,
            Resolution::R1600x900 => 1600 * 900,
            Resolution::R1920x1080 => 1920 * 1080,
            Resolution::R2560x1440 => 2560 * 1440,
            Resolution::R3840x2160 => 3840 * 2160,
            Resolution::R1280x800 => 1280 * 800,
            Resolution::R1600x1000 => 1600 * 1000,
            Resolution::R1920x1200 => 1920 * 1200,
            Resolution::R2560x1600 => 2560 * 1600,
            Resolution::R3840x2400 => 3840 * 2400,
            Resolution::R1280x960 => 1280 * 960,
            Resolution::R1600x1200_4_3 => 1600 * 1200,
            Resolution::R1920x1440 => 1920 * 1440,
            Resolution::R2560x1920 => 2560 * 1920,
            Resolution::R3840x2880 => 3840 * 2880,
            Resolution::R1280x1024 => 1280 * 1024,
            Resolution::R1600x1280 => 1600 * 1280,
            Resolution::R1920x1536 => 1920 * 1536,
            Resolution::R2560x2048 => 2560 * 2048,
            Resolution::R3840x3072 => 3840 * 3072,
        }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
