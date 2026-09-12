// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::cached_image_missing;
use crate::ui::favorites::state::ThumbState;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 检查已加载缩略图的缓存文件是否仍存在，失效项重载
    ///
    /// 进入页面时触发：缓存目录被清空（应用内清空或外部删除）或路径变更后，
    /// 已加载项的缩略图缓存丢失，需置回加载中并重新加载
    /// （缩略图加载自带缓存命中判断，文件存在则直接复用，不会重复下载/生成）
    pub(in crate::ui::favorites) fn favorites_check_thumbs(&mut self) -> Task<AppMessage> {
        let force_reload = self.favorites_state.thumbs_stale;
        self.favorites_state.thumbs_stale = false;

        // 预览原图缓存丢失时直接关闭模态（重新打开即可重新加载）
        if self.favorites_state.modal_visible
            && self
                .favorites_state
                .modal_handle
                .as_ref()
                .is_some_and(cached_image_missing)
        {
            self.favorites_state.close_modal();
        }

        let proxy = self.config.resolved_proxy();
        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (index, thumb) in self.favorites_state.thumbs.iter_mut().enumerate() {
            let ThumbState::Loaded(handle) = thumb else {
                continue;
            };
            if !force_reload && !cached_image_missing(handle) {
                continue;
            }

            let Some(entry) = self.favorites_state.entries.get(index) else {
                continue;
            };
            // 本地项源文件已失效：直接标记失败（卡片显示失效占位）
            if entry.fav.kind == crate::services::database::KIND_LOCAL && !entry.in_library {
                *thumb = ThumbState::Failed;
                continue;
            }

            *thumb = ThumbState::Loading;
            tasks.push(Self::favorite_thumb_task(index, entry, &proxy, &cache_path));
        }

        Task::batch(tasks)
    }
}
