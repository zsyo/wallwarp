// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::download::DownloadService;
use crate::services::wallhaven;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::PathBuf;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn set_wallpaper_from_cache(&mut self, index: usize) -> Task<AppMessage> {
        // 从缓存或 data_path 设置壁纸
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
                    // 文件已存在且大小匹配，直接设置壁纸
                    let full_path = helpers::get_absolute_path(&target_path.to_string_lossy().to_string());
                    let wallpaper_mode = self.config.wallpaper.mode;
                    let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                    return Task::perform(
                        async_task::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                        move |result| match result {
                            Ok(_) => MainMessage::AddToWallpaperHistory(full_path).into(),
                            Err(e) => MainMessage::ShowNotification(
                                format!("{}: {}", failed_message, e),
                                NotificationType::Error,
                            )
                            .into(),
                        },
                    );
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
                            // 复制成功，设置壁纸
                            let full_path = helpers::get_absolute_path(&target_path.to_string_lossy().to_string());
                            let wallpaper_mode = self.config.wallpaper.mode;
                            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                            return Task::perform(
                                async_task::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                                move |result| match result {
                                    Ok(_) => MainMessage::AddToWallpaperHistory(full_path).into(),
                                    Err(e) => MainMessage::ShowNotification(
                                        format!("{}: {}", failed_message, e),
                                        NotificationType::Error,
                                    )
                                    .into(),
                                },
                            );
                        }
                        Err(e) => {
                            error!("[模态窗口设置壁纸] [ID:{}] 从缓存复制失败: {}", id, e);
                            let error_message =
                                format!("{}: {}", self.i18n.t("download-tasks.copy-failed").to_string(), e);
                            return self.show_notification(error_message, NotificationType::Error);
                        }
                    }
                } else {
                    // 缓存文件不存在
                    let error_message = self.i18n.t("download-tasks.cache-file-not-found").to_string();
                    return self.show_notification(error_message, NotificationType::Error);
                }
            } else {
                // 获取缓存路径失败
                let error_message = self.i18n.t("download-tasks.get-cache-path-failed").to_string();
                return self.show_notification(error_message, NotificationType::Error);
            }
        }
        Task::none()
    }
}
