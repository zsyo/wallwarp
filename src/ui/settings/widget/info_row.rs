// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::ROW_SPACING;
use crate::ui::style::ThemeColors;
use iced::widget::{row, text};
use iced::{Alignment, Element, Length};

/// 创建信息行
///
/// # 参数
/// - `label`: 标签文本
/// - `value`: 值文本
/// - `theme_colors`: 主题颜色
pub fn create_info_row<'a, Message: 'a>(
    label: String,
    value: String,
    theme_colors: ThemeColors,
) -> Element<'a, Message> {
    row![
        text(label).style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        }),
        text(value)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.light_text),
            }),
    ]
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}
