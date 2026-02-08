// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 选中/全选处理模块
//!
//! 处理下载任务的多选逻辑

impl crate::ui::App {
    /// 切换任务选中状态
    ///
    /// 当用户点击某个任务的选中框时：
    /// - 如果任务已选中，则取消选中
    /// - 如果任务未选中，则添加到选中集合
    /// - 如果表头处于全选状态，取消表头的全选状态（但不影响其他任务的选中状态）
    pub fn toggle_task_selection(&mut self, task_id: usize) {
        // 切换任务的选中状态
        if self.download_state.selected_task_ids.contains(&task_id) {
            self.download_state.selected_task_ids.remove(&task_id);
        } else {
            self.download_state.selected_task_ids.insert(task_id);
        }

        // 如果表头处于全选状态，取消表头的全选状态
        // 但不取消其他任务的选中状态（符合需求3）
        if self.download_state.select_all {
            self.download_state.select_all = false;
        }
    }

    /// 切换全选状态
    ///
    /// 当用户点击表头的选中框时：
    /// - 如果当前是全选状态，则清空所有选中（取消全选）
    /// - 如果当前不是全选状态，则选中所有当前筛选显示的任务
    ///
    /// 注意：这里只选中当前筛选显示的任务，而不是所有任务
    pub fn toggle_select_all(&mut self) {
        // 获取当前筛选后的任务列表
        let filtered_task_ids: Vec<usize> = self
            .download_state
            .tasks
            .iter()
            .filter(|task| {
                if let Some(filter_status) = &self.download_state.status_filter {
                    filter_status.matches(&task.task.status)
                } else {
                    true
                }
            })
            .map(|task| task.task.id)
            .collect();

        if self.download_state.select_all {
            // 取消全选：清空所有选中
            self.download_state.select_all = false;
            self.download_state.selected_task_ids.clear();
        } else {
            // 全选：选中所有当前筛选显示的任务
            self.download_state.select_all = true;
            self.download_state.selected_task_ids.clear();
            for task_id in filtered_task_ids {
                self.download_state.selected_task_ids.insert(task_id);
            }
        }
    }
}