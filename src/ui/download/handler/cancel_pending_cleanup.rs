// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::App;
use crate::ui::download::DownloadStatus;
use tracing::info;

impl App {
    /// 取消所有等待中/已暂停的下载任务，并清理其半成品文件
    ///
    /// 用于新搜索/刷新在线列表前，丢弃不再需要的排队任务及其部分下载文件
    pub(in crate::ui) fn cancel_pending_tasks_and_cleanup(&mut self) {
        let waiting_tasks: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|t| {
                matches!(
                    t.task.status,
                    DownloadStatus::Waiting | DownloadStatus::Paused
                )
            })
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
                        t.task.total_size,
                    )
                });

            // 取消任务
            self.download_state.cancel_task(task_id);
            // 将任务状态设置为已取消
            self.download_state
                .update_status(task_id, DownloadStatus::Cancelled);

            // 清除未完成的下载文件
            if let Some((url, save_path, total_size)) = task_info {
                // 1. 删除目标文件（data_path中的文件）
                if let Ok(_metadata) = std::fs::metadata(&save_path) {
                    let _ = std::fs::remove_file(&save_path);
                    info!(
                        "[下载任务] [ID:{}] 已删除未完成的目标文件: {}",
                        task_id, save_path
                    );
                }

                // 2. 删除缓存文件（.download 临时文件与最终文件）
                // 临时缓存路径由 URL+总大小哈希生成，必须用任务的真实 total_size 定位
                let cache_path = self.config.data.cache_path.clone();
                for path_result in [
                    DownloadService::get_online_image_cache_path(&cache_path, &url, total_size),
                    DownloadService::get_online_image_cache_final_path(
                        &cache_path,
                        &url,
                        total_size,
                    ),
                ] {
                    if let Ok(cache_file_path) = path_result
                        && let Ok(_metadata) = std::fs::metadata(&cache_file_path)
                    {
                        let _ = std::fs::remove_file(&cache_file_path);
                        info!(
                            "[下载任务] [ID:{}] 已删除未完成的缓存文件: {}",
                            task_id, cache_file_path
                        );
                    }
                }
            }
        }
    }
}
