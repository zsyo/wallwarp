// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use serde::{Deserialize, Serialize};

// 比例选项（支持多选）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AspectRatio {
    R16x9,
    R16x10,
    R21x9,
    R32x9,
    R48x9,
    R9x16,
    R10x16,
    R9x18,
    R1x1,
    R3x2,
    R4x3,
    R5x4,
}

impl AspectRatio {
    pub fn all() -> [AspectRatio; 12] {
        [
            AspectRatio::R16x9,
            AspectRatio::R16x10,
            AspectRatio::R21x9,
            AspectRatio::R32x9,
            AspectRatio::R48x9,
            AspectRatio::R9x16,
            AspectRatio::R10x16,
            AspectRatio::R9x18,
            AspectRatio::R1x1,
            AspectRatio::R3x2,
            AspectRatio::R4x3,
            AspectRatio::R5x4,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            AspectRatio::R16x9 => "16x9",
            AspectRatio::R16x10 => "16x10",
            AspectRatio::R21x9 => "21x9",
            AspectRatio::R32x9 => "32x9",
            AspectRatio::R48x9 => "48x9",
            AspectRatio::R9x16 => "9x16",
            AspectRatio::R10x16 => "10x16",
            AspectRatio::R9x18 => "9x18",
            AspectRatio::R1x1 => "1x1",
            AspectRatio::R3x2 => "3x2",
            AspectRatio::R4x3 => "4x3",
            AspectRatio::R5x4 => "5x4",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AspectRatio::R16x9 => "16x9",
            AspectRatio::R16x10 => "16x10",
            AspectRatio::R21x9 => "21x9",
            AspectRatio::R32x9 => "32x9",
            AspectRatio::R48x9 => "48x9",
            AspectRatio::R9x16 => "9x16",
            AspectRatio::R10x16 => "10x16",
            AspectRatio::R9x18 => "9x18",
            AspectRatio::R1x1 => "1x1",
            AspectRatio::R3x2 => "3x2",
            AspectRatio::R4x3 => "4x3",
            AspectRatio::R5x4 => "5x4",
        }
    }

    pub fn group(&self) -> AspectRatioGroup {
        match self {
            AspectRatio::R16x9 | AspectRatio::R16x10 => AspectRatioGroup::Wide,
            AspectRatio::R21x9 | AspectRatio::R32x9 | AspectRatio::R48x9 => AspectRatioGroup::Ultrawide,
            AspectRatio::R9x16 | AspectRatio::R10x16 | AspectRatio::R9x18 => AspectRatioGroup::Portrait,
            AspectRatio::R1x1 | AspectRatio::R3x2 | AspectRatio::R4x3 | AspectRatio::R5x4 => AspectRatioGroup::Square,
        }
    }
}

impl std::fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

// 比例分组
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectRatioGroup {
    Wide,
    Ultrawide,
    Portrait,
    Square,
}

impl AspectRatioGroup {
    pub fn display_name(&self) -> &'static str {
        match self {
            AspectRatioGroup::Wide => "online-wallpapers.ratio-group-wide",
            AspectRatioGroup::Ultrawide => "online-wallpapers.ratio-group-ultrawide",
            AspectRatioGroup::Portrait => "online-wallpapers.ratio-group-portrait",
            AspectRatioGroup::Square => "online-wallpapers.ratio-group-square",
        }
    }
}
