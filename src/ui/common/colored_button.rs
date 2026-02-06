// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::BUTTON_TEXT_SIZE;
use iced::widget::{button, text};
use iced::{Alignment, Color, Element};

/// 创建带颜色的按钮（接收文本字符串）
pub fn create_colored_button<'a, Message>(label: String, color: Color, message: Message) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(label)
            .size(BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(message)
    .style(move |theme, status| button::Style {
        background: Some(iced::Background::Color(color)),
        text_color: iced::Color::WHITE,
        border: iced::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
        ..iced::widget::button::primary(theme, status)
    })
}

/// 创建带颜色的按钮（接收 text 控件，可自定义字体和颜色）
pub fn create_colored_button_with_text<'a, Message>(
    text_element: Element<'a, Message>,
    color: Color,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text_element)
        .on_press(message)
        .style(move |_theme: &iced::Theme, _status| {
            let base = button::text(_theme, _status);
            button::Style {
                background: Some(iced::Background::Color(color)),
                text_color: iced::Color::WHITE,
                ..base
            }
        })
}
