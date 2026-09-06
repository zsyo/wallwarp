// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载任务统一启动路径
//!
//! start/resume/retry/completed 自动续段等入口的公共部分：
//! 查任务、换发取消令牌、改状态、存库、发起带进度的下载任务并映射完成消息

use crate::services::async_task::{self, DownloadTaskParams};
use crate::ui::download::{DownloadMessage, DownloadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;
use std::time::Instant;

impl App {
    /// 启动指定任务的下载（统一公共路径）
    ///
    /// - 换发新的取消令牌并递增代数：旧令牌保持取消状态，旧下载循环退出后的
    ///   完成事件因代数不匹配被忽略，不会覆盖新一轮下载的状态；
    /// - 置为下载中并重置速度，按 `downloaded_size` 恢复进度显示；
    /// - 保存状态到数据库、占用并发槽位；
    /// - 发起带进度上报的下载任务，完成后回传 `DownloadCompleted`。
    ///
    /// # 参数
    /// - `task_id`: 任务 ID
    /// - `downloaded_size`: 断点续传偏移量（新任务/重新下载为 0）
    /// - `pre_start`: 先于下载执行的前置任务（如清理旧文件），无前置时传 `Task::none()`
    pub(in crate::ui::download) fn spawn_download(
        &mut self,
        task_id: usize,
        downloaded_size: u64,
        pre_start: Task<AppMessage>,
    ) -> Task<AppMessage> {
        let Some(task_full) = self.download_state.get_task(task_id) else {
            tracing::warn!("[下载任务] [ID:{}] 启动下载失败：任务不存在", task_id);
            return pre_start;
        };

        let (cancel_token, generation) = task_full.task.begin_new_round();
        task_full.task.status = DownloadStatus::Downloading;
        task_full.task.start_time = Some(Instant::now());
        task_full.task.downloaded_size = downloaded_size;
        task_full.task.speed = 0;
        if task_full.task.total_size > 0 {
            task_full.task.progress = downloaded_size as f32 / task_full.task.total_size as f32;
        }

        let url = task_full.task.url.clone();
        let save_path = PathBuf::from(&task_full.task.save_path);
        let proxy = task_full.proxy.clone();
        let total_size = task_full.task.total_size;
        let task_full_clone = task_full.clone();

        // 保存状态到数据库
        let _ = self.download_state.save_to_database(&task_full_clone);

        let cache_path = self.config.data.cache_path.clone();
        self.download_state.increment_downloading();

        pre_start.chain(Task::perform(
            async_task::async_download_wallpaper_task_with_progress(DownloadTaskParams {
                url,
                save_path,
                proxy,
                task_id,
                cancel_token,
                downloaded_size,
                total_size,
                cache_path,
                generation,
            }),
            move |result| match result {
                Ok(size) => {
                    // 完成事件由 service 层"下载完成"日志记录，此处仅调试细节
                    tracing::debug!(
                        "[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes",
                        task_id,
                        size
                    );
                    DownloadMessage::DownloadCompleted(task_id, size, None, generation).into()
                }
                Err(e) => {
                    tracing::error!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                    DownloadMessage::DownloadCompleted(task_id, 0, Some(e), generation).into()
                }
            },
        ))
    }
}
