// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::platform;
use crate::ui::main::main_window_settings;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    /// 显示主窗口并置前（三平台通用 iced API）
    ///
    /// 悬浮球/托盘唤醒入口：窗口隐藏时重新可见（保留最大化状态）、
    /// 最小化时恢复、最后请求键盘焦点
    pub(in crate::ui::main) fn show_window(&mut self) -> Task<AppMessage> {
        tracing::info!("[显示窗口] 恢复并置前主窗口");

        // Wayland 协议限制：winit 的 set_visible 为空操作、客户端无法解除
        // 最小化（compositor 直接忽略），隐藏/最小化后的窗口无法原地恢复，
        // 改为"新建主窗口替换旧窗口"实现恢复（悬浮球在 Wayland 不存在，
        // 主窗口是唯一窗口，先开后关不会触发 daemon 因无窗口而退出）
        if platform::is_wayland() {
            let mut settings = main_window_settings(&self.config, true);
            // 保留用户上次调整的窗口尺寸（位置在 Wayland 下无法查询，居中打开）
            if let Some((width, height)) = self.main_state.pending_window_size {
                settings.size = iced::Size::new(width as f32, height as f32);
            }
            let (new_id, open_task) = window::open(settings);
            let old_id = std::mem::replace(&mut self.main_window_id, new_id);
            self.main_state.is_maximized = false;
            self.main_state.is_visible = true;
            tracing::info!("[显示窗口] [Wayland] 重建主窗口恢复可见: {old_id:?} -> {new_id:?}");
            return Task::batch(vec![
                open_task.map(|_| AppMessage::None),
                window::close::<AppMessage>(old_id),
            ]);
        }

        let main_id = self.main_window_id;
        Task::batch(vec![
            // Hidden → 可见（set_mode 不改变最大化状态）
            window::set_mode(main_id, window::Mode::Windowed),
            // 最小化 → 恢复（保留最大化状态）
            window::minimize(main_id, false),
            // 置前并获得焦点
            window::gain_focus(main_id),
        ])
    }
}
