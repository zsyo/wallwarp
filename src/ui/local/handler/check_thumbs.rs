// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::local::Wallpaper;
use crate::ui::common::cached_image_missing;
use crate::ui::local::{LocalMessage, WallpaperLoadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 检查已加载缩略图的缓存文件是否仍存在，失效项重载
    ///
    /// 进入页面时触发：缓存目录被清空（应用内清空或外部删除）或路径变更后，
    /// 已加载项的缩略图缓存丢失，需置回加载中并重发生成
    /// （本地缩略图生成自带"缓存存在则复用"判断，文件存在不会重复生成）
    pub(in crate::ui::local) fn local_check_thumbs(&mut self) -> Task<AppMessage> {
        let force_reload = self.local_state.thumbs_stale;
        self.local_state.thumbs_stale = false;

        // 模态大图文件丢失时直接关闭模态（重新打开即可重新加载）
        if self.local_state.modal_visible
            && self
                .local_state
                .modal_image_handle
                .as_ref()
                .is_some_and(cached_image_missing)
        {
            self.local_state.modal_visible = false;
            self.local_state.modal_image_handle = None;
        }

        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (idx, status) in self.local_state.wallpapers.iter_mut().enumerate() {
            let WallpaperLoadStatus::Loaded(wallpaper) = status else {
                continue;
            };
            if wallpaper.failed {
                continue;
            }
            let handle_missing = wallpaper
                .image_handle
                .as_ref()
                .is_some_and(cached_image_missing)
                || cached_image_missing(&iced::widget::image::Handle::from_path(
                    &wallpaper.thumbnail_path,
                ));
            if !force_reload && !handle_missing {
                continue;
            }

            let path = wallpaper.path.clone();
            *status = WallpaperLoadStatus::Loading;

            tasks.push(Task::perform(
                async_task::async_load_single_wallpaper_with_fallback(
                    path.clone(),
                    cache_path.clone(),
                ),
                move |result| match result {
                    Ok(wallpaper) => LocalMessage::LoadPageSuccess(vec![(idx, wallpaper)]).into(),
                    Err(_) => {
                        let name = std::path::Path::new(&path)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let mut failed_wallpaper = Wallpaper::new(path, name, 0, 0, 0);
                        failed_wallpaper.failed = true;
                        LocalMessage::LoadPageSuccess(vec![(idx, failed_wallpaper)]).into()
                    }
                },
            ));
        }

        Task::batch(tasks)
    }
}
