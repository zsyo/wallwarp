// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::ui::common;
use crate::ui::style::ThemeConfig;
use crate::ui::style::{SECTION_PADDING, SECTION_TITLE_SIZE, SETTING_ROW_DESC_SIZE};
use iced::widget::{column, container, text};
use iced::{Alignment, Element, Font, Length};

/// 创建"功能开发中"占位区块
///
/// 居中图标 + 标题 + 说明文字，用于尚未实现的功能分类。
///
/// # 参数
/// - `icon`: bootstrap 图标字符（码点须已验证）
/// - `title`: 标题文本
/// - `description`: 说明文本
/// - `theme_config`: 主题配置
pub fn create_placeholder_section<'a, Message: 'a>(
    icon: &'static str,
    title: String,
    description: String,
    theme_config: &'a ThemeConfig,
) -> Element<'a, Message> {
    let theme_colors = theme_config.get_theme_colors();

    let content = column![
        text(icon)
            .font(Font::with_name("bootstrap-icons"))
            .size(32.0)
            .color(theme_colors.disabled_color),
        text(title)
            .size(SECTION_TITLE_SIZE)
            .color(theme_colors.text),
        text(description)
            .size(SETTING_ROW_DESC_SIZE)
            .color(theme_colors.light_text_sub),
    ]
    .spacing(10)
    .align_x(Alignment::Center)
    .width(Length::Fill);

    container(content)
        .width(Length::Fill)
        .padding(SECTION_PADDING)
        .style(common::create_bordered_container_style_with_bg(
            theme_config,
        ))
        .into()
}
