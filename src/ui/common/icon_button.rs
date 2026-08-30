// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{ICON_BUTTON_PADDING, ICON_BUTTON_TEXT_SIZE, RADIUS_SM, darken, with_alpha};
use iced::border::{Border, Radius};
use iced::widget::{button, text};
use iced::{Alignment, Color, Font};

/// 图标按钮的通用样式：透明背景，悬停/按下时以图标色淡染填充
pub fn icon_button_style(
    icon_color: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let fill = match status {
            button::Status::Hovered => Some(with_alpha(icon_color, 0.10)),
            button::Status::Pressed => Some(with_alpha(icon_color, 0.18)),
            _ => None,
        };
        button::Style {
            text_color: icon_color,
            background: fill.map(iced::Background::Color),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..Default::default()
        }
    }
}

/// 实底图标按钮样式（工具栏按钮等）：悬停/按下时背景加深
pub fn solid_icon_button_style(
    background: Color,
    icon_color: Color,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let bg = match status {
            button::Status::Hovered => darken(background, 0.05),
            button::Status::Pressed => darken(background, 0.10),
            _ => background,
        };
        button::Style {
            text_color: icon_color,
            background: Some(iced::Background::Color(bg)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..Default::default()
        }
    }
}

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F30A}" download）
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
    .style(icon_button_style(icon_color))
    .on_press(message)
}

/// 创建带图标的操作按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F30A}" download）
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
    .style(icon_button_style(icon_color))
    .on_press(message)
}
