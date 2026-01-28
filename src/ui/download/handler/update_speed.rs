// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn update_download_speed(&mut self) -> Task<AppMessage> {
        self.download_state.update_speed();
        Task::none()
    }
}
