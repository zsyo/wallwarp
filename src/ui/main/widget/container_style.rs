// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::SEPARATOR_WIDTH;
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::border::{Border, Radius};
use iced::widget::container;
/// 创建主内容区容器样式（无边框，右侧添加分隔线）
pub fn create_main_container_style(theme_config: &ThemeConfig) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.background)),
        border: Border {
            color: theme_colors.separator,
            width: SEPARATOR_WIDTH,
            radius: Radius::from(0.0),
        },
        ..Default::default()
    }
}

/// 创建侧边栏容器样式（无边框，根据主题设置背景色）
pub fn create_sidebar_container_style(theme_config: &ThemeConfig) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0.0),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    }
}
