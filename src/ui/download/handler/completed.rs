// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task::{self, DownloadTaskParams};
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    pub(in crate::ui::download) fn download_completed(
        &mut self,
        id: usize,
        size: u64,
        error: Option<String>,
    ) -> Task<AppMessage> {
        let task_index = self.download_state.find_task_index(id);
        if let Some(index) = task_index {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                // 检查当前状态
                let current_status = task.task.status.clone();

                if current_status == DownloadStatus::Paused {
                    // 任务已暂停，保持暂停状态
                } else if error.is_some() {
                    // 下载失败
                    let error_msg = error.unwrap();
                    // 检查是否是用户取消
                    if error_msg == crate::services::download::DOWNLOAD_CANCELLED {
                        // 检查任务是否在暂停状态被取消
                        // 如果任务原本是暂停状态，则保持暂停，否则设置为已取消
                        // 如果不是暂停状态，设置为已取消
                        if current_status != DownloadStatus::Paused {
                            task.task.status = DownloadStatus::Cancelled;
                        }
                    } else {
                        // 失败状态的临时文件清理在下载任务错误路径中完成
                        // (那里持有真实临时路径,此处无法可靠定位)
                        task.task.status = DownloadStatus::Failed(error_msg.clone());
                    }
                } else {
                    // 下载成功
                    // 验证实际文件大小
                    let actual_size = if let Ok(metadata) = std::fs::metadata(&task.task.save_path)
                    {
                        metadata.len()
                    } else {
                        size
                    };

                    task.task.status = DownloadStatus::Completed;
                    task.task.progress = 1.0;
                    task.task.total_size = actual_size;
                    task.task.downloaded_size = actual_size;
                    // 壁纸库新增了文件，本地页列表缓存失效
                    self.local_state.loaded_data_path = None;

                    // 检查是否需要自动设置壁纸
                    let file_name = std::path::Path::new(&task.task.save_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");

                    if let Some(pending_filename) =
                        self.online_state.pending_set_wallpaper_filename.as_ref()
                        && pending_filename == file_name
                    {
                        // 当前下载的文件是待设置壁纸的文件，自动设置壁纸
                        let full_path =
                            crate::utils::helpers::get_absolute_path(&task.task.save_path);

                        // 清除待设置壁纸的文件名
                        self.online_state.pending_set_wallpaper_filename = None;

                        return self.apply_wallpaper(full_path);
                    }
                }

                // 保存状态到数据库（在状态修改完成后）
                if let Some(task_full) = self.download_state.tasks.get(index) {
                    let _ = self.download_state.save_to_database(task_full);
                }
            }
        }

        // 减少正在下载的任务计数
        self.download_state.decrement_downloading();

        // 检查是否有等待中的任务需要开始
        if let Some(next_task) = self.download_state.get_next_waiting_task() {
            let next_url = next_task.task.url.clone();
            let next_save_path = PathBuf::from(&next_task.task.save_path);
            let next_proxy = next_task.proxy.clone();
            let next_task_id = next_task.task.id;
            let next_cancel_token = next_task.task.cancel_token.clone().unwrap();
            let next_downloaded_size = next_task.task.downloaded_size;
            let next_total_size = next_task.task.total_size;
            next_task.task.status = DownloadStatus::Downloading;
            next_task.task.start_time = Some(Instant::now());
            self.download_state.increment_downloading();

            // 保存状态到数据库
            if let Some(task_full) = self
                .download_state
                .tasks
                .iter()
                .find(|t| t.task.id == next_task_id)
            {
                let _ = self.download_state.save_to_database(task_full);
            }
            let cache_path = self.config.data.cache_path.clone();

            // 启动下一个下载任务
            return Task::perform(
                async_task::async_download_wallpaper_task_with_progress(DownloadTaskParams {
                    url: next_url.to_string(),
                    save_path: next_save_path,
                    proxy: next_proxy,
                    task_id: next_task_id,
                    cancel_token: next_cancel_token,
                    downloaded_size: next_downloaded_size,
                    total_size: next_total_size,
                    cache_path,
                }),
                move |result| match result {
                    Ok(s) => {
                        // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                        tracing::debug!(
                            "[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes",
                            next_task_id,
                            s
                        );
                        DownloadMessage::DownloadCompleted(next_task_id, s, None).into()
                    }
                    Err(e) => {
                        tracing::error!("[下载任务] [ID:{}] 下载失败: {}", next_task_id, e);
                        DownloadMessage::DownloadCompleted(next_task_id, 0, Some(e)).into()
                    }
                },
            );
        }
        Task::none()
    }
}
