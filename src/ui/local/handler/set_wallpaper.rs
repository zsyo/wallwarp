// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Task;

impl App {
    /// 设置壁纸
    pub(in crate::ui::local) fn local_set_as_wallpaper(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        // 设置壁纸
        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
            let full_path = helpers::get_absolute_path(&path);
            return self.apply_wallpaper(full_path);
        }
        Task::none()
    }
}
