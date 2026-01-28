// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Task;

impl App {
    /// 在文件夹中查看壁纸
    pub(in crate::ui::local) fn view_local_file(&mut self, index: usize) -> Task<AppMessage> {
        // 查看文件夹并选中文件
        if let Some(path) = self.local_state.all_paths.get(index) {
            let full_path = helpers::get_absolute_path(path);
            helpers::open_file_in_explorer(&full_path);
        }

        Task::none()
    }
}
