// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn delete_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 先保存任务信息，因为删除后可能无法访问
        let task_info = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), t.task.save_path.clone(), t.task.status.clone()));

        // 检查任务状态
        let current_status = task_info.as_ref().map(|(_, _, status)| status.clone());

        // 如果任务处于下载中、等待中或暂停状态，需要取消网络请求
        if let Some(status) = current_status {
            if status == DownloadStatus::Downloading
                || status == DownloadStatus::Waiting
                || status == DownloadStatus::Paused
            {
                // 取消任务（设置取消标志，终止网络请求）
                self.download_state.cancel_task(id);

                // 仅对下载中状态的任务减少下载计数
                if status == DownloadStatus::Downloading {
                    self.download_state.decrement_downloading();
                }
            }
        }

        // 清除未完成的下载文件（对于下载中、等待中或暂停的任务）
        if let Some((url, _save_path, status)) = task_info {
            if status == DownloadStatus::Downloading
                || status == DownloadStatus::Waiting
                || status == DownloadStatus::Paused
            {
                let cache_path = self.config.data.cache_path.clone();

                // 删除缓存文件（cache_path/online中的 .download 文件）
                if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                    if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        tracing::info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                    }
                }
            }
        }

        // 最后删除任务记录
        self.download_state.remove_task(id);

        Task::none()
    }

    pub(in crate::ui::download) fn clear_download_completed_tasks(&mut self) -> Task<AppMessage> {
        self.download_state.clear_completed();
        Task::none()
    }
}
