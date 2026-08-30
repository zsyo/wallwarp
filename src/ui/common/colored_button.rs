// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{BUTTON_TEXT_SIZE, RADIUS_SM, darken, with_alpha};
use iced::border::{Border, Radius};
use iced::widget::{button, text};
use iced::{Alignment, Color, Element};

/// 彩色按钮的通用样式：悬停调暗、按下进一步调暗、禁用降透明度
fn colored_button_style(color: Color) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |theme, status| {
        let base = button::primary(theme, status);
        let (bg, text_color) = match status {
            button::Status::Active => (color, Color::WHITE),
            button::Status::Hovered => (darken(color, 0.08), Color::WHITE),
            button::Status::Pressed => (darken(color, 0.15), Color::WHITE),
            button::Status::Disabled => (with_alpha(color, 0.4), with_alpha(Color::WHITE, 0.8)),
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..base
        }
    }
}

/// 创建带颜色的按钮（接收文本字符串）
pub fn create_colored_button<'a, Message>(
    label: String,
    color: Color,
    message: Message,
) -> button::Button<'a, Message>
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
    .style(colored_button_style(color))
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
        .style(colored_button_style(color))
}
