// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{BORDER_RADIUS, BORDER_WIDTH, COLOR_SIDEBAR_BG, shadows::CARD_SHADOW};
use crate::ui::style::{ThemeColors, ThemeConfig};
use iced::border::{Border, Radius};
use iced::widget::container;

/// 创建带边框的容器样式
pub fn create_bordered_container_style(_theme: &iced::Theme) -> container::Style {
    use COLOR_SIDEBAR_BG;

    container::Style {
        background: Some(iced::Background::Color(COLOR_SIDEBAR_BG)),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: iced::Vector { x: 0.0, y: 2.0 },
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

/// 创建带边框的容器样式（带背景色）
///
/// # 参数
/// - `theme`: 主题
/// - `bg_color`: 背景颜色
pub fn create_bordered_container_style_with_bg(
    theme_config: &ThemeConfig,
) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: Border {
            color: theme_colors.border,
            width: BORDER_WIDTH,
            radius: Radius::from(BORDER_RADIUS),
        },
        shadow: CARD_SHADOW,
        ..Default::default()
    }
}
