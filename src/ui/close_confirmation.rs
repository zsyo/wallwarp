use iced::{
    Alignment, Color, Length,
    widget::{button, column, container, row, text, toggler},
};

use super::{App, AppMessage, CloseConfirmationAction};

// 常量定义
const TITLE_SIZE: f32 = 16.0;
const MESSAGE_SIZE: f32 = 14.0;
const BUTTON_TEXT_SIZE: f32 = 14.0;
const TOGGLE_TEXT_SIZE: f32 = 12.0;

const DIALOG_MAX_WIDTH: f32 = 500.0;
const DIALOG_BORDER_RADIUS: f32 = 8.0;
const DIALOG_BORDER_WIDTH: f32 = 1.0;

const DIALOG_PADDING: f32 = 20.0;
const DIALOG_SPACING: f32 = 15.0;
const BUTTON_SPACING: f32 = 10.0;
const TOGGLE_SPACING: f32 = 5.0;
const DIALOG_INNER_PADDING: f32 = 10.0;

const MASK_ALPHA: f32 = 0.5;
const BORDER_COLOR_GRAY: f32 = 0.8;

// 按钮颜色
const BUTTON_COLOR_BLUE: Color = Color::from_rgb8(0, 123, 255);
const BUTTON_COLOR_RED: Color = Color::from_rgb8(220, 53, 69);
const BUTTON_COLOR_GRAY: Color = Color::from_rgb8(108, 117, 125);

pub fn close_confirmation_view(app: &App) -> iced::Element<'_, AppMessage> {
    if !app.show_close_confirmation {
        return iced::widget::Space::new().into();
    }

    let dialog_content = column![
        text(app.i18n.t("close-confirmation.title"))
            .size(TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(app.i18n.t("close-confirmation.message"))
            .size(MESSAGE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        row![
            create_colored_button(
                app.i18n.t("close-confirmation.minimize-to-tray"),
                BUTTON_COLOR_BLUE,
                AppMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::MinimizeToTray,
                    app.remember_close_setting
                )
            ),
            create_colored_button(
                app.i18n.t("close-confirmation.exit"),
                BUTTON_COLOR_RED,
                AppMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::CloseApp,
                    app.remember_close_setting
                )
            ),
            create_colored_button(
                app.i18n.t("close-confirmation.cancel"),
                BUTTON_COLOR_GRAY,
                AppMessage::CloseConfirmationCancelled
            ),
        ]
        .spacing(BUTTON_SPACING)
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

    let modal_content = container(
        iced::widget::stack(vec![
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
        ])
    )
    .width(Length::Fill)
    .height(Length::Fill);

    iced::widget::opaque(modal_content).into()
}

fn create_colored_button<'a>(
    label: String,
    color: Color,
    message: AppMessage,
) -> button::Button<'a, AppMessage> {
    button(text(label).size(BUTTON_TEXT_SIZE))
        .on_press(message)
        .style(move |_theme: &iced::Theme, _status| {
            let base = iced::widget::button::text(_theme, _status);
            iced::widget::button::Style {
                background: Some(iced::Background::Color(color)),
                text_color: iced::Color::WHITE,
                ..base
            }
        })
}
