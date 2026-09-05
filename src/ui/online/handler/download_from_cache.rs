// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::wallhaven;
use crate::ui::main::MainMessage;
use crate::ui::online::handler::resolve_online_file::OnlineFileHit;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn download_from_cache(&mut self, index: usize) -> Task<AppMessage> {
        // 从缓存复制文件到 data_path
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let id = wallpaper.id.clone();
            let file_type = wallpaper.file_type.clone();
            let file_size = wallpaper.file_size;

            let location = self.resolve_online_file(&url, &id, &file_type, file_size);

            return match location.source {
                Some(OnlineFileHit::InData) => {
                    // 文件已存在于 data_path 中
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
                    // 缓存文件存在且大小匹配，异步复制到 data_path
                    let file_name = wallhaven::generate_file_name(
                        &id,
                        file_type.split('/').next_back().unwrap_or("jpg"),
                    );
                    let success_message = format!(
                        "{}: {}",
                        self.i18n.t("download-tasks.copied-from-cache"),
                        file_name
                    );
                    // 提前获取翻译文本，避免闭包中访问 self
                    let copy_failed_message = self.i18n.t("download-tasks.copy-failed").to_string();
                    let id_for_log = id;
                    let target = location.target_path.to_string_lossy().to_string();
                    Task::perform(
                        async_task::async_copy_file(cache_file_path, target),
                        move |result| match result {
                            Ok(()) => MainMessage::ShowNotification(
                                success_message,
                                NotificationType::Success,
                            )
                            .into(),
                            Err(e) => {
                                error!("[模态窗口下载] [ID:{}] 从缓存复制失败: {}", id_for_log, e);
                                MainMessage::ShowNotification(
                                    format!("{}: {}", copy_failed_message, e),
                                    NotificationType::Error,
                                )
                                .into()
                            }
                        },
                    )
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
