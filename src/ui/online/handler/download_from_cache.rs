// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::wallhaven;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::path::PathBuf;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn download_from_cache(&mut self, index: usize) -> Task<AppMessage> {
        // 从缓存复制文件到 data_path
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let id = wallpaper.id.clone();
            let file_type = wallpaper.file_type.clone();
            let file_size = wallpaper.file_size;

            // 生成目标文件路径
            let file_name = wallhaven::generate_file_name(&id, file_type.split('/').last().unwrap_or("jpg"));
            let data_path = self.config.data.data_path.clone();
            let target_path = PathBuf::from(&data_path).join(&file_name);

            // 1. 检查目标文件是否已存在于 data_path 中
            if let Ok(metadata) = std::fs::metadata(&target_path) {
                let actual_size = metadata.len();
                if actual_size == file_size {
                    // 文件已存在且大小匹配
                    let success_message = format!(
                        "{}: {}",
                        self.i18n.t("download-tasks.file-already-exists").to_string(),
                        file_name
                    );
                    return Task::done(AppMessage::ShowNotification(success_message, NotificationType::Info));
                }
            }

            // 2. 获取缓存文件路径
            let cache_path = self.config.data.cache_path.clone();
            if let Ok(cache_file_path) =
                DownloadService::get_online_image_cache_final_path(&cache_path, &url, file_size)
            {
                // 检查缓存文件是否存在
                let cache_path_buf = PathBuf::from(&cache_file_path);
                if cache_path_buf.exists() {
                    // 缓存文件存在，复制到 data_path
                    let _ = std::fs::create_dir_all(&data_path);
                    match std::fs::copy(&cache_path_buf, &target_path) {
                        Ok(_) => {
                            let success_message = format!(
                                "{}: {}",
                                self.i18n.t("download-tasks.copied-from-cache").to_string(),
                                file_name
                            );
                            return Task::done(AppMessage::ShowNotification(
                                success_message,
                                NotificationType::Success,
                            ));
                        }
                        Err(e) => {
                            error!("[模态窗口下载] [ID:{}] 从缓存复制失败: {}", id, e);
                            let error_message =
                                format!("{}: {}", self.i18n.t("download-tasks.copy-failed").to_string(), e);
                            return Task::done(AppMessage::ShowNotification(error_message, NotificationType::Error));
                        }
                    }
                } else {
                    // 缓存文件不存在
                    let error_message = self.i18n.t("download-tasks.cache-file-not-found").to_string();
                    return Task::done(AppMessage::ShowNotification(error_message, NotificationType::Error));
                }
            } else {
                // 获取缓存路径失败
                let error_message = self.i18n.t("download-tasks.get-cache-path-failed").to_string();
                return Task::done(AppMessage::ShowNotification(error_message, NotificationType::Error));
            }
        }

        Task::none()
    }
}
