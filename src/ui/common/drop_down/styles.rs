// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下拉组件的统一样式与标准触发按钮
//!
//! 全项目的下拉选择器（设置页/在线筛选/下载筛选）应使用此处提供的
//! 样式函数与触发按钮构造器，保证面板、选项、悬停与选中态一致。

use crate::ui::style::{RADIUS_MD, RADIUS_SM, ThemeColors, darken, shadows::DIALOG_SHADOW, tint};
use iced::border::{Border, Radius};
use iced::widget::{Row, Space, button, container, row, text};
use iced::{Alignment, Color, Font, Length};

/// 下拉箭头图标（bootstrap-icons chevron-down）
pub const CHEVRON_DOWN: &str = "\u{F282}";

/// 触发按钮内容：左侧文字 + 右侧 chevron 图标
fn trigger_underlay<'a, Message>(
    label: String,
    theme_colors: ThemeColors,
) -> Row<'a, Message>
where
    Message: Clone + 'a,
{
    row![
        text(label).size(14).color(theme_colors.light_text),
        Space::new().width(Length::Fill),
        text(CHEVRON_DOWN)
            .font(Font::with_name("bootstrap-icons"))
            .size(12)
            .color(theme_colors.light_text_sub),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
}

/// 标准下拉触发按钮：左侧文字 + 右侧 chevron 图标
///
/// 悬停时边框加深、按下时边框变强调色。
pub fn dropdown_trigger_button<'a, Message>(
    label: String,
    width: f32,
    theme_colors: ThemeColors,
    on_press: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(trigger_underlay(label, theme_colors))
        .padding(iced::Padding {
            top: 6.0,
            bottom: 6.0,
            left: 8.0,
            right: 8.0,
        })
        .width(Length::Fixed(width))
        .on_press(on_press)
        .style(move |_theme: &iced::Theme, status| {
            let border_color = match status {
                button::Status::Hovered => theme_colors.secondary,
                button::Status::Pressed => theme_colors.primary,
                _ => theme_colors.border,
            };
            button::Style {
                background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
                text_color: theme_colors.light_text,
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: Radius::from(RADIUS_SM),
                },
                ..button::text(_theme, status)
            }
        })
}

/// 扁平下拉触发按钮：无边框浅底样式，与在线筛选栏的分辨率/比例/颜色触发按钮一致
///
/// 悬停/按下时背景轻微加深。
pub fn flat_dropdown_trigger_button<'a, Message>(
    label: String,
    width: f32,
    theme_colors: ThemeColors,
    on_press: Message,
) -> button::Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(trigger_underlay(label, theme_colors))
        .padding(iced::Padding {
            top: 6.0,
            bottom: 6.0,
            left: 8.0,
            right: 8.0,
        })
        .width(Length::Fixed(width))
        .on_press(on_press)
        .style(move |_theme: &iced::Theme, status| {
            let bg = match status {
                button::Status::Hovered => darken(theme_colors.light_button, 0.05),
                button::Status::Pressed => darken(theme_colors.light_button, 0.10),
                _ => theme_colors.light_button,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: theme_colors.light_text,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(RADIUS_SM),
                },
                ..button::text(_theme, status)
            }
        })
}

/// 下拉列表选项按钮样式
///
/// 选中 = 强调色淡染底 + 强调色文字；未选中悬停 = 中性淡染底。
pub fn dropdown_option_style(
    theme_colors: ThemeColors,
    is_selected: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let bg = if is_selected {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    tint(theme_colors.primary, 0.20)
                }
                _ => tint(theme_colors.primary, 0.12),
            }
        } else {
            match status {
                button::Status::Hovered => theme_colors.hover_fill,
                button::Status::Pressed => tint(theme_colors.primary, 0.08),
                _ => Color::TRANSPARENT,
            }
        };
        let text_color = if is_selected {
            theme_colors.primary
        } else {
            theme_colors.light_text
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(RADIUS_SM),
            },
            ..button::text(_theme, status)
        }
    }
}

/// 下拉网格单元格按钮样式（分辨率/比例等网格选项）
///
/// 选中 = 强调色淡染底 + 强调色 1.5px 描边 + 强调色文字；
/// 禁用 = 弱化文字且无交互反馈。
pub fn dropdown_cell_style(
    theme_colors: ThemeColors,
    is_selected: bool,
    is_disabled: bool,
) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, status| {
        let (bg, border_color, border_width, text_color) = if is_disabled {
            (
                Color::TRANSPARENT,
                Color::TRANSPARENT,
                0.0,
                theme_colors.disabled_color,
            )
        } else if is_selected {
            (
                tint(theme_colors.primary, 0.12),
                theme_colors.primary,
                1.5,
                theme_colors.primary,
            )
        } else {
            let bg = match status {
                button::Status::Hovered => theme_colors.hover_fill,
                button::Status::Pressed => tint(theme_colors.primary, 0.08),
                _ => Color::TRANSPARENT,
            };
            (bg, Color::TRANSPARENT, 0.0, theme_colors.light_text)
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color,
            border: Border {
                color: border_color,
                width: border_width,
                radius: Radius::from(RADIUS_SM),
            },
            ..button::text(_theme, status)
        }
    }
}

/// 下拉面板容器样式：主题底色 + 1px 边框 + 悬浮阴影
pub fn dropdown_panel_style(
    theme_colors: ThemeColors,
) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.dialog_bg)),
        border: Border {
            color: theme_colors.border,
            width: 1.0,
            radius: Radius::from(RADIUS_MD),
        },
        shadow: DIALOG_SHADOW,
        ..Default::default()
    }
}
