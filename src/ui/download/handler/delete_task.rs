// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use iced::Task;

impl App {
    pub(in crate::ui::download) fn delete_task(&mut self, id: usize) -> Task<AppMessage> {
        // 仅删除任务记录，不删除文件
        // 因为文件已经下载完成，用户可能需要保留文件
        self.download_state.remove_task(id);
        Task::none()
    }

    pub(in crate::ui::download) fn clear_completed_tasks(&mut self) -> Task<AppMessage> {
        self.download_state.clear_completed();
        Task::none()
    }
}
