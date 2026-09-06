// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task::{self, DownloadTaskParams};
use crate::services::download::DownloadService;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;

impl App {
    pub(in crate::ui::download) fn resume_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 使用索引查找任务
        // 先检查是否可以开始下载并保存所有需要的数据
        let can_start = self.download_state.can_start_download();
        let current_status = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| t.task.status.clone());

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

        if let Some((url, save_path, proxy, task_id, total_size)) = task_data
            && (current_status == Some(DownloadStatus::Waiting)
                || current_status == Some(DownloadStatus::Paused)
                || current_status == Some(DownloadStatus::Cancelled)
                || matches!(current_status, Some(DownloadStatus::Failed(_))))
        {
            if can_start {
                // 更新状态为下载中
                let should_reset = current_status == Some(DownloadStatus::Cancelled)
                    || matches!(current_status, Some(DownloadStatus::Failed(_)));
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    task_full.task.status = DownloadStatus::Downloading;
                    task_full.task.start_time = Some(std::time::Instant::now());

                    // 如果任务已取消或失败，重置已下载大小和进度
                    if should_reset {
                        task_full.task.downloaded_size = 0;
                        task_full.task.progress = 0.0;
                        task_full.task.speed = 0;
                    }

                    // 克隆任务以避免借用冲突
                    let task_full_clone = task_full.clone();
                    // 保存状态到数据库
                    let _ = self.download_state.save_to_database(&task_full_clone);
                }

                // 开启新一轮下载：换发新的取消令牌（旧令牌保持取消状态，
                // 旧下载循环退出后的完成事件因代数不匹配被忽略）
                let Some((cancel_token, generation)) = self
                    .download_state
                    .get_task(task_id)
                    .map(|t| t.task.begin_new_round())
                else {
                    tracing::warn!("[下载任务] [ID:{}] 恢复下载失败：任务不存在", task_id);
                    return Task::none();
                };

                // 已取消/失败的任务重开时清空旧的目标文件
                // 删除放入阻塞线程池执行，并在清理完成后才开始下载
                let cleanup_task = if should_reset {
                    let stale_save_path = save_path.clone();
                    Task::perform(
                        tokio::task::spawn_blocking(move || {
                            let _ = std::fs::remove_file(&stale_save_path);
                        }),
                        |_| AppMessage::None,
                    )
                } else {
                    Task::none()
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
                            let safe_size = size.saturating_sub(1024);
                            tracing::debug!(
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
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    task_full.task.downloaded_size = actual_file_size;
                    // 更新进度
                    if total_size > 0 {
                        task_full.task.progress = actual_file_size as f32 / total_size as f32;
                    }
                }
                tracing::debug!(
                    "[下载任务] [ID:{}] 恢复：使用偏移量 = {} bytes, total_size = {} bytes",
                    task_id,
                    actual_file_size,
                    total_size
                );

                self.download_state.increment_downloading();
                return cleanup_task.chain(Task::perform(
                    async_task::async_download_wallpaper_task_with_progress(DownloadTaskParams {
                        url: url.to_string(),
                        save_path,
                        proxy,
                        task_id,
                        cancel_token,
                        downloaded_size: actual_file_size,
                        total_size,
                        cache_path,
                        generation,
                    }),
                    move |result| match result {
                        Ok(size) => {
                            // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                            tracing::debug!(
                                "[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes",
                                task_id,
                                size
                            );
                            DownloadMessage::DownloadCompleted(task_id, size, None, generation)
                                .into()
                        }

                        Err(e) => {
                            tracing::error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                            DownloadMessage::DownloadCompleted(task_id, 0, Some(e), generation)
                                .into()
                        }
                    },
                ));
            } else {
                // 无法立即开始下载，加入排队
                // 换发新令牌并递增代数：排队期间旧一轮下载循环的
                // 迟到完成事件会因代数不匹配被忽略，不会把排队任务改写成已取消
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    let _cancel_token = task_full.task.begin_new_round();
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
