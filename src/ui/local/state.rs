// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::message::WallpaperLoadStatus;

#[derive(Debug)]
pub struct LocalState {
    pub wallpapers: Vec<WallpaperLoadStatus>,
    pub all_paths: Vec<String>,
    pub loading_page: bool,
    pub current_page: usize,
    pub page_size: usize,
    pub total_count: usize,
    pub modal_visible: bool,
    pub current_image_index: usize,
    pub delete_confirm_visible: bool,
    pub delete_target_index: Option<usize>,
    pub modal_image_handle: Option<iced::widget::image::Handle>,
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            wallpapers: Vec::new(),
            all_paths: Vec::new(),
            loading_page: false,
            current_page: 0,
            page_size: 20,
            total_count: 0,
            modal_visible: false,
            current_image_index: 0,
            delete_confirm_visible: false,
            delete_target_index: None,
            modal_image_handle: None,
        }
    }
}

impl LocalState {
    /// 查找下一个有效的图片索引
    pub fn find_next_valid_image_index(&self, start_index: usize, direction: i32) -> Option<usize> {
        if self.all_paths.is_empty() {
            return None;
        }

        let total = self.all_paths.len();
        let mut current_index = start_index;
        let loop_count = total; // 防止无限循环

        for _ in 0..loop_count {
            // 根据方向更新索引
            if direction > 0 {
                // 向前查找
                current_index = if current_index < total - 1 {
                    current_index + 1
                } else {
                    0
                };
            } else {
                // 向后查找
                current_index = if current_index > 0 {
                    current_index - 1
                } else {
                    total - 1
                };
            }

            // 检查是否回到起始位置
            if current_index == start_index {
                return None;
            }

            // 检查当前索引是否有效
            if let Some(wallpaper_status) = self.wallpapers.get(current_index) {
                if let WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                    if wallpaper.name != "加载失败" {
                        return Some(current_index);
                    }
                }
            }
        }

        None
    }
}