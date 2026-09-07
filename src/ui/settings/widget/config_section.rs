// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{
    ROW_SPACING, SECTION_CONTENT_SPACING, SECTION_ICON_SIZE, SECTION_PADDING, SECTION_TITLE_SIZE,
};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Font, Length};

/// 创建配置区块
///
/// 头部为水平居中的图标 + 标题，内容为设置行列表。
///
/// # 参数
/// - `icon`: 区块图标（bootstrap 图标字符，码点须已验证）
/// - `title`: 区块标题
/// - `rows`: 区块内容行
/// - `theme_config`: 主题配置
pub(super) fn create_config_section<'a, Message: 'a>(
    icon: &'static str,
    title: String,
    rows: Vec<Element<'a, Message>>,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = theme_config.get_theme_colors();

    let header = container(
        row![
            text(icon)
                .font(Font::with_name("bootstrap-icons"))
                .size(SECTION_ICON_SIZE)
                .color(theme_colors.primary),
            text(title)
                .size(SECTION_TITLE_SIZE)
                .color(theme_colors.text),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .align_x(Alignment::Center);

    let mut rows_column = column![].spacing(ROW_SPACING);
    for row in rows {
        rows_column = rows_column.push(row);
    }

    let column_content = column![header, rows_column].spacing(SECTION_CONTENT_SPACING);

    container(column_content)
        .padding(SECTION_PADDING)
        .width(Length::Fill)
        .style(common::create_bordered_container_style_with_bg(
            theme_config,
        ))
        .into()
}
