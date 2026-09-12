// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::common::cached_image_missing;
use crate::ui::history::HistoryMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 检查已加载缩略图的缓存文件是否仍存在，失效项重载
    ///
    /// 进入页面时触发：缓存目录被清空（应用内清空或外部删除）或路径变更后，
    /// 已加载项的缩略图缓存丢失，需置回加载中并重发生成
    /// （本地缩略图生成自带"缓存存在则复用"判断，文件存在不会重复生成）
    pub(in crate::ui) fn history_check_thumbs(&mut self) -> Task<AppMessage> {
        let force_reload = self.history_state.thumbs_stale;
        self.history_state.thumbs_stale = false;

        // 预览原图缓存丢失时直接关闭模态（重新打开即可重新加载）
        if self.history_state.modal_visible
            && self
                .history_state
                .modal_handle
                .as_ref()
                .is_some_and(cached_image_missing)
        {
            self.history_state.close_modal();
        }

        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (index, thumb) in self.history_state.thumbs.iter_mut().enumerate() {
            let Some(handle) = thumb else {
                continue;
            };
            if !force_reload && !cached_image_missing(handle) {
                continue;
            }

            let Some(entry) = self.history_state.entries.get(index) else {
                continue;
            };
            let path = entry.path.clone();
            *thumb = None;

            tasks.push(Task::perform(
                async_task::async_load_single_wallpaper_with_fallback(path, cache_path.clone()),
                move |result| {
                    HistoryMessage::ThumbLoaded {
                        index,
                        wallpaper: result.ok(),
                    }
                    .into()
                },
            ));
        }

        Task::batch(tasks)
    }
}
