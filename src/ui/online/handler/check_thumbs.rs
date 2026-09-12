// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::common::cached_image_missing;
use crate::ui::online::{OnlineMessage, WallpaperLoadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

impl App {
    /// 检查已加载缩略图的缓存文件是否仍存在，失效项重载
    ///
    /// 进入页面时触发：缓存目录被清空（应用内清空或外部删除）或路径变更后，
    /// 已加载项的缓存文件丢失，需置回加载中并重发缩略图任务
    /// （在线缩略图加载自带缓存命中判断，文件存在则直接复用，不会重复下载）
    pub(in crate::ui::online) fn online_check_thumbs(&mut self) -> Task<AppMessage> {
        let force_reload = self.online_state.thumbs_stale;
        self.online_state.thumbs_stale = false;

        // 模态大图缓存丢失时直接关闭模态（大图走流式下载，重新打开即可重新加载）
        if self.online_state.modal_visible
            && self
                .online_state
                .modal_image_handle
                .as_ref()
                .is_some_and(cached_image_missing)
        {
            self.online_state.modal_visible = false;
            self.online_state.modal_image_handle = None;
            self.online_state.cancel_modal_download();
        }

        // 先收集失效项索引，避免遍历中同时借用 wallpapers 与 wallpapers_data
        let stale_indexes: Vec<usize> = self
            .online_state
            .wallpapers
            .iter()
            .enumerate()
            .filter(|(idx, status)| {
                if !matches!(status, WallpaperLoadStatus::Loaded) {
                    return false;
                }
                if force_reload {
                    return true;
                }
                self.online_state.wallpapers_data[*idx]
                    .image_handle
                    .as_ref()
                    .is_some_and(cached_image_missing)
            })
            .map(|(idx, _)| idx)
            .collect();

        let proxy = self.config.resolved_proxy();
        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for idx in stale_indexes {
            let Some(wallpaper) = self.online_state.wallpapers_data.get(idx) else {
                continue;
            };
            let url = wallpaper.thumb_large.clone();
            let file_size = wallpaper.file_size;

            self.online_state.wallpapers[idx] = WallpaperLoadStatus::Loading;
            if let Some(data) = self.online_state.wallpapers_data.get_mut(idx) {
                data.image_handle = None;
            }
            let cancel_token = Arc::new(AtomicBool::new(false));
            self.online_state
                .thumb_load_cancel_tokens
                .push(cancel_token.clone());

            tasks.push(Task::perform(
                async_task::async_load_online_wallpaper_thumb_with_cache_with_cancel(
                    url,
                    file_size,
                    cache_path.clone(),
                    proxy.clone(),
                    cancel_token,
                ),
                move |result| match result {
                    Ok(handle) => OnlineMessage::ThumbLoaded(idx, handle).into(),
                    Err(_) => OnlineMessage::ThumbLoadFailed(idx).into(),
                },
            ));
        }

        Task::batch(tasks)
    }
}
