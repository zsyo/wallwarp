// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::history::HistoryMessage;
use crate::ui::local::LocalMessage;
use crate::ui::online::OnlineMessage;
use crate::ui::{ActivePage, App, AppMessage};
use iced::keyboard::{Key, key};
use iced::Task;

impl App {
    /// 键盘按键事件分发
    ///
    /// 优先级：热键录制捕获 > 预览模态快捷键（Esc 关闭、左右箭头翻页）。
    /// 仅处理主窗口按键；悬浮球窗口无文本交互，直接忽略。
    pub(in crate::ui) fn key_event(
        &mut self,
        window_id: iced::window::Id,
        key: Key,
        modifiers: iced::keyboard::Modifiers,
    ) -> Task<AppMessage> {
        if window_id != self.main_window_id {
            return Task::none();
        }

        // 热键录制中：按键交给录制捕获逻辑（含 Esc 取消）
        if self.settings_state.hotkey_recording.is_some() {
            return self.hotkey_record_capture(key, modifiers);
        }

        // 预览模态快捷键：仅响应无修饰键的 Esc/左右箭头
        if !modifiers.is_empty() {
            return Task::none();
        }

        let named = match key {
            Key::Named(name) => name,
            _ => return Task::none(),
        };

        match self.active_page {
            ActivePage::LocalList if self.local_state.modal_visible => match named {
                key::Named::Escape => return self.close_local_modal(),
                key::Named::ArrowLeft => {
                    return Task::done(LocalMessage::PreviousImage.into());
                }
                key::Named::ArrowRight => return Task::done(LocalMessage::NextImage.into()),
                _ => {}
            },
            ActivePage::WallpaperHistory if self.history_state.modal_visible => match named {
                key::Named::Escape => {
                    self.history_state.close_modal();
                    return Task::none();
                }
                key::Named::ArrowLeft => {
                    return Task::done(HistoryMessage::PreviousImage.into());
                }
                key::Named::ArrowRight => return Task::done(HistoryMessage::NextImage.into()),
                _ => {}
            },
            ActivePage::OnlineWallpapers if self.online_state.modal_visible => match named {
                key::Named::Escape => return self.close_online_modal(),
                key::Named::ArrowLeft => {
                    return Task::done(OnlineMessage::PreviousImage.into());
                }
                key::Named::ArrowRight => return Task::done(OnlineMessage::NextImage.into()),
                _ => {}
            },
            _ => {}
        }
        Task::none()
    }
}
