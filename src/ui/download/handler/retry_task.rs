// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
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
        let task_data = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| {
            (
                t.task.url.clone(),
                PathBuf::from(&t.task.save_path),
                t.proxy.clone(),
                t.task.id,
            )
        });

        if let Some((url, save_path, proxy, task_id)) = task_data {
            if can_start {
                if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
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

                    // 清空已下载的文件（data_path中的文件）
                    let _ = std::fs::remove_file(&task_full.task.save_path);
                    tracing::info!(
                        "[下载任务] [ID:{}] 重新下载：已清空文件: {}",
                        task_id,
                        task_full.task.save_path
                    );

                    // 清空缓存文件（cache_path/online中的文件）
                    let cache_path = self.config.data.cache_path.clone();
                    if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        tracing::info!(
                            "[下载任务] [ID:{}] 重新下载：已清空缓存文件: {}",
                            task_id,
                            cache_file_path
                        );
                    }

                    // 清空最终缓存文件（不带.download后缀的文件）
                    if let Ok(final_cache_path) =
                        DownloadService::get_online_image_cache_final_path(&cache_path, &url, 0)
                    {
                        let _ = std::fs::remove_file(&final_cache_path);
                        tracing::info!(
                            "[下载任务] [ID:{}] 重新下载：已清空最终缓存文件: {}",
                            task_id,
                            final_cache_path
                        );
                    }

                    // 克隆任务以避免借用冲突
                    let task_full_clone = task_full.clone();
                    // 保存状态到数据库
                    let _ = self.download_state.save_to_database(&task_full_clone);
                }

                self.download_state.increment_downloading();

                // 获取取消令牌和文件总大小（已下载大小为0，因为要重新下载）
                let (cancel_token, total_size) =
                    if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == task_id) {
                        (task.task.cancel_token.clone().unwrap(), task.task.total_size)
                    } else {
                        (Arc::new(AtomicBool::new(false)), 0)
                    };

                let cache_path = self.config.data.cache_path.clone();
                return Task::perform(
                    async_task::async_download_wallpaper_task_with_progress(
                        url.to_string(),
                        save_path,
                        proxy,
                        task_id,
                        cancel_token,
                        0,          // 重新下载，从0开始
                        total_size, // 保留文件总大小，用于缓存路径计算
                        cache_path,
                    ),
                    move |result| match result {
                        Ok(size) => {
                            tracing::info!("[下载任务] [ID:{}] 重新下载成功, 文件大小: {} bytes", task_id, size);
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
                if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
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
