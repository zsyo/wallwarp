// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 通用复选框样式：选中=强调色底+白色对勾，未选中=对话框底色

use crate::ui::style::ThemeColors;

/// 统一复选框样式：选中=强调色底+白色对勾，未选中=对话框底色
pub fn checkbox_style(
    theme_colors: ThemeColors,
    is_checked: bool,
) -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
    move |_theme: &iced::Theme, _status| iced::widget::checkbox::Style {
        background: iced::Background::Color(if is_checked {
            theme_colors.primary
        } else {
            theme_colors.dialog_bg
        }),
        border: iced::Border {
            color: if is_checked {
                theme_colors.primary
            } else {
                theme_colors.border
            },
            width: 1.0,
            radius: 3.0.into(),
        },
        text_color: Some(theme_colors.text),
        icon_color: iced::Color::WHITE,
    }
}
