// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::online::handler::resolve_online_file::OnlineFileHit;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn download_online_wallpaper(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        // 下载壁纸
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
                    let copy_failed_message =
                        self.i18n.t("download-tasks.copy-failed").to_string();
                    let id_for_log = id;
                    let target = location.target_path.to_string_lossy().to_string();
                    // 壁纸库新增了文件，本地页列表缓存失效
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
                                tracing::error!(
                                    "[在线壁纸] [ID:{}] 从缓存复制失败: {}",
                                    id_for_log,
                                    e
                                );
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
                    // 检查下载任务列表中是否已有相同 URL 的任务
                    if self.has_active_download_for(&url) {
                        // 任务已在下载队列中
                        let info_message = self
                            .i18n
                            .t("download-tasks.task-already-in-queue")
                            .to_string();
                        return self.show_notification(info_message, NotificationType::Info);
                    }

                    // 开始下载
                    let add_to_queue_message = self
                        .i18n
                        .t("download-tasks.added-to-download-queue")
                        .to_string();
                    let download_task = self.start_download(url, &id, &file_type);

                    // 显示添加到下载队列的通知
                    return Task::batch([
                        download_task,
                        self.show_notification(add_to_queue_message, NotificationType::Info),
                    ]);
                }
            };
        }

        Task::none()
    }
}
