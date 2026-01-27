// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::App;
use crate::ui::AppMessage;
use crate::ui::async_tasks;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use iced::Task;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

impl App {
    pub(in crate::ui::download) fn resume_task(&mut self, id: usize) -> Task<AppMessage> {
        // 使用索引查找任务
        // 先检查是否可以开始下载并保存所有需要的数据
        let can_start = self.download_state.can_start_download();
        let current_status = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| t.task.status.clone());

        let task_data = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| {
            (
                t.task.url.clone(),
                PathBuf::from(&t.task.save_path),
                t.proxy.clone(),
                t.task.id,
            )
        });

        if let Some((url, save_path, proxy, task_id)) = task_data {
            if current_status == Some(DownloadStatus::Waiting)
                || current_status == Some(DownloadStatus::Paused)
                || current_status == Some(DownloadStatus::Cancelled)
                || matches!(current_status, Some(DownloadStatus::Failed(_)))
            {
                if can_start {
                    // 更新状态为下载中
                    let should_reset = current_status == Some(DownloadStatus::Cancelled)
                        || matches!(current_status, Some(DownloadStatus::Failed(_)));
                    if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                        task_full.task.status = DownloadStatus::Downloading;
                        task_full.task.start_time = Some(std::time::Instant::now());

                        // 重置取消令牌
                        if let Some(cancel_token) = &task_full.task.cancel_token {
                            let cancel_token: &Arc<AtomicBool> = cancel_token;
                            cancel_token.store(false, Ordering::Relaxed);
                        }

                        // 如果任务已取消或失败，重置已下载大小和进度
                        if should_reset {
                            task_full.task.downloaded_size = 0;
                            task_full.task.progress = 0.0;
                            task_full.task.speed = 0;

                            // 清空已下载的文件
                            let _ = std::fs::remove_file(&task_full.task.save_path);
                        }
                    }

                    // 获取取消令牌、文件总大小
                    let (cancel_token, total_size) =
                        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == task_id) {
                            (task.task.cancel_token.clone().unwrap(), task.task.total_size)
                        } else {
                            (Arc::new(AtomicBool::new(false)), 0)
                        };

                    // 读取实际文件大小作为下载偏移量
                    let cache_path = self.config.data.cache_path.clone();
                    let actual_file_size = if total_size > 0 {
                        if let Ok(cache_file_path) =
                            DownloadService::get_online_image_cache_path(&cache_path, &url, total_size)
                        {
                            if let Ok(metadata) = std::fs::metadata(&cache_file_path) {
                                let size = metadata.len();
                                // 减去 1KB 作为安全边界
                                let safe_size = if size > 1024 { size - 1024 } else { 0 };
                                tracing::info!(
                                    "[下载任务] [ID:{}] 恢复：实际文件大小 = {} bytes, 安全偏移量 = {} bytes",
                                    task_id,
                                    size,
                                    safe_size
                                );
                                safe_size
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    // 更新任务的 downloaded_size
                    if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                        task_full.task.downloaded_size = actual_file_size;
                        // 更新进度
                        if total_size > 0 {
                            task_full.task.progress = actual_file_size as f32 / total_size as f32;
                        }
                    }
                    tracing::info!(
                        "[下载任务] [ID:{}] 恢复：使用偏移量 = {} bytes, total_size = {} bytes",
                        task_id,
                        actual_file_size,
                        total_size
                    );

                    self.download_state.increment_downloading();
                    return Task::perform(
                        async_tasks::async_download_wallpaper_task_with_progress(
                            url.to_string(),
                            save_path,
                            proxy,
                            task_id,
                            cancel_token,
                            actual_file_size,
                            total_size,
                            cache_path,
                        ),
                        move |result| match result {
                            Ok(size) => {
                                tracing::info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                AppMessage::Download(DownloadMessage::DownloadCompleted(task_id, size, None))
                            }

                            Err(e) => {
                                tracing::error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                AppMessage::Download(DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                            }
                        },
                    );
                }
            }
        }
        Task::none()
    }
}
