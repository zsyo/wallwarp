// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Basic 档图源共用的颜色筛选枚举
//!
//! 取 Pixabay colors 与 Unsplash color 两个参数的能力并集，各图源 service
//! 层按自己的取值表映射（如图源无对应语义则忽略该筛选，见各 service 的
//! map_color）；Wallhaven 的 29 色板仍用 [`super::filters::ColorOption`]

use serde::{Deserialize, Serialize};

/// Basic 档图源颜色筛选（Any 为不限）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceColor {
    /// 不限颜色
    Any,
    /// 黑白（Pixabay grayscale / Unsplash black_and_white）
    Gray,
    Red,
    Orange,
    Yellow,
    Green,
    /// 青（Pixabay turquoise / Unsplash teal）
    Teal,
    Blue,
    /// 紫（Pixabay lilac / Unsplash purple）
    Purple,
    /// 粉（Pixabay pink / Unsplash magenta）
    Pink,
    Brown,
    Black,
    White,
}

impl SourceColor {
    pub fn all() -> [SourceColor; 13] {
        [
            SourceColor::Any,
            SourceColor::Gray,
            SourceColor::Red,
            SourceColor::Orange,
            SourceColor::Yellow,
            SourceColor::Green,
            SourceColor::Teal,
            SourceColor::Blue,
            SourceColor::Purple,
            SourceColor::Pink,
            SourceColor::Brown,
            SourceColor::Black,
            SourceColor::White,
        ]
    }

    /// 配置与壁纸组快照的统一取值（各图源参数值由 service 层映射）
    pub fn value(&self) -> &'static str {
        match self {
            SourceColor::Any => "any",
            SourceColor::Gray => "gray",
            SourceColor::Red => "red",
            SourceColor::Orange => "orange",
            SourceColor::Yellow => "yellow",
            SourceColor::Green => "green",
            SourceColor::Teal => "teal",
            SourceColor::Blue => "blue",
            SourceColor::Purple => "purple",
            SourceColor::Pink => "pink",
            SourceColor::Brown => "brown",
            SourceColor::Black => "black",
            SourceColor::White => "white",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SourceColor::Any => "online-wallpapers.source-color-any",
            SourceColor::Gray => "online-wallpapers.source-color-gray",
            SourceColor::Red => "online-wallpapers.source-color-red",
            SourceColor::Orange => "online-wallpapers.source-color-orange",
            SourceColor::Yellow => "online-wallpapers.source-color-yellow",
            SourceColor::Green => "online-wallpapers.source-color-green",
            SourceColor::Teal => "online-wallpapers.source-color-teal",
            SourceColor::Blue => "online-wallpapers.source-color-blue",
            SourceColor::Purple => "online-wallpapers.source-color-purple",
            SourceColor::Pink => "online-wallpapers.source-color-pink",
            SourceColor::Brown => "online-wallpapers.source-color-brown",
            SourceColor::Black => "online-wallpapers.source-color-black",
            SourceColor::White => "online-wallpapers.source-color-white",
        }
    }

    /// 解析配置取值；未知取值回退到 Any，不会失败
    pub fn parse(s: &str) -> Self {
        match s {
            "any" => SourceColor::Any,
            "gray" => SourceColor::Gray,
            "red" => SourceColor::Red,
            "orange" => SourceColor::Orange,
            "yellow" => SourceColor::Yellow,
            "green" => SourceColor::Green,
            "teal" => SourceColor::Teal,
            "blue" => SourceColor::Blue,
            "purple" => SourceColor::Purple,
            "pink" => SourceColor::Pink,
            "brown" => SourceColor::Brown,
            "black" => SourceColor::Black,
            "white" => SourceColor::White,
            _ => SourceColor::Any,
        }
    }
}
