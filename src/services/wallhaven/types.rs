//! Wallhaven API 响应类型
//!
//! 定义 Wallhaven API 返回的数据结构

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
    #[serde(default)]
    pub tags: Option<Vec<WallhavenTag>>,
}

#[derive(Debug, Deserialize)]
pub struct WallhavenThumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

#[derive(Debug, Deserialize)]
pub struct WallhavenTag {
    pub name: String,
}

/// 在线壁纸数据结构（UI层使用）
#[derive(Debug, Clone)]
pub struct OnlineWallpaper {
    pub id: String,
    pub url: String,
    pub path: String,
    pub thumb_large: String,
    pub thumb_original: String,
    pub thumb_small: String,
    pub width: u32,
    pub height: u32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: u64,
    pub file_type: String,
    pub category: String,
    pub purity: String,
    pub views: u32,
    pub favorites: u32,
    pub colors: Vec<String>,
    pub tags: Vec<String>,
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
            tags: data.tags.unwrap_or_default().into_iter().map(|t| t.name).collect(),
        }
    }
}