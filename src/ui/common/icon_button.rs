// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{ICON_BUTTON_PADDING, ICON_BUTTON_TEXT_SIZE};
use iced::border::{Border, Radius};
use iced::widget::{button, text};
use iced::{Alignment, Color, Font};

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `message`: 按钮点击消息
pub fn create_icon_button<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(icon_char)
            .color(icon_color)
            .font(Font::with_name("bootstrap-icons"))
            .size(ICON_BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(ICON_BUTTON_PADDING)
    .style(|_theme: &iced::Theme, _status| button::Style {
        text_color: iced::Color::WHITE,
        background: None,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0.0),
        },
        ..Default::default()
    })
    .on_press(message)
}

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `size`: 按钮大小
/// - `message`: 按钮点击消息
pub fn create_icon_button_with_size<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    size: impl Into<iced::Pixels>,
    message: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(
        text(icon_char)
            .color(icon_color)
            .font(Font::with_name("bootstrap-icons"))
            .size(size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(ICON_BUTTON_PADDING)
    .style(|_theme: &iced::Theme, _status| button::Style {
        text_color: iced::Color::WHITE,
        background: None,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0.0),
        },
        ..Default::default()
    })
    .on_press(message)
}
