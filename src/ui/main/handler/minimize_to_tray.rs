// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::platform;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    pub(in crate::ui::main) fn minimize_to_tray(&mut self) -> Task<AppMessage> {
        self.main_state.is_visible = false;

        // KDE Wayland：写入"跳过任务栏"窗口规则并触发 KWin 重载（对已
        // 映射窗口即时生效），最小化后任务栏不再显示条目；reconfigure
        // 有 200ms 防抖，随后 minimize 照常执行，其余平台为空操作
        platform::on_minimized_to_tray();

        let main_id = self.main_window_id;
        window::minimize(main_id, true).chain(
            // 延迟 500ms 后发送隐藏消息
            Task::perform(
                async { tokio::time::sleep(std::time::Duration::from_millis(500)).await },
                move |_| MainMessage::WindowHiddenReady(main_id).into(),
            ),
        )
    }

    pub(in crate::ui::main) fn window_hidden_ready(&mut self, id: window::Id) -> Task<AppMessage> {
        window::set_mode(id, window::Mode::Hidden)
    }
}
