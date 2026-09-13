// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Wallhaven API 响应类型
//!
//! 定义 Wallhaven API 返回的数据结构

use crate::services::source::{OnlineWallpaper, SourceKind};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WallhavenResponse<T> {
    pub data: T,
    pub meta: Option<WallhavenMeta>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WallhavenMeta {
    pub current_page: u64,
    pub last_page: u64,
    pub per_page: serde_json::Value,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct WallpaperData {
    pub id: String,
    pub url: String,
    pub path: String,
    pub thumbs: WallhavenThumbs,
    pub dimension_x: u32,
    pub dimension_y: u32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
    pub category: String,
    pub purity: String,
    pub views: u32,
    pub favorites: u32,
    #[serde(default)]
    pub colors: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct WallhavenThumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

impl From<WallpaperData> for OnlineWallpaper {
    fn from(data: WallpaperData) -> Self {
        OnlineWallpaper {
            id: data.id,
            url: data.url,
            path: data.path,
            thumb_large: data.thumbs.large,
            thumb_original: data.thumbs.original,
            thumb_small: data.thumbs.small,
            width: data.dimension_x,
            height: data.dimension_y,
            resolution: data.resolution,
            ratio: data.ratio,
            file_size: data.file_size,
            file_type: data.file_type,
            category: data.category,
            purity: data.purity,
            views: data.views,
            favorites: data.favorites,
            colors: data.colors,
            source: SourceKind::Wallhaven,
            image_handle: None, // Handle 将在后续加载时设置
        }
    }
}
