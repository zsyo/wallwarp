// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::AppMessage;
use iced::widget::{button, row, text};
use iced::{Alignment, Element};

/// 创建批量操作按钮
pub fn create_batch_button(
    label: String,
    icon: &'static str,
    enabled: bool,
    message: AppMessage,
    theme_colors: crate::ui::style::ThemeColors,
    button_color: iced::Color,
) -> Element<'static, AppMessage> {
    let button_content = row![
        text(icon)
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(14)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if enabled {
                    iced::Color::WHITE
                } else {
                    theme_colors.light_text_sub
                }),
            }),
        text(label.clone())
            .size(13)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if enabled {
                    iced::Color::WHITE
                } else {
                    theme_colors.light_text_sub
                }),
            }),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let btn = if enabled {
        button(button_content).style(move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                text_color: iced::Color::WHITE,
                background: Some(iced::Background::Color(button_color)),
                border: iced::Border {
                    color: button_color,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
    } else {
        button(button_content).style(move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                text_color: theme_colors.light_text_sub,
                background: Some(iced::Background::Color(theme_colors.light_button)),
                border: iced::Border {
                    color: theme_colors.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
    };

    btn.on_press_maybe(if enabled { Some(message) } else { None })
        .padding([6, 12])
        .into()
}
