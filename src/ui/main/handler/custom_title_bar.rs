// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    pub(in crate::ui::main) fn title_bar_drag(&mut self) -> Task<AppMessage> {
        window::oldest().and_then(move |id| window::drag(id))
    }

    pub(in crate::ui::main) fn title_bar_minimize(&mut self) -> Task<AppMessage> {
        window::oldest().and_then(|id: window::Id| window::minimize(id, true).map(|_: ()| AppMessage::None))
    }

    pub(in crate::ui::main) fn title_bar_maximize(&mut self) -> Task<AppMessage> {
        let is_maximized = !self.main_state.is_maximized;
        self.main_state.is_maximized = is_maximized;

        // 当窗口从最大化状态还原时,需要重新应用窗口样式以确保拖拽功能正常
        window::oldest()
            .and_then(move |id: window::Id| window::maximize(id, is_maximized).map(|_: ()| AppMessage::None))
            .chain(if !is_maximized {
                // 窗口还原后,重新启用窗口调整大小样式
                self.enable_window_drag_resize()
            } else {
                Task::none()
            })
    }
}
