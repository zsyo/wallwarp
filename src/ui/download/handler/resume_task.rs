// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn resume_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 使用索引查找任务
        // 先保存所有需要的数据，再检查状态
        let Some((url, save_path, total_size, current_status)) = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| {
                (
                    t.task.url.clone(),
                    t.task.save_path.clone(),
                    t.task.total_size,
                    t.task.status.clone(),
                )
            })
        else {
            return Task::none();
        };

        let resumable = current_status == DownloadStatus::Waiting
            || current_status == DownloadStatus::Paused
            || current_status == DownloadStatus::Cancelled
            || matches!(current_status, DownloadStatus::Failed(_));
        if !resumable {
            return Task::none();
        }

        if self.download_state.can_start_download() {
            // 已取消/失败的任务重开时清空旧的目标文件
            // 删除放入阻塞线程池执行，并在清理完成后才开始下载
            let should_reset = current_status == DownloadStatus::Cancelled
                || matches!(current_status, DownloadStatus::Failed(_));
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
                            id,
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
            tracing::debug!(
                "[下载任务] [ID:{}] 恢复：使用偏移量 = {} bytes, total_size = {} bytes",
                id,
                actual_file_size,
                total_size
            );

            return self.spawn_download(id, actual_file_size, cleanup_task);
        }

        // 无法立即开始下载，加入排队
        // 换发新令牌并递增代数：排队期间旧一轮下载循环的
        // 迟到完成事件会因代数不匹配被忽略，不会把排队任务改写成已取消
        let queue_order = self.download_state.queue_counter;
        self.download_state.queue_counter += 1;
        if let Some(task_full) = self.download_state.get_task(id) {
            let _cancel_token = task_full.task.begin_new_round();
            task_full.task.status = DownloadStatus::Waiting;
            task_full.task.queue_order = queue_order;
            // 保存状态到数据库
            let task_full_clone = task_full.clone();
            let _ = self.download_state.save_to_database(&task_full_clone);
        }
        Task::none()
    }
}
