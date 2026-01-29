// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{INPUT_HEIGHT, ROW_SPACING, SECTION_CONTENT_SPACING, SECTION_PADDING, SECTION_TITLE_SIZE};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建配置区块
///
/// # 参数
/// - `title`: 区块标题
/// - `rows`: 区块内容行
pub fn create_config_section<'a, Message: 'a>(
    title: String,
    rows: Vec<Element<'a, Message>>,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let mut column_content = column!(
        text(title)
            .size(SECTION_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
    )
    .spacing(SECTION_CONTENT_SPACING);
    column_content = column_content.push(Space::new().height(Length::Fixed(20.0)));

    for row in rows {
        column_content = column_content.push(row);
    }

    container(column_content)
        .padding(SECTION_PADDING)
        .width(Length::Fill)
        .style(super::create_bordered_container_style_with_bg(theme_config))
        .into()
}

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
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

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
