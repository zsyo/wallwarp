// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::online::WallpaperLoadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

impl App {
    pub(in crate::ui::online) fn online_thumb_loaded(&mut self, idx: usize, handle: Handle) -> Task<AppMessage> {
        // 缩略图加载完成
        if idx < self.online_state.wallpapers.len() {
            if let Some(wallpaper) = self.online_state.wallpapers_data.get(idx) {
                self.online_state.wallpapers[idx] = WallpaperLoadStatus::ThumbLoaded(wallpaper.clone(), handle);
            }
        }
        Task::none()
    }
}
