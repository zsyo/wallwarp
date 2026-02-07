// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::online::WallpaperLoadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_thumb_loaded(&mut self, idx: usize, handle: iced::widget::image::Handle) -> Task<AppMessage> {
        // 缩略图加载完成，缓存 Handle 到 OnlineWallpaper 中
        if idx < self.online_state.wallpapers.len() {
            if let Some(wallpaper) = self.online_state.wallpapers_data.get_mut(idx) {
                // 缓存 Handle
                wallpaper.image_handle = Some(handle);
                // 更新状态为 Loaded（Handle 已缓存到 wallpaper 中）
                self.online_state.wallpapers[idx] = WallpaperLoadStatus::Loaded(wallpaper.clone());
            }
        }
        Task::none()
    }
}
