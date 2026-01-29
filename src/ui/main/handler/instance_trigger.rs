// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use crate::utils::single_instance::{SingleInstanceGuard, WAKE_UP};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::main) fn external_instance_triggered(&mut self, payload: String) -> Task<AppMessage> {
        info!("外部实例触发事件: {}", payload);

        let show_window_task = if payload.contains(WAKE_UP) {
            self.show_window()
        } else {
            Task::none()
        };
        let next_listen_task = Task::perform(SingleInstanceGuard::listen(), |payload| {
            MainMessage::ExternalInstanceTriggered(payload).into()
        });

        Task::batch(vec![show_window_task, next_listen_task])
    }
}
