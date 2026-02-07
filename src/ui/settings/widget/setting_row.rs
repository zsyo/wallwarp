// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{INPUT_HEIGHT, ROW_SPACING};
use crate::ui::style::ThemeConfig;
use iced::widget::{row, text};
use iced::{Alignment, Element, Length};

/// 创建设置行
///
/// # 参数
/// - `label`: 标签文本
/// - `widget`: 控件
/// - `theme_config`: 主题配置
pub fn create_setting_row<'a, Message: 'a>(
    label: String,
    widget: impl Into<Element<'a, Message>>,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = theme_config.get_theme_colors();

    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        widget.into(),
    ]
    .align_y(Alignment::Center)
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}
