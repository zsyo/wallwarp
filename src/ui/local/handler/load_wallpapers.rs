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
    pub(in crate::ui::local) fn load_local_wallpapers_success(
        &mut self,
        paths: Vec<String>,
    ) -> Task<AppMessage> {
        // 更新本地状态，初始化壁纸加载状态列表
        self.local_state.all_paths = paths;
        self.local_state.loaded_data_path = Some(self.config.data.data_path.clone());
        self.local_state.total_count = self.local_state.all_paths.len();

        // 重置分页游标：重载可能发生在已浏览过若干页之后（如下载完成
        // 置空 loaded_data_path 后再进本页），残留的游标会让后续 LoadPage
        // 误判"没有更多壁纸"直接返回，整页停留在此处初始化的 Loading 状态
        self.local_state.current_page = 0;
        self.local_state.loading_page = false;

        // 初始化壁纸状态为Loading，并加载第一页
        let page_end = std::cmp::min(self.local_state.page_size, self.local_state.total_count);
        self.local_state.wallpapers = vec![WallpaperLoadStatus::Loading; page_end];

        // 触发第一页加载
        Task::done(LocalMessage::LoadPage.into())
    }
}
