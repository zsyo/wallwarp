// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏项设为壁纸 / 下载 / 在文件夹中查看

use crate::services::wallhaven;
use crate::ui::online::OnlineFileHit;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use tracing::info;

impl App {
    /// 收藏项设为壁纸
    ///
    /// 本地项：直接应用；在线项：库/缓存命中则应用，
    /// 否则加入下载队列并在完成后自动应用（pending_apply_filename）
    pub(in crate::ui::favorites) fn apply_favorite_entry(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };

        if entry.fav.kind == crate::services::database::KIND_LOCAL {
            let full_path = helpers::get_absolute_path(&entry.fav.path);
            return self.apply_wallpaper(full_path);
        }

        // 在线项
        let url = entry.fav.url.clone();
        let id = entry.fav.wallhaven_id.clone();
        let file_type = entry.fav.file_type.clone();
        let file_size = entry.fav.file_size.max(0) as u64;

        let location = self.resolve_online_file(&url, &id, &file_type, file_size);

        match location.source {
            Some(OnlineFileHit::InData) => {
                let full_path =
                    helpers::get_absolute_path(&location.target_path.to_string_lossy());
                self.apply_wallpaper(full_path)
            }
            Some(OnlineFileHit::InCache(cache_file_path)) => {
                self.apply_wallpaper_after_copy(cache_file_path, &location.target_path)
            }
            None => {
                // 文件不存在：下载完成后自动设为壁纸
                let file_name = wallhaven::generate_file_name(
                    &id,
                    file_type.split('/').next_back().unwrap_or("jpg"),
                );
                self.favorites_state.pending_apply_filename = Some(file_name.clone());

                info!("[收藏夹] [ID:{}] 文件未入库，下载后自动设为壁纸: {}", id, file_name);

                let downloading_message = self
                    .i18n
                    .t("download-tasks.downloading-for-wallpaper")
                    .to_string();
                let download_task = if self.has_active_download_for(&url) {
                    Task::none()
                } else {
                    self.start_download(url, &id, &file_type)
                };

                Task::batch([
                    download_task,
                    self.show_notification(downloading_message, NotificationType::Info),
                ])
            }
        }
    }

    /// 下载在线收藏项到壁纸库（不自动设壁纸）
    pub(in crate::ui::favorites) fn download_favorite_entry(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };
        if entry.fav.kind != crate::services::database::KIND_ONLINE {
            return Task::none();
        }

        let url = entry.fav.url.clone();
        let id = entry.fav.wallhaven_id.clone();
        let file_type = entry.fav.file_type.clone();
        let file_size = entry.fav.file_size.max(0) as u64;

        let location = self.resolve_online_file(&url, &id, &file_type, file_size);

        match location.source {
            Some(OnlineFileHit::InData) => {
                let file_name = wallhaven::generate_file_name(
                    &id,
                    file_type.split('/').next_back().unwrap_or("jpg"),
                );
                let success_message = format!(
                    "{}: {}",
                    self.i18n.t("download-tasks.file-already-exists"),
                    file_name
                );
                self.show_notification(success_message, NotificationType::Info)
            }
            Some(OnlineFileHit::InCache(cache_file_path)) => {
                // 缓存命中：复制到壁纸库
                let file_name = wallhaven::generate_file_name(
                    &id,
                    file_type.split('/').next_back().unwrap_or("jpg"),
                );
                let success_message = format!(
                    "{}: {}",
                    self.i18n.t("download-tasks.copied-from-cache"),
                    file_name
                );
                let copy_failed_message =
                    self.i18n.t("download-tasks.copy-failed").to_string();
                let target = location.target_path.to_string_lossy().to_string();
                self.local_state.loaded_data_path = None;
                Task::perform(
                    crate::services::async_task::async_copy_file(cache_file_path, target),
                    move |result| match result {
                        Ok(()) => crate::ui::main::MainMessage::ShowNotification(
                            success_message,
                            NotificationType::Success,
                        )
                        .into(),
                        Err(e) => {
                            tracing::error!("[收藏夹] [ID:{}] 从缓存复制失败: {}", id, e);
                            crate::ui::main::MainMessage::ShowNotification(
                                format!("{}: {}", copy_failed_message, e),
                                NotificationType::Error,
                            )
                            .into()
                        }
                    },
                )
            }
            None => {
                if self.has_active_download_for(&url) {
                    let info_message = self
                        .i18n
                        .t("download-tasks.task-already-in-queue")
                        .to_string();
                    return self.show_notification(info_message, NotificationType::Info);
                }

                let add_to_queue_message = self
                    .i18n
                    .t("download-tasks.added-to-download-queue")
                    .to_string();
                let download_task = self.start_download(url, &id, &file_type);

                Task::batch([
                    download_task,
                    self.show_notification(add_to_queue_message, NotificationType::Info),
                ])
            }
        }
    }

    /// 在文件夹中查看本地收藏项（打开目录并选中文件）
    pub(in crate::ui::favorites) fn view_favorite_file(&mut self, index: usize) -> Task<AppMessage> {
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };
        if entry.fav.kind != crate::services::database::KIND_LOCAL {
            return Task::none();
        }

        let full_path = helpers::get_absolute_path(&entry.fav.path);

        // 检查文件是否存在
        if !std::path::Path::new(&full_path).exists() {
            let message = self
                .i18n
                .t_with_args("notification.file-deleted", &[("path", full_path.clone())]);
            return self.show_notification(message, NotificationType::Error);
        }

        helpers::open_file_in_explorer(&full_path);
        Task::none()
    }
}
