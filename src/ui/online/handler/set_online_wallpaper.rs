// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::online::handler::resolve_online_file::OnlineFileHit;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    pub(in crate::ui::online) fn set_online_wallpaper(&mut self, index: usize) -> Task<AppMessage> {
        // 设为壁纸
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
                    // 文件不存在，启动下载任务
                    // 设置待设置壁纸的文件名
                    let file_name = wallhaven::generate_file_name(
                        &id,
                        file_type.split('/').next_back().unwrap_or("jpg"),
                    );
                    self.online_state.pending_set_wallpaper_filename = Some(file_name);

                    // 检查下载任务列表中是否已有相同 URL 的任务
                    if self.has_active_download_for(&url) {
                        let downloading_message = self
                            .i18n
                            .t("download-tasks.downloading-for-wallpaper")
                            .to_string();
                        return self.show_notification(downloading_message, NotificationType::Info);
                    }

                    // 开始下载
                    let downloading_message = self
                        .i18n
                        .t("download-tasks.downloading-for-wallpaper")
                        .to_string();
                    let download_task = self.start_download(url, &id, &file_type);

                    // 显示正在下载以完成设置的通知
                    return Task::batch([
                        download_task,
                        self.show_notification(downloading_message, NotificationType::Info),
                    ]);
                }
            };
        }

        Task::none()
    }
}
