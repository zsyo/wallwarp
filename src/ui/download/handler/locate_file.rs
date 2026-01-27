// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use iced::Task;

impl App {
    pub(in crate::ui::download) fn locate_file(&mut self, id: usize) -> Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let full_path = crate::ui::common::get_absolute_path(&task.task.save_path);
            crate::utils::helpers::open_file_in_explorer(&full_path);
        }

        Task::none()
    }
}
