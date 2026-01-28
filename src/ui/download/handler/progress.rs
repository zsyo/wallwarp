// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn update_download_progress(
        &mut self,
        id: usize,
        downloaded: u64,
        total: u64,
        speed: u64,
    ) -> Task<AppMessage> {
        self.download_state.update_progress(id, downloaded, total, speed);
        Task::none()
    }
}
