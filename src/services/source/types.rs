// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线壁纸条目类型（图源无关）

use super::SourceKind;

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
    /// 来源图源
    pub source: SourceKind,
    /// 缓存的缩略图 Handle，避免每次渲染都重新创建
    pub image_handle: Option<iced::widget::image::Handle>,
}

impl OnlineWallpaper {
    /// 生成下载文件名（委托来源图源的命名规则）
    pub fn download_file_name(&self) -> String {
        self.source.download_file_name(&self.id, &self.file_type)
    }
}
