// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::main::MainMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_RED, DIALOG_BORDER_RADIUS, DIALOG_BORDER_WIDTH,
    DIALOG_BUTTON_SPACING, DIALOG_INNER_PADDING, DIALOG_MAX_WIDTH, DIALOG_MESSAGE_SIZE, DIALOG_PADDING, DIALOG_SPACING,
    DIALOG_TITLE_SIZE, MASK_ALPHA, TOGGLE_SPACING, TOGGLE_TEXT_SIZE,
};
use crate::ui::{App, AppMessage, CloseConfirmationAction};
use iced::widget::{Space, column, container, opaque, row, stack, text, toggler};
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
                MainMessage::CloseConfirmationResponse(CloseConfirmationAction::CloseApp, app.main_state.remember_close_setting)
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
            toggler(app.main_state.remember_close_setting).on_toggle(|state| MainMessage::ToggleRememberSetting(state).into()),
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

    let modal_dialog = container(dialog_content)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .max_width(DIALOG_MAX_WIDTH)
        .padding(DIALOG_INNER_PADDING)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.dialog_bg)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(DIALOG_BORDER_RADIUS),
                width: DIALOG_BORDER_WIDTH,
                color: theme_colors.border,
            },
            ..Default::default()
        });

    let modal_content = container(stack(vec![
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: MASK_ALPHA,
                })),
                ..Default::default()
            })
            .into(),
        container(modal_dialog)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]))
    .width(Length::Fill)
    .height(Length::Fill);

    opaque(modal_content).into()
}
