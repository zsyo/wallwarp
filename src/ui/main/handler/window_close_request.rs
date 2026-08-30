// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use crate::utils::config::CloseAction;
use iced::Task;

impl App {
    pub(in crate::ui::main) fn window_close_requested(
        &mut self,
        id: iced::window::Id,
    ) -> Task<AppMessage> {
        // 仅响应主窗口的关闭请求（悬浮球窗口 closeable=false 不会触发）
        if id != self.main_window_id {
            return Task::none();
        }

        // 根据配置处理关闭请求
        match self.config.global.close_action {
            CloseAction::MinimizeToTray => Task::done(MainMessage::MinimizeToTray.into()),
            CloseAction::CloseApp => self.quit_program(),
            CloseAction::Ask => Task::done(MainMessage::ShowCloseConfirmation.into()),
        }
    }

    /// 退出程序：先用与设置开关一致的方式关闭悬浮球窗口（干净消失、
    /// 不改动配置），等待其销毁完成后再真正退出
    ///
    /// 同一周期内"关窗口 + 退出"会打断窗口的正常销毁流程，
    /// 悬浮球窗口会重绘出最后一帧不透明残影后才消失
    pub(crate) fn quit_program(&mut self) -> Task<AppMessage> {
        let close_ball = self.close_floating_ball_window();
        Task::batch(vec![
            close_ball,
            // 等待窗口销毁完成后再退出
            Task::perform(
                tokio::time::sleep(std::time::Duration::from_millis(200)),
                |_| MainMessage::ExitProgram.into(),
            ),
        ])
    }

    /// 真正退出程序（由贴边清理完成后的延迟消息触发）
    pub(crate) fn exit_program(&mut self) -> Task<AppMessage> {
        iced::exit()
    }
}
