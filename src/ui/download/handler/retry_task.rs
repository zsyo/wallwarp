// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task::{self, DownloadTaskParams};
use crate::services::download::DownloadService;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    pub(in crate::ui::download) fn retry_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 重新下载：清空已下载文件，从头开始下载
        // 先保存所有需要的数据，再修改状态
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
                    t.task.status.clone(),
                )
            });

        if let Some((url, save_path, proxy, task_id, total_size, current_status)) = task_data {
            // 对仍在下载中的任务重试：旧一轮下载还占用并发槽位，先取消并释放；
            // 旧循环退出后的完成事件因下方 begin_new_round 递增的代数而被忽略
            if current_status == DownloadStatus::Downloading {
                self.download_state.cancel_task(id);
                self.download_state.decrement_downloading();
            }

            // 释放旧槽位后再判断能否立即开始下载
            let can_start = self.download_state.can_start_download();
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

                    // 克隆任务以避免借用冲突
                    let task_full_clone = task_full.clone();
                    // 保存状态到数据库
                    let _ = self.download_state.save_to_database(&task_full_clone);
                }

                self.download_state.increment_downloading();

                // 开启新一轮下载：换发新的取消令牌（旧令牌保持取消状态，
                // 旧下载循环退出后的完成事件因代数不匹配被忽略）
                let Some((cancel_token, generation)) = self
                    .download_state
                    .get_task(task_id)
                    .map(|t| t.task.begin_new_round())
                else {
                    tracing::warn!("[下载任务] [ID:{}] 重新下载失败：任务不存在", task_id);
                    return Task::none();
                };

                // 清空缓存文件（cache_path/online中的 .download 文件）
                // 临时缓存路径由 URL+总大小哈希生成,必须用任务的真实 total_size 定位
                // 删除放入阻塞线程池执行，并在删除完成后才开始重新下载
                let cleanup_cache_path = self.config.data.cache_path.clone();
                let cleanup_url = url.clone();
                let cleanup_task = Task::perform(
                    tokio::task::spawn_blocking(move || {
                        if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(
                            &cleanup_cache_path,
                            &cleanup_url,
                            total_size,
                        ) {
                            let _ = std::fs::remove_file(&cache_file_path);
                            tracing::info!(
                                "[下载任务] [ID:{}] 重新下载：已清空缓存文件: {}",
                                task_id,
                                cache_file_path
                            );
                        }
                    }),
                    |_| AppMessage::None,
                );

                let cache_path = self.config.data.cache_path.clone();
                return cleanup_task.chain(Task::perform(
                    async_task::async_download_wallpaper_task_with_progress(DownloadTaskParams {
                        url: url.to_string(),
                        save_path,
                        proxy,
                        task_id,
                        cancel_token,
                        downloaded_size: 0, // 重新下载，从0开始,
                        total_size,         // 保留文件总大小，用于缓存路径计算,
                        cache_path,
                        generation,
                    }),
                    move |result| match result {
                        Ok(size) => {
                            // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                            tracing::debug!(
                                "[下载任务] [ID:{}] 重新下载成功, 文件大小: {} bytes",
                                task_id,
                                size
                            );
                            DownloadMessage::DownloadCompleted(task_id, size, None, generation)
                                .into()
                        }
                        Err(e) => {
                            tracing::error!("[下载任务] [ID:{}] 重新下载失败: {}", task_id, e);
                            DownloadMessage::DownloadCompleted(task_id, 0, Some(e), generation)
                                .into()
                        }
                    },
                ));
            } else {
                // 无法立即开始下载，加入排队
                // 同样换发新令牌并递增代数：排队期间旧一轮下载循环的
                // 迟到完成事件会因代数不匹配被忽略，不会把排队任务改写成已取消
                if let Some(task_full) = self
                    .download_state
                    .tasks
                    .iter_mut()
                    .find(|t| t.task.id == id)
                {
                    let _cancel_token = task_full.task.begin_new_round();
                    // 重试语义：进度归零，从头开始
                    task_full.task.status = DownloadStatus::Waiting;
                    task_full.task.downloaded_size = 0;
                    task_full.task.progress = 0.0;
                    task_full.task.speed = 0;
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
