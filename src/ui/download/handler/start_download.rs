// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::path::PathBuf;

impl App {
    /// 辅助方法：开始下载壁纸（支持并行限制和进度更新）
    pub fn start_download(&mut self, url: String, id: &str, file_type: &str) -> Task<AppMessage> {
        let file_name =
            wallhaven::generate_file_name(id, file_type.split('/').next_back().unwrap_or("jpg"));
        let data_path = self.config.data.data_path.clone();
        let proxy = self.config.resolved_proxy();
        let file_type = file_type
            .split('/')
            .next_back()
            .unwrap_or("jpg")
            .to_string();

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
            return self.spawn_download(task_id, 0, Task::none());
        }

        // 显示通知
        self.show_notification(
            self.i18n.t("notification.added-to-download-queue"),
            NotificationType::Success,
        )
    }
}
