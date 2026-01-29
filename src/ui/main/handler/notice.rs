// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn hide_notification(&mut self) -> Task<AppMessage> {
        self.show_notification = false;
        Task::none()
    }
}
