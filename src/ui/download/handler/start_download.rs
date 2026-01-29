// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::wallhaven;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    /// 辅助方法：开始下载壁纸（支持并行限制和进度更新）
    pub fn start_download(&mut self, url: String, id: &str, file_type: &str) -> Task<AppMessage> {
        let file_name = wallhaven::generate_file_name(id, file_type.split('/').last().unwrap_or("jpg"));
        let data_path = self.config.data.data_path.clone();
        let cache_path = self.config.data.cache_path.clone();
        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };
        let file_type = file_type.split('/').last().unwrap_or("jpg").to_string();

        // 生成完整保存路径
        let full_save_path = PathBuf::from(&data_path).join(&file_name);

        // 添加任务（倒序排列）
        self.download_state.add_task(
            url.clone(),
            full_save_path.to_string_lossy().to_string(),
            file_name.clone(),
            proxy.clone(),
            file_type.clone(),
        );

        // 获取任务ID
        let task_id = self.download_state.next_id.saturating_sub(1);

        if self.download_state.can_start_download() {
            // 可以开始下载 - 使用索引查找任务
            let task_index = self.download_state.find_task_index(task_id);
            if let Some(index) = task_index {
                let task_full = self.download_state.get_task_by_index(index);
                if let Some(task_full) = task_full {
                    // 先保存所有需要的数据，再修改状态
                    let url = task_full.task.url.clone();
                    let save_path = PathBuf::from(&task_full.task.save_path);
                    let proxy = task_full.proxy.clone();
                    let task_id = task_full.task.id;
                    let cancel_token = task_full.task.cancel_token.clone().unwrap();
                    let downloaded_size = task_full.task.downloaded_size;
                    let total_size = task_full.task.total_size;
                    let cache_path = cache_path.clone();

                    // 更新状态
                    task_full.task.status = DownloadStatus::Downloading;
                    task_full.task.start_time = Some(Instant::now());
                    self.download_state.increment_downloading();

                    // 启动异步下载任务（带进度更新）
                    return Task::perform(
                        async_task::async_download_wallpaper_task_with_progress(
                            url.to_string(),
                            save_path,
                            proxy,
                            task_id,
                            cancel_token,
                            downloaded_size,
                            total_size,
                            cache_path,
                        ),
                        move |result| match result {
                            Ok(size) => {
                                tracing::info!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                DownloadMessage::DownloadCompleted(task_id, size, None).into()
                            }
                            Err(e) => {
                                tracing::error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                DownloadMessage::DownloadCompleted(task_id, 0, Some(e)).into()
                            }
                        },
                    );
                }
            }
        }

        // 显示通知
        self.show_notification(format!("已添加到下载队列 (等待中)"), NotificationType::Success)
    }
}
