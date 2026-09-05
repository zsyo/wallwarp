// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::online::handler::resolve_online_file::OnlineFileHit;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    pub(in crate::ui::online) fn set_wallpaper_from_cache(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        // 从缓存或 data_path 设置壁纸
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let id = wallpaper.id.clone();
            let file_type = wallpaper.file_type.clone();
            let file_size = wallpaper.file_size;

            let location = self.resolve_online_file(&url, &id, &file_type, file_size);

            return match location.source {
                Some(OnlineFileHit::InData) => {
                    // 文件已在壁纸库中，直接设置壁纸
                    let full_path =
                        helpers::get_absolute_path(&location.target_path.to_string_lossy());
                    self.apply_wallpaper(full_path)
                }
                Some(OnlineFileHit::InCache(cache_file_path)) => {
                    // 缓存命中，异步复制到 data_path 后设置壁纸
                    self.apply_wallpaper_after_copy(cache_file_path, &location.target_path)
                }
                None => {
                    // 库与缓存中都没有可用文件
                    let error_message = self
                        .i18n
                        .t("download-tasks.cache-file-not-found")
                        .to_string();
                    self.show_notification(error_message, NotificationType::Error)
                }
            };
        }
        Task::none()
    }
}
