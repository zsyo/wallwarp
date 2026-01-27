// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::App;
use crate::ui::AppMessage;
use crate::ui::download::DownloadStatus;
use iced::Task;

impl App {
    pub(in crate::ui::download) fn cancel_task(&mut self, id: usize) -> Task<AppMessage> {
        // 先保存任务信息，因为取消后可能无法访问
        let task_info = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| {
            (
                t.task.url.clone(),
                t.task.save_path.clone(),
                t.task.file_name.clone(),
                t.task.status.clone(),
            )
        });

        // 取消任务
        self.download_state.cancel_task(id);
        // 将任务状态设置为已取消
        self.download_state.update_status(id, DownloadStatus::Cancelled);

        // 清除未完成的下载文件
        if let Some((url, save_path, _file_name, status)) = task_info {
            // 只有在下载中、等待中或暂停时才清除文件
            if status == DownloadStatus::Downloading
                || status == DownloadStatus::Waiting
                || status == DownloadStatus::Paused
            {
                // 1. 删除目标文件（data_path中的文件）
                if let Ok(_metadata) = std::fs::metadata(&save_path) {
                    // 只有文件未完全下载时才删除（通过检查文件大小是否与完整大小匹配）
                    // 这里我们简单处理：只要状态不是Completed就删除
                    let _ = std::fs::remove_file(&save_path);
                    tracing::info!("[下载任务] [ID:{}] 已删除未完成的目标文件: {}", id, save_path);
                }

                // 2. 删除缓存文件（cache_path/online中的文件）
                // 需要计算缓存文件路径
                let cache_path = self.config.data.cache_path.clone();
                if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                    if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        tracing::info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                    }
                }

                // 3. 删除缓存文件（不带.download后缀的最终文件）
                if let Ok(final_cache_path) = DownloadService::get_online_image_cache_final_path(&cache_path, &url, 0) {
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
        }

        Task::none()
    }
}
