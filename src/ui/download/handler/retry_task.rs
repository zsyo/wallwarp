// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task::{self, DownloadTaskParams};
use crate::services::download::DownloadService;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

impl App {
    pub(in crate::ui::download) fn retry_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 重新下载：清空已下载文件，从头开始下载
        // 先检查是否可以开始下载并保存所有需要的数据
        let can_start = self.download_state.can_start_download();
        let task_data = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| {
                (
                    t.task.url.clone(),
                    PathBuf::from(&t.task.save_path),
                    t.proxy.clone(),
                    t.task.id,
                    t.task.total_size,
                )
            });

        if let Some((url, save_path, proxy, task_id, total_size)) = task_data {
            if can_start {
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    // 重置任务状态和进度
                    task_full.task.status = DownloadStatus::Downloading;
                    task_full.task.start_time = Some(Instant::now());
                    task_full.task.downloaded_size = 0;
                    task_full.task.progress = 0.0;
                    task_full.task.speed = 0;

                    // 重置取消令牌
                    if let Some(cancel_token) = &task_full.task.cancel_token {
                        let cancel_token: &Arc<AtomicBool> = cancel_token;
                        cancel_token.store(false, Ordering::Relaxed);
                    }

                    // 清空缓存文件（cache_path/online中的文件）
                    // 临时缓存路径由 URL+总大小哈希生成,必须用任务的真实 total_size 定位
                    let cache_path = self.config.data.cache_path.clone();
                    if let Ok(cache_file_path) =
                        DownloadService::get_online_image_cache_path(&cache_path, &url, total_size)
                    {
                        let _ = std::fs::remove_file(&cache_file_path);
                        tracing::info!(
                            "[下载任务] [ID:{}] 重新下载：已清空缓存文件: {}",
                            task_id,
                            cache_file_path
                        );
                    }

                    // 克隆任务以避免借用冲突
                    let task_full_clone = task_full.clone();
                    // 保存状态到数据库
                    let _ = self.download_state.save_to_database(&task_full_clone);
                }

                self.download_state.increment_downloading();

                // 获取取消令牌（已下载大小为0，因为要重新下载）
                let cancel_token = if let Some(task) = self
                    .download_state
                    .tasks
                    .iter()
                    .find(|t| t.task.id == task_id)
                {
                    task.task.cancel_token.clone().unwrap()
                } else {
                    Arc::new(AtomicBool::new(false))
                };

                let cache_path = self.config.data.cache_path.clone();
                return Task::perform(
                    async_task::async_download_wallpaper_task_with_progress(DownloadTaskParams {
                        url: url.to_string(),
                        save_path,
                        proxy,
                        task_id,
                        cancel_token,
                        downloaded_size: 0, // 重新下载，从0开始,
                        total_size,         // 保留文件总大小，用于缓存路径计算,
                        cache_path,
                    }),
                    move |result| match result {
                        Ok(size) => {
                            // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                            tracing::debug!(
                                "[下载任务] [ID:{}] 重新下载成功, 文件大小: {} bytes",
                                task_id,
                                size
                            );
                            DownloadMessage::DownloadCompleted(task_id, size, None).into()
                        }
                        Err(e) => {
                            tracing::error!("[下载任务] [ID:{}] 重新下载失败: {}", task_id, e);
                            DownloadMessage::DownloadCompleted(task_id, 0, Some(e)).into()
                        }
                    },
                );
            } else {
                // 无法立即开始下载，加入排队
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    task_full.task.status = DownloadStatus::Waiting;
                    task_full.task.queue_order = self.download_state.queue_counter;
                    self.download_state.queue_counter += 1;
                    // 保存状态到数据库
                    let task_full_clone = task_full.clone();
                    let _ = self.download_state.save_to_database(&task_full_clone);
                }
            }
        }
        Task::none()
    }
}
