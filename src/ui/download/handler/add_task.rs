// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::PathBuf;

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
            return self.spawn_download(task_id, 0, Task::none());
        }
        Task::none()
    }
}
