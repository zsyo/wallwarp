// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{
    ICON_BUTTON_PADDING, ICON_BUTTON_TEXT_SIZE, TOOLTIP_BG_COLOR, TOOLTIP_BORDER_COLOR, TOOLTIP_BORDER_RADIUS,
    TOOLTIP_BORDER_WIDTH,
};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::border::{Border, Radius};
use iced::widget::{button, container, text, tooltip};
use iced::{Alignment, Color, Element, Font};

/// 创建带 tooltip 的图标按钮
///
/// # 参数
/// - `icon_char`: 图标字符（如 "\u{F341}"）
/// - `icon_color`: 图标颜色
/// - `message`: 按钮点击消息
/// - `tooltip_text`: tooltip 文本
pub fn create_icon_button_with_tooltip<'a, Message>(
    icon_char: &'static str,
    icon_color: Color,
    message: Message,
    tooltip_text: String,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let btn = button(
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
    .on_press(message);

    tooltip(btn, text(tooltip_text), tooltip::Position::Top)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
            border: Border {
                color: TOOLTIP_BORDER_COLOR,
                width: TOOLTIP_BORDER_WIDTH,
                radius: Radius::from(TOOLTIP_BORDER_RADIUS),
            },
            ..Default::default()
        })
        .into()
}

/// 创建带提示的按钮
///
/// # 参数
/// - `button`: 按钮组件
/// - `tooltip_text`: tooltip 文本
/// - `position`: tooltip 显示位置
/// - `theme_config`: 主题配置
pub fn create_button_with_tooltip<'a, Message>(
    button: button::Button<'a, Message>,
    tooltip_text: String,
    position: tooltip::Position,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    // 使用主题的文本颜色创建 tooltip 文本
    let tooltip_text_element = text(tooltip_text).style(move |_theme: &iced::Theme| text::Style {
        color: Some(theme_colors.text),
    });

    tooltip(button, tooltip_text_element, position)
        .gap(5.0)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.tooltip_bg_color)),
            border: Border {
                color: theme_colors.tooltip_border_color,
                width: TOOLTIP_BORDER_WIDTH,
                radius: Radius::from(TOOLTIP_BORDER_RADIUS),
            },
            ..Default::default()
        })
        .into()
}
