// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    pub(in crate::ui::main) fn title_bar_drag(&mut self) -> Task<AppMessage> {
        window::drag::<AppMessage>(self.main_window_id)
    }

    pub(in crate::ui::main) fn title_bar_minimize(&mut self) -> Task<AppMessage> {
        window::minimize(self.main_window_id, true).map(|_: ()| AppMessage::None)
    }

    pub(in crate::ui::main) fn title_bar_maximize(&mut self) -> Task<AppMessage> {
        let is_maximized = !self.main_state.is_maximized;
        self.main_state.is_maximized = is_maximized;

        window::maximize(self.main_window_id, is_maximized).map(|_: ()| AppMessage::None)
    }
}
