// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::ThemeColors;
use crate::ui::style::{TOOLTIP_BORDER_RADIUS, TOOLTIP_BORDER_WIDTH};
use iced::border::{Border, Radius};
use iced::widget::{container, radio, text, tooltip};
use iced::{Alignment, Color, Element, Length};

/// 创建带提示的单选按钮
///
/// # 参数
/// - `label`: 标签文本
/// - `value`: 选项值
/// - `selected_value`: 当前选中的值
/// - `on_selected`: 选中时的回调
/// - `tooltip_text`: 提示文本
/// - `theme_colors`: 主题颜色
pub fn create_radio_with_tooltip<'a, Message, V>(
    label: String,
    value: V,
    selected_value: Option<V>,
    on_selected: impl FnOnce(V) -> Message + 'a,
    tooltip_text: String,
    theme_colors: ThemeColors,
) -> Element<'a, Message>
where
    V: Copy + Eq + 'a,
    Message: Clone + 'a,
{
    let radio_button = radio(label, value, selected_value, on_selected)
        .size(16)
        .spacing(8)
        .style(move |theme: &iced::Theme, status| radio::Style {
            text_color: Some(theme_colors.text),
            background: iced::Background::Color(Color::TRANSPARENT),
            ..radio::default(theme, status)
        });

    let content = container(radio_button)
        .height(Length::Fixed(30.0))
        .align_y(Alignment::Center);

    // 使用主题的文本颜色创建 tooltip 文本
    let tooltip_text_element = text(tooltip_text).style(move |_theme: &iced::Theme| text::Style {
        color: Some(theme_colors.text),
    });

    tooltip(content, tooltip_text_element, tooltip::Position::Top)
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
