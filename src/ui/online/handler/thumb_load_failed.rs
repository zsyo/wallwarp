// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::online::WallpaperLoadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::warn;

impl App {
    pub(in crate::ui::online) fn online_thumb_load_failed(
        &mut self,
        idx: usize,
    ) -> Task<AppMessage> {
        // 缩略图加载失败，标记为 Failed（不缓存 Handle，网格显示失败占位卡片）
        if idx < self.online_state.wallpapers.len() {
            warn!("[在线壁纸] [idx:{}] 缩略图加载失败", idx);
            self.online_state.wallpapers[idx] = WallpaperLoadStatus::Failed;
        }
        Task::none()
    }
}
