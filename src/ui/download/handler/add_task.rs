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
        let proxy = self.config.resolved_proxy();

        // 合并目录和文件名生成完整路径
        let full_save_path = PathBuf::from(&save_path).join(&file_name);
        let full_path_str = full_save_path.to_string_lossy().to_string();

        // 添加任务（使用完整路径），获取新任务 id
        let task_id = self.download_state.add_task(
            url.clone(),
            full_path_str.clone(),
            file_name.clone(),
            proxy.clone(),
            file_type.clone(),
        );

        // 未满并发上限时立即开始下载；已满则任务保持排队(Waiting)，
        // 由 download_completed 在有空闲槽位时按排队顺序自动启动
        if self.download_state.can_start_download() {
            if let Some(task_full) = self.download_state.get_task(task_id) {
                task_full.task.status = DownloadStatus::Downloading;
                task_full.task.start_time = Some(Instant::now());
            }
            self.download_state.increment_downloading();

            if let Some(task_full) = self
                .download_state
                .tasks
                .iter()
                .find(|t| t.task.id == task_id)
            {
                // 保存状态到数据库
                let _ = self.download_state.save_to_database(task_full);

                let url = task_full.task.url.clone();
                let save_path = PathBuf::from(&task_full.task.save_path);
                let proxy = task_full.proxy.clone();

                return Task::perform(
                    async_task::async_download_wallpaper_task(url, save_path, proxy, task_id),
                    move |result| match result {
                        Ok(size) => {
                            // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                            tracing::debug!(
                                "[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes",
                                task_id,
                                size
                            );
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
        Task::none()
    }
}
