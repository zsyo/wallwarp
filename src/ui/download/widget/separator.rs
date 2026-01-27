// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::AppMessage;
use crate::ui::style::TABLE_SEPARATOR_WIDTH;
use crate::ui::style::ThemeColors;
use crate::ui::style::ThemeConfig;
use iced::widget::{Space, container};
use iced::{Element, Length};

/// 创建水平分隔线
pub fn create_horizontal_separator(theme_config: &ThemeConfig) -> Element<'_, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    container(Space::new())
        .width(Length::Fill)
        .height(TABLE_SEPARATOR_WIDTH)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.table_separator_color)),
            ..Default::default()
        })
        .into()
}

/// 创建垂直分隔线
pub fn create_vertical_separator(theme_config: &ThemeConfig) -> Element<'_, AppMessage> {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    container(Space::new())
        .width(TABLE_SEPARATOR_WIDTH)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.table_separator_color)),
            ..Default::default()
        })
        .into()
}
