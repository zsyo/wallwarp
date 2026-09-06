// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn retry_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 重新下载：清空已下载文件，从头开始下载
        // 先保存所有需要的数据，再修改状态
        let Some((url, total_size, current_status)) = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), t.task.total_size, t.task.status.clone()))
        else {
            return Task::none();
        };

        // 对仍在下载中的任务重试：旧一轮下载还占用并发槽位，先取消并释放；
        // 旧循环退出后的完成事件因下方 begin_new_round 递增的代数而被忽略
        if current_status == DownloadStatus::Downloading {
            self.download_state.cancel_task(id);
            self.download_state.decrement_downloading();
        }

        // 释放旧槽位后再判断能否立即开始下载
        if self.download_state.can_start_download() {
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
                            id,
                            cache_file_path
                        );
                    }
                }),
                |_| AppMessage::None,
            );

            return self.spawn_download(id, 0, cleanup_task);
        }

        // 无法立即开始下载，加入排队
        // 同样换发新令牌并递增代数：排队期间旧一轮下载循环的
        // 迟到完成事件会因代数不匹配被忽略，不会把排队任务改写成已取消
        let queue_order = self.download_state.queue_counter;
        self.download_state.queue_counter += 1;
        if let Some(task_full) = self.download_state.get_task(id) {
            let _cancel_token = task_full.task.begin_new_round();
            // 重试语义：进度归零，从头开始
            task_full.task.status = DownloadStatus::Waiting;
            task_full.task.downloaded_size = 0;
            task_full.task.progress = 0.0;
            task_full.task.speed = 0;
            task_full.task.queue_order = queue_order;
            // 保存状态到数据库
            let task_full_clone = task_full.clone();
            let _ = self.download_state.save_to_database(&task_full_clone);
        }
        Task::none()
    }
}
