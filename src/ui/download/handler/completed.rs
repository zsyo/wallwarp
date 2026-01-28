// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::async_tasks;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    pub(in crate::ui::download) fn download_completed(
        &mut self,
        id: usize,
        size: u64,
        error: Option<String>,
    ) -> Task<AppMessage> {
        let task_index = self.download_state.find_task_index(id);
        if let Some(index) = task_index {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                // 检查当前状态
                let current_status = task.task.status.clone();

                if current_status == DownloadStatus::Paused {
                    // 任务已暂停，保持暂停状态
                } else if error.is_some() {
                    // 下载失败
                    let error_msg = error.unwrap();
                    // 检查是否是用户取消
                    if error_msg == "下载已取消" {
                        // 检查任务是否在暂停状态被取消
                        // 如果任务原本是暂停状态，则保持暂停，否则设置为已取消
                        // 如果不是暂停状态，设置为已取消
                        if current_status != DownloadStatus::Paused {
                            task.task.status = DownloadStatus::Cancelled;
                        }
                    } else {
                        task.task.status = DownloadStatus::Failed(error_msg.clone());

                        // 清除未完成的下载文件
                        let url = task.task.url.clone();
                        let save_path = task.task.save_path.clone();

                        // 1. 删除目标文件（data_path中的文件）
                        if let Ok(_metadata) = std::fs::metadata(&save_path) {
                            let _ = std::fs::remove_file(&save_path);
                            tracing::info!("[下载任务] [ID:{}] 已删除未完成的目标文件: {}", id, save_path);
                        }

                        // 2. 删除缓存文件（cache_path/online中的.download文件）
                        let cache_path = self.config.data.cache_path.clone();
                        if let Ok(cache_file_path) =
                            DownloadService::get_online_image_cache_path(&cache_path, &url, size)
                        {
                            if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                                let _ = std::fs::remove_file(&cache_file_path);
                                tracing::info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                            }
                        }

                        // 3. 删除最终缓存文件（不带.download后缀的文件）
                        if let Ok(final_cache_path) =
                            DownloadService::get_online_image_cache_final_path(&cache_path, &url, size)
                        {
                            if let Ok(_metadata) = std::fs::metadata(&final_cache_path) {
                                let _ = std::fs::remove_file(&final_cache_path);
                                tracing::info!(
                                    "[下载任务] [ID:{}] 已删除未完成的最终缓存文件: {}",
                                    id,
                                    final_cache_path
                                );
                            }
                        }
                    }
                } else {
                    // 下载成功
                    // 验证实际文件大小
                    let actual_size = if let Ok(metadata) = std::fs::metadata(&task.task.save_path) {
                        metadata.len()
                    } else {
                        size
                    };

                    task.task.status = DownloadStatus::Completed;
                    task.task.progress = 1.0;
                    task.task.total_size = actual_size;
                    task.task.downloaded_size = actual_size;

                    // 检查是否需要自动设置壁纸
                    let file_name = std::path::Path::new(&task.task.save_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");

                    if let Some(pending_filename) = self.online_state.pending_set_wallpaper_filename.as_ref() {
                        if pending_filename == file_name {
                            // 当前下载的文件是待设置壁纸的文件，自动设置壁纸
                            let full_path = crate::utils::helpers::get_absolute_path(&task.task.save_path);
                            let wallpaper_mode = self.config.wallpaper.mode;
                            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                            // 清除待设置壁纸的文件名
                            self.online_state.pending_set_wallpaper_filename = None;

                            // 异步设置壁纸
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
                }
            }
        }

        // 减少正在下载的任务计数
        self.download_state.decrement_downloading();

        // 检查是否有等待中的任务需要开始
        if let Some(next_task) = self.download_state.get_next_waiting_task() {
            let _next_id = next_task.task.id;
            let next_url = next_task.task.url.clone();
            let next_save_path = PathBuf::from(&next_task.task.save_path);
            let next_proxy = next_task.proxy.clone();
            let next_task_id = next_task.task.id;
            let next_cancel_token = next_task.task.cancel_token.clone().unwrap();
            let next_downloaded_size = next_task.task.downloaded_size;
            let next_total_size = next_task.task.total_size;

            next_task.task.status = DownloadStatus::Downloading;
            next_task.task.start_time = Some(Instant::now());
            self.download_state.increment_downloading();

            let cache_path = self.config.data.cache_path.clone();

            // 启动下一个下载任务
            return Task::perform(
                async_tasks::async_download_wallpaper_task_with_progress(
                    next_url.to_string(),
                    next_save_path,
                    next_proxy,
                    next_task_id,
                    next_cancel_token,
                    next_downloaded_size,
                    next_total_size,
                    cache_path,
                ),
                move |result| match result {
                    Ok(s) => {
                        tracing::info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", next_task_id, s);
                        AppMessage::Download(DownloadMessage::DownloadCompleted(next_task_id, s, None))
                    }
                    Err(e) => {
                        tracing::error!("[下载任务] [ID:{}] 下载失败: {}", next_task_id, e);
                        AppMessage::Download(DownloadMessage::DownloadCompleted(next_task_id, 0, Some(e)))
                    }
                },
            );
        }
        Task::none()
    }
}
