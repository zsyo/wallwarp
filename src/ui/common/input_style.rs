// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 统一的文本输入框与进度条样式
//!
//! 全项目所有 text_input / progress_bar 应使用此处提供的样式函数，
//! 保证边框、焦点态、圆角与主题配色一致。

use crate::ui::style::{RADIUS_SM, ThemeColors, with_alpha};
use iced::Color;
use iced::border::Border;
use iced::widget::{progress_bar, text_input};

/// 统一的文本输入框样式：主题底色 + 1px 边框 + 焦点态强调色描边
///
/// 禁用态自动弱化（底色降透明、文字用禁用色），调用方无需再自行计算。
pub fn styled_text_input(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style {
    move |_theme: &iced::Theme, status| {
        let (bg, border_color, border_width, value_color, sub_color) = match status {
            text_input::Status::Focused { .. } => (
                theme_colors.text_input_background,
                theme_colors.primary,
                1.5,
                theme_colors.light_text,
                theme_colors.light_text_sub,
            ),
            text_input::Status::Hovered => (
                theme_colors.text_input_background,
                theme_colors.secondary,
                1.0,
                theme_colors.light_text,
                theme_colors.light_text_sub,
            ),
            text_input::Status::Disabled => (
                with_alpha(theme_colors.text_input_background, 0.45),
                theme_colors.border,
                1.0,
                theme_colors.disabled_color,
                theme_colors.disabled_color,
            ),
            text_input::Status::Active => (
                theme_colors.text_input_background,
                theme_colors.border,
                1.0,
                theme_colors.light_text,
                theme_colors.light_text_sub,
            ),
        };
        text_input::Style {
            background: iced::Background::Color(bg),
            border: Border {
                color: border_color,
                width: border_width,
                radius: RADIUS_SM.into(),
            },
            icon: sub_color,
            placeholder: sub_color,
            value: value_color,
            selection: theme_colors.text_input_selection_color,
        }
    }
}

/// 统一的进度条样式：主题轨道底 + 强调色填充 + 圆角
pub fn styled_progress_bar(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme) -> progress_bar::Style {
    move |_theme: &iced::Theme| progress_bar::Style {
        background: iced::Background::Color(theme_colors.light_bg),
        bar: iced::Background::Color(theme_colors.primary),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: RADIUS_SM.into(),
        },
    }
}
