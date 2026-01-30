// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::settings) fn settings_language_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换语言选择器的展开/收起状态
        self.language_picker_expanded = !self.language_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_language_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭语言选择器
        self.language_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_protocol_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换代理协议选择器的展开/收起状态
        self.proxy_protocol_picker_expanded = !self.proxy_protocol_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_protocol_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭代理协议选择器
        self.proxy_protocol_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_theme_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换主题选择器的展开/收起状态
        self.theme_picker_expanded = !self.theme_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_theme_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭主题选择器
        self.theme_picker_expanded = false;
        Task::none()
    }
}
