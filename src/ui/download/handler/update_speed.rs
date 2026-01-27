// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use iced::Task;

impl App {
    pub(in crate::ui::download) fn update_speed(&mut self) -> Task<AppMessage> {
        self.download_state.update_speed();
        Task::none()
    }
}
