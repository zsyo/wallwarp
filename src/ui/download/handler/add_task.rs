// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    pub(in crate::ui::download) fn add_download_task(
        &mut self,
        url: String,
        save_path: String,
        file_name: String,
        file_type: String,
    ) -> Task<AppMessage> {
        let proxy = if self.config.global.proxy_enabled && !self.config.global.proxy.is_empty() {
            Some(self.config.global.proxy.clone())
        } else {
            None
        };

        // 合并目录和文件名生成完整路径
        let full_save_path = PathBuf::from(&save_path).join(&file_name);

        // 添加任务（使用完整路径）
        let full_path_str = full_save_path.to_string_lossy().to_string();
        self.download_state.add_task(
            url.clone(),
            full_path_str.clone(),
            file_name.clone(),
            proxy.clone(),
            file_type.clone(),
        );

        // 获取新添加的任务ID
        let task_id = self.download_state.next_id.saturating_sub(1);

        // 更新状态为下载中并启动下载
        match self.download_state.get_task(task_id) {
            Some(task_full) => {
                task_full.task.status = DownloadStatus::Downloading;
                task_full.task.start_time = Some(Instant::now());

                let url = task_full.task.url.clone();
                let save_path = PathBuf::from(&task_full.task.save_path);
                let proxy = task_full.proxy.clone();
                let task_id = task_full.task.id;

                return Task::perform(
                    async_task::async_download_wallpaper_task(url, save_path, proxy, task_id),
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
            None => {}
        }
        Task::none()
    }
}
