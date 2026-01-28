// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::online) fn online_search(&mut self) -> Task<AppMessage> {
        // 搜索：重置到第一页并重新加载
        self.online_state.current_page = 1;

        // 取消所有等待中的下载任务
        let waiting_tasks: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|t| matches!(t.task.status, DownloadStatus::Waiting | DownloadStatus::Paused))
            .map(|t| t.task.id)
            .collect();

        for task_id in waiting_tasks {
            // 先保存任务信息，因为取消后可能无法访问
            let task_info = self
                .download_state
                .tasks
                .iter()
                .find(|t| t.task.id == task_id)
                .map(|t| {
                    (
                        t.task.url.clone(),
                        t.task.save_path.clone(),
                        t.task.file_name.clone(),
                        t.task.status.clone(),
                    )
                });

            // 取消任务
            self.download_state.cancel_task(task_id);
            // 将任务状态设置为已取消
            self.download_state.update_status(task_id, DownloadStatus::Cancelled);

            // 清除未完成的下载文件
            if let Some((url, save_path, _file_name, status)) = task_info {
                // 只有在下载中、等待中或暂停时才清除文件
                if status == DownloadStatus::Downloading
                    || status == DownloadStatus::Waiting
                    || status == DownloadStatus::Paused
                {
                    // 1. 删除目标文件（data_path中的文件）
                    if let Ok(_metadata) = std::fs::metadata(&save_path) {
                        let _ = std::fs::remove_file(&save_path);
                        info!("[下载任务] [ID:{}] 已删除未完成的目标文件: {}", task_id, save_path);
                    }

                    // 2. 删除缓存文件
                    let cache_path = self.config.data.cache_path.clone();
                    if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                        if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                            let _ = std::fs::remove_file(&cache_file_path);
                            info!(
                                "[下载任务] [ID:{}] 已删除未完成的缓存文件: {}",
                                task_id, cache_file_path
                            );
                        }
                    }

                    // 3. 删除缓存文件（不带.download后缀的最终文件）
                    if let Ok(final_cache_path) =
                        DownloadService::get_online_image_cache_final_path(&cache_path, &url, 0)
                    {
                        if let Ok(_metadata) = std::fs::metadata(&final_cache_path) {
                            let _ = std::fs::remove_file(&final_cache_path);
                            info!(
                                "[下载任务] [ID:{}] 已删除未完成的最终缓存文件: {}",
                                task_id, final_cache_path
                            );
                        }
                    }
                }
            }
        }

        // 滚动到顶部，避免触发自动加载下一页
        let scroll_to_top_task = Task::perform(async {}, |_| AppMessage::ScrollToTop("online_wallpapers".to_string()));

        // 执行搜索和滚动到顶部
        Task::batch([self.load_online_wallpapers(), scroll_to_top_task])
    }
}
