// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::services::wallhaven;
use crate::ui::async_tasks;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::PathBuf;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn set_online_wallpaper(&mut self, index: usize) -> Task<AppMessage> {
        // 设为壁纸
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
                        async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                        move |result| match result {
                            Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                            Err(e) => AppMessage::ShowNotification(
                                format!("{}: {}", failed_message, e),
                                NotificationType::Error,
                            ),
                        },
                    );
                }
            }

            // 2. 检查缓存文件是否存在且大小匹配
            let cache_path = self.config.data.cache_path.clone();
            if let Ok(cache_file_path) =
                DownloadService::get_online_image_cache_final_path(&cache_path, &url, file_size)
            {
                if let Ok(metadata) = std::fs::metadata(&cache_file_path) {
                    let cache_size = metadata.len();
                    if cache_size == file_size {
                        // 缓存文件存在且大小匹配，复制到 data_path
                        let _ = std::fs::create_dir_all(&data_path);
                        match std::fs::copy(&cache_file_path, &target_path) {
                            Ok(_) => {
                                // 复制成功，设置壁纸
                                let full_path = helpers::get_absolute_path(&target_path.to_string_lossy().to_string());
                                let wallpaper_mode = self.config.wallpaper.mode;
                                let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                                return Task::perform(
                                    async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                                    move |result| match result {
                                        Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                                        Err(e) => AppMessage::ShowNotification(
                                            format!("{}: {}", failed_message, e),
                                            NotificationType::Error,
                                        ),
                                    },
                                );
                            }
                            Err(e) => {
                                error!("[在线壁纸] [ID:{}] 从缓存复制失败: {}", id, e);
                                // 复制失败，继续走下载流程
                            }
                        }
                    }
                }
            }

            // 3. 文件不存在，启动下载任务
            // 设置待设置壁纸的文件名
            self.online_state.pending_set_wallpaper_filename = Some(file_name.clone());

            // 检查下载任务列表中是否已有相同 URL 的任务
            let has_duplicate = self.download_state.tasks.iter().any(|task| {
                task.task.url == url
                    && task.task.status != DownloadStatus::Completed
                    && task.task.status != DownloadStatus::Cancelled
                    && !matches!(task.task.status, DownloadStatus::Failed(_))
            });

            if has_duplicate {
                // 任务已在下载队列中，只更新待设置壁纸的文件名
                let info_message = self.i18n.t("download-tasks.task-already-in-queue").to_string();
                return Task::done(AppMessage::ShowNotification(info_message, NotificationType::Info));
            }

            // 开始下载
            return self.start_download(url, &id, &file_type);
        }

        Task::none()
    }
}
