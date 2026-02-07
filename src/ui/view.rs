// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::common;
use super::{App, AppMessage, NotificationType};
use crate::ui::main::{MainMessage, close_confirm_view, main_view};
use crate::ui::settings::SettingsMessage;
use crate::ui::style;
use iced::widget::{container, stack, text};
use iced::{Element, Length, Task};

impl App {
    pub fn view(&self) -> Element<'_, AppMessage> {
        // 先渲染底层内容
        let base_content = main_view(self);

        // 如果显示任何确认对话框，则将对话框叠加在底层内容上
        let main_content = if self.main_state.show_close_confirmation {
            Self::create_stack(base_content, close_confirm_view(self))
        } else if self.settings_state.show_path_clear_confirmation {
            Self::create_stack(base_content, self.path_clear_confirmation_view())
        } else {
            base_content
        };

        // 如果显示通知，则将通知叠加在主要内容之上
        if self.main_state.show_notification {
            Self::create_stack(main_content, self.notification_view())
        } else {
            main_content
        }
    }

    // 渲染路径清空确认对话框
    fn path_clear_confirmation_view(&self) -> iced::Element<'_, AppMessage> {
        let path_display = self.get_path_display(&self.settings_state.path_to_clear);

        // 将消息转换为字符串（简化处理）
        let message_text = format!("{}\n{}", self.i18n.t("path-clear-confirmation.message"), path_display);

        common::create_confirmation_dialog(
            self.i18n.t("path-clear-confirmation.title"),
            message_text,
            self.i18n.t("path-clear-confirmation.confirm"),
            self.i18n.t("path-clear-confirmation.cancel"),
            SettingsMessage::ConfirmPathClear(self.settings_state.path_to_clear.clone()).into(),
            SettingsMessage::CancelPathClear.into(),
        )
    }

    // 渲染通知组件
    fn notification_view(&self) -> iced::Element<'_, AppMessage> {
        // 根据通知类型设置颜色
        let (bg_color, text_color) = match self.main_state.notification_type {
            NotificationType::Success => (style::NOTIFICATION_SUCCESS_BG, style::NOTIFICATION_TEXT_COLOR),
            NotificationType::Error => (style::NOTIFICATION_ERROR_BG, style::NOTIFICATION_TEXT_COLOR),
            NotificationType::Info => (style::NOTIFICATION_INFO_BG, style::NOTIFICATION_TEXT_COLOR),
        };

        let notification_content = container(text(&self.main_state.notification_message).size(14).style(
            move |_theme| text::Style {
                color: Some(text_color),
            },
        ))
        .padding(10)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(move |_theme| container::Style {
            background: Some(iced::Background::Color(bg_color)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(8.0),
                width: 1.0,
                color: iced::Color::TRANSPARENT,
            },
            ..Default::default()
        });

        // 将通知放在窗口底部中央
        container(
            container(notification_content)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .padding(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Bottom)
        .into()
    }

    // 辅助方法：创建叠加层（底层内容 + 覆盖内容）
    fn create_stack<'a>(base: Element<'a, AppMessage>, overlay: Element<'a, AppMessage>) -> Element<'a, AppMessage> {
        stack(vec![base, overlay])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // 辅助方法：获取路径显示字符串
    fn get_path_display(&self, path_type: &str) -> &str {
        match path_type {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => "",
        }
    }

    // 辅助方法：显示通知
    pub fn show_notification(&mut self, message: String, notification_type: NotificationType) -> Task<AppMessage> {
        self.main_state.notification_message = message;
        self.main_state.notification_type = notification_type;
        self.main_state.show_notification = true;
        // 递增通知版本号，确保只有最新版本的通知能被隐藏
        self.main_state.notification_version += 1;
        let current_version = self.main_state.notification_version;

        Task::perform(
            async move {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                current_version
            },
            |version| MainMessage::HideNotificationWithVersion(version).into(),
        )
    }
}
