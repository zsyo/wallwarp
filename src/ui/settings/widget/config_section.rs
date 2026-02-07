// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::style::{SECTION_CONTENT_SPACING, SECTION_PADDING, SECTION_TITLE_SIZE};
use crate::ui::style::ThemeConfig;
use iced::widget::{Space, column, container, text};
use iced::{Alignment, Element, Length};

/// 创建配置区块
///
/// # 参数
/// - `title`: 区块标题
/// - `rows`: 区块内容行
pub(super) fn create_config_section<'a, Message: 'a>(
    title: String,
    rows: Vec<Element<'a, Message>>,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = theme_config.get_theme_colors();

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
        .style(common::create_bordered_container_style_with_bg(theme_config))
        .into()
}
