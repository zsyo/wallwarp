// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 批量操作处理模块
//!
//! 处理下载任务的批量操作逻辑

use crate::ui::AppMessage;
use crate::ui::download::state::DownloadStatus;
use iced::Task;

impl crate::ui::App {
    /// 批量开始选中的任务
    ///
    /// 仅对暂停状态的任务生效
    pub fn batch_start_selected_tasks(&mut self) -> Task<AppMessage> {
        // 收集所有可以开始的任务ID
        let task_ids: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|task| {
                self.download_state.selected_task_ids.contains(&task.task.id)
                    && matches!(task.task.status, DownloadStatus::Paused)
            })
            .map(|task| task.task.id)
            .collect();

        // 按任务ID从大到小排序，设置不同的排队序列值
        let mut sorted_ids = task_ids.clone();
        sorted_ids.sort_by(|a, b| b.cmp(a)); // 从大到小排序

        // 收集所有任务的 Task
        let mut tasks: Vec<Task<AppMessage>> = Vec::new();

        for (index, task_id) in sorted_ids.iter().enumerate() {
            if let Some(task) = self.download_state.tasks.iter_mut().find(|t| t.task.id == *task_id) {
                // 更新排队顺序
                task.task.queue_order = index;
                // 调用恢复任务方法并收集返回的 Task
                let task = self.resume_download_task(*task_id);
                tasks.push(task);
            }
        }

        // 清空选中状态
        self.download_state.selected_task_ids.clear();
        self.download_state.select_all = false;

        // 合并所有 Task 并返回
        Task::batch(tasks)
    }

    /// 批量暂停选中的任务
    ///
    /// 仅对下载中和排队中的任务生效
    pub fn batch_pause_selected_tasks(&mut self) {
        // 收集所有可以暂停的任务ID
        let task_ids: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|task| {
                self.download_state.selected_task_ids.contains(&task.task.id)
                    && matches!(task.task.status, DownloadStatus::Downloading | DownloadStatus::Waiting)
            })
            .map(|task| task.task.id)
            .collect();

        // 暂停每个任务
        for task_id in task_ids {
            let _ = self.pause_download_task(task_id);
        }

        // 清空选中状态
        self.download_state.selected_task_ids.clear();
        self.download_state.select_all = false;
    }

    /// 批量重新开始选中的任务
    ///
    /// 仅对暂停中和下载失败和已取消的任务生效
    pub fn batch_retry_selected_tasks(&mut self) -> Task<AppMessage> {
        // 收集所有可以重新开始的任务ID
        let task_ids: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|task| {
                self.download_state.selected_task_ids.contains(&task.task.id)
                    && matches!(
                        task.task.status,
                        DownloadStatus::Paused | DownloadStatus::Failed(_) | DownloadStatus::Cancelled
                    )
            })
            .map(|task| task.task.id)
            .collect();

        // 按任务ID从大到小排序，设置不同的排队序列值
        let mut sorted_ids = task_ids.clone();
        sorted_ids.sort_by(|a, b| b.cmp(a)); // 从大到小排序

        // 收集所有任务的 Task
        let mut tasks: Vec<Task<AppMessage>> = Vec::new();

        for (index, task_id) in sorted_ids.iter().enumerate() {
            if let Some(task) = self.download_state.tasks.iter_mut().find(|t| t.task.id == *task_id) {
                // 更新排队顺序
                task.task.queue_order = index;
                // 调用重新开始任务方法并收集返回的 Task
                let task = self.retry_download_task(*task_id);
                tasks.push(task);
            }
        }

        // 清空选中状态
        self.download_state.selected_task_ids.clear();
        self.download_state.select_all = false;

        // 合并所有 Task 并返回
        Task::batch(tasks)
    }

    /// 批量取消选中的任务
    ///
    /// 对排队中、下载中、暂停中的任务生效
    pub fn batch_cancel_selected_tasks(&mut self) {
        // 收集所有可以取消的任务ID
        let task_ids: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|task| {
                self.download_state.selected_task_ids.contains(&task.task.id)
                    && matches!(
                        task.task.status,
                        DownloadStatus::Waiting | DownloadStatus::Downloading | DownloadStatus::Paused
                    )
            })
            .map(|task| task.task.id)
            .collect();

        // 取消每个任务
        for task_id in task_ids {
            let _ = self.cancel_download_task(task_id);
        }

        // 清空选中状态
        self.download_state.selected_task_ids.clear();
        self.download_state.select_all = false;
    }

    /// 批量删除选中的任务
    ///
    /// 对所有状态的任务都生效
    pub fn batch_delete_selected_tasks(&mut self) {
        // 收集所有要删除的任务ID
        let task_ids: Vec<usize> = self.download_state.selected_task_ids.iter().cloned().collect();

        // 删除每个任务
        for task_id in task_ids {
            let _ = self.delete_download_task(task_id);
        }

        // 清空选中状态
        self.download_state.selected_task_ids.clear();
        self.download_state.select_all = false;
    }
}
