// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::platform;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    /// 边缘感应层触发的窗口缩放（iced 0.14 原生 API，Windows/X11/Wayland 支持）
    pub(in crate::ui::main) fn drag_resize_window(
        &mut self,
        direction: window::Direction,
    ) -> Task<AppMessage> {
        window::drag_resize(self.main_window_id, direction)
    }

    /// 启用无边框窗口的系统级边缘缩放
    ///
    /// 仅 Windows 需要（注入 WS_THICKFRAME 让系统接管边缘缩放与阴影）；
    /// macOS 使用原生 fullsize content view，Linux 使用自绘边缘感应层，
    /// 两者均为空操作
    pub fn enable_window_drag_resize(&self) -> Task<AppMessage> {
        let main_id = self.main_window_id;
        window::run(main_id, |mw| platform::enable_resize_border(mw)).map(|_| AppMessage::None)
    }
}
