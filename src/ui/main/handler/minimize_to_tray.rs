// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    pub(in crate::ui::main) fn minimize_to_tray(&mut self) -> Task<AppMessage> {
        self.main_state.is_visible = false;

        window::oldest().and_then(|id| {
            window::minimize(id, true).chain(
                // 延迟 500ms 后发送隐藏消息
                Task::perform(
                    async { tokio::time::sleep(std::time::Duration::from_millis(500)).await },
                    move |_| MainMessage::WindowHiddenReady(id).into(),
                ),
            )
        })
    }

    pub(in crate::ui::main) fn window_hidden_ready(&mut self, id: window::Id) -> Task<AppMessage> {
        window::set_mode(id, window::Mode::Hidden)
    }
}
