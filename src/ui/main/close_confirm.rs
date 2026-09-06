// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::main::MainMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_RED, DIALOG_BUTTON_SPACING,
    DIALOG_MESSAGE_SIZE, DIALOG_PADDING, DIALOG_SPACING, DIALOG_TITLE_SIZE, TOGGLE_SPACING,
    TOGGLE_TEXT_SIZE,
};
use crate::ui::{App, AppMessage, CloseConfirmationAction};
use iced::widget::{Space, column, row, text, toggler};
use iced::{Alignment, Length};

pub fn close_confirm_view(app: &App) -> iced::Element<'_, AppMessage> {
    if !app.main_state.show_close_confirmation {
        return Space::new().into();
    }

    let theme_colors = app.theme_colors;

    let dialog_content = column![
        text(app.i18n.t("close-confirmation.title"))
            .size(DIALOG_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        text(app.i18n.t("close-confirmation.message"))
            .size(DIALOG_MESSAGE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        row![
            common::create_colored_button(
                app.i18n.t("close-confirmation.minimize-to-tray"),
                BUTTON_COLOR_BLUE,
                MainMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::MinimizeToTray,
                    app.main_state.remember_close_setting
                )
                .into()
            ),
            common::create_colored_button(
                app.i18n.t("close-confirmation.exit"),
                BUTTON_COLOR_RED,
                MainMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::CloseApp,
                    app.main_state.remember_close_setting
                )
                .into()
            ),
            common::create_colored_button(
                app.i18n.t("close-confirmation.cancel"),
                BUTTON_COLOR_GRAY,
                MainMessage::CloseConfirmationCancelled.into()
            ),
        ]
        .spacing(DIALOG_BUTTON_SPACING)
        .align_y(Alignment::Center),
        row![
            toggler(app.main_state.remember_close_setting)
                .on_toggle(|state| MainMessage::ToggleRememberSetting(state).into()),
            text(app.i18n.t("close-confirmation.remember-setting"))
                .size(TOGGLE_TEXT_SIZE)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                }),
        ]
        .align_y(Alignment::Center)
        .spacing(TOGGLE_SPACING)
    ]
    .padding(DIALOG_PADDING)
    .spacing(DIALOG_SPACING)
    .align_x(Alignment::Center);

    // 复用公共模态对话框外壳（遮罩 + 居中对话框容器）
    common::modal_dialog_shell(theme_colors, dialog_content.into())
}
