// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::local::{LocalMessage, WallpaperLoadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::error;

impl App {
    /// 加载本地壁纸列表
    pub(in crate::ui::local) fn load_local_wallpapers(&mut self) -> Task<AppMessage> {
        let data_path = self.config.data.data_path.clone();
        Task::perform(
            async_task::async_load_wallpaper_paths(data_path),
            |result| match result {
                Ok(paths) => LocalMessage::LoadWallpapersSuccess(paths).into(),
                Err(e) => {
                    error!("[本地壁纸] 加载列表失败: {}", e);
                    AppMessage::None
                }
            },
        )
    }

    /// 处理本地壁纸列表加载成功
    pub(in crate::ui::local) fn load_local_wallpapers_success(&mut self, paths: Vec<String>) -> Task<AppMessage> {
        // 更新本地状态，初始化壁纸加载状态列表
        self.local_state.all_paths = paths;
        self.local_state.total_count = self.local_state.all_paths.len();

        // 初始化壁纸状态为Loading，并加载第一页
        let page_end = std::cmp::min(self.local_state.page_size, self.local_state.total_count);
        self.local_state.wallpapers = vec![WallpaperLoadStatus::Loading; page_end];

        // 触发第一页加载
        Task::done(LocalMessage::LoadPage.into())
    }
}
