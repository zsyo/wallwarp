use iced::{
    Alignment, Length,
    widget::{column, container, row, text, toggler},
};

use super::common;
use super::style::{
    BORDER_COLOR_GRAY, BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_RED, DIALOG_BORDER_RADIUS,
    DIALOG_BORDER_WIDTH, DIALOG_BUTTON_SPACING, DIALOG_INNER_PADDING, DIALOG_MAX_WIDTH, DIALOG_MESSAGE_SIZE,
    DIALOG_PADDING, DIALOG_SPACING, DIALOG_TITLE_SIZE, MASK_ALPHA, TOGGLE_SPACING, TOGGLE_TEXT_SIZE,
};
use super::{App, AppMessage, CloseConfirmationAction};

pub fn close_confirmation_view(app: &App) -> iced::Element<'_, AppMessage> {
    if !app.show_close_confirmation {
        return iced::widget::Space::new().into();
    }

    let dialog_content = column![
        text(app.i18n.t("close-confirmation.title"))
            .size(DIALOG_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(app.i18n.t("close-confirmation.message"))
            .size(DIALOG_MESSAGE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        row![
            common::create_colored_button(
                app.i18n.t("close-confirmation.minimize-to-tray"),
                BUTTON_COLOR_BLUE,
                AppMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::MinimizeToTray,
                    app.remember_close_setting
                )
            ),
            common::create_colored_button(
                app.i18n.t("close-confirmation.exit"),
                BUTTON_COLOR_RED,
                AppMessage::CloseConfirmationResponse(CloseConfirmationAction::CloseApp, app.remember_close_setting)
            ),
            common::create_colored_button(
                app.i18n.t("close-confirmation.cancel"),
                BUTTON_COLOR_GRAY,
                AppMessage::CloseConfirmationCancelled
            ),
        ]
        .spacing(DIALOG_BUTTON_SPACING)
        .align_y(Alignment::Center),
        row![
            toggler(app.remember_close_setting).on_toggle(AppMessage::ToggleRememberSetting),
            text(app.i18n.t("close-confirmation.remember-setting")).size(TOGGLE_TEXT_SIZE),
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
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(DIALOG_BORDER_RADIUS),
                width: DIALOG_BORDER_WIDTH,
                color: iced::Color::from_rgb(BORDER_COLOR_GRAY, BORDER_COLOR_GRAY, BORDER_COLOR_GRAY),
            },
            ..Default::default()
        });

    let modal_content = container(iced::widget::stack(vec![
        container(iced::widget::Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
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

    iced::widget::opaque(modal_content).into()
}
