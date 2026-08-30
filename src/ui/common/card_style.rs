// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸卡片统一样式
//!
//! 本地/在线壁纸卡片、加载与错误占位卡片共用：
//! 按钮层负责阴影与悬停描边，内层图片容器负责圆角裁剪。

use crate::ui::style::{
    BORDER_WIDTH, RADIUS_MD, ThemeColors, shadows::CARD_SHADOW, shadows::CARD_SHADOW_HOVER, tint,
};
use iced::Background;
use iced::border::{Border, Radius};
use iced::widget::{button, container};

/// 壁纸卡片按钮样式：柔和阴影 + 悬停抬升 + 强调色描边
pub fn wallpaper_card_button_style(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let (shadow, border_color) = match status {
            button::Status::Hovered | button::Status::Pressed => {
                (CARD_SHADOW_HOVER, tint(theme_colors.primary, 0.45))
            }
            _ => (CARD_SHADOW, theme_colors.border),
        };
        button::Style {
            background: Some(Background::Color(theme_colors.sidebar_bg)),
            text_color: theme_colors.text,
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: Radius::from(RADIUS_MD),
            },
            shadow,
            ..button::text(_theme, status)
        }
    }
}

/// 卡片内图片容器样式：主题占位底色 + 1px 边框 + 圆角
///
/// 配合 `container(...).clip(true)` 使用，实现图片真圆角裁剪。
pub fn wallpaper_image_container_style(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme: &iced::Theme| container::Style {
        background: Some(Background::Color(theme_colors.sidebar_bg)),
        border: Border {
            color: theme_colors.border,
            width: BORDER_WIDTH,
            radius: Radius::from(RADIUS_MD),
        },
        ..Default::default()
    }
}
