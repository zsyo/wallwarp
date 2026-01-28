// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::AspectRatio;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Color, Element, Length};

/// 创建比例网格选择器内容
pub fn create_ratio_grid_options<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    // 定义比例分组
    static RATIO_GROUPS: [(&str, &[(AspectRatio, &str)]); 4] = [
        (
            "online-wallpapers.ratio-group-wide",
            &[(AspectRatio::R16x9, "16x9"), (AspectRatio::R16x10, "16x10")],
        ),
        (
            "online-wallpapers.ratio-group-ultrawide",
            &[
                (AspectRatio::R21x9, "21x9"),
                (AspectRatio::R32x9, "32x9"),
                (AspectRatio::R48x9, "48x9"),
            ],
        ),
        (
            "online-wallpapers.ratio-group-portrait",
            &[
                (AspectRatio::R9x16, "9x16"),
                (AspectRatio::R10x16, "10x16"),
                (AspectRatio::R9x18, "9x18"),
            ],
        ),
        (
            "online-wallpapers.ratio-group-square",
            &[
                (AspectRatio::R1x1, "1x1"),
                (AspectRatio::R3x2, "3x2"),
                (AspectRatio::R4x3, "4x3"),
                (AspectRatio::R5x4, "5x4"),
            ],
        ),
    ];

    // 判断分组是否应该被禁用
    let is_wide_disabled = state.ratio_landscape_selected;
    let is_ultrawide_disabled = state.ratio_landscape_selected;
    let is_portrait_disabled = state.ratio_portrait_selected;
    let is_square_disabled = state.ratio_all_selected;
    let is_all_disabled = state.ratio_all_selected;

    // 判断额外选项是否应该被禁用
    let is_landscape_button_disabled = state.ratio_all_selected;
    let is_portrait_button_disabled = state.ratio_all_selected;
    let is_all_button_disabled = false;

    // 创建顶部额外选项按钮（水平居中）
    let landscape_button = button(text(i18n.t("online-wallpapers.ratio-mode-landscape")).size(14))
        .padding(6)
        .on_press(if is_landscape_button_disabled {
            AppMessage::None
        } else {
            AppMessage::Online(OnlineMessage::RatioLandscapeToggled)
        })
        .style(move |_theme, _status| {
            let is_selected = state.ratio_landscape_selected;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                theme_colors.light_button
            };
            let text_color = if is_landscape_button_disabled {
                DISABLED_COLOR
            } else if is_selected {
                Color::WHITE
            } else {
                theme_colors.light_text
            };
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            }
        });

    let portrait_button = button(text(i18n.t("online-wallpapers.ratio-mode-portrait")).size(14))
        .padding(6)
        .on_press(if is_portrait_button_disabled {
            AppMessage::None
        } else {
            AppMessage::Online(OnlineMessage::RatioPortraitToggled)
        })
        .style(move |_theme, _status| {
            let is_selected = state.ratio_portrait_selected;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                theme_colors.light_button
            };
            let text_color = if is_portrait_button_disabled {
                DISABLED_COLOR
            } else if is_selected {
                Color::WHITE
            } else {
                theme_colors.light_text
            };
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            }
        });

    let all_button = button(text(i18n.t("online-wallpapers.ratio-mode-all")).size(14))
        .padding(6)
        .on_press(if is_all_button_disabled {
            AppMessage::None
        } else {
            AppMessage::Online(OnlineMessage::RatioAllToggled)
        })
        .style(move |_theme, _status| {
            let is_selected = state.ratio_all_selected;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                theme_colors.light_button
            };
            let text_color = if is_selected {
                Color::WHITE
            } else {
                theme_colors.light_text
            };
            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            }
        });

    let option_buttons = container(row![landscape_button, portrait_button, all_button].spacing(4))
        .width(Length::Fill)
        .center_x(Length::Fill);

    // 创建比例表格（水平排列分组）
    let mut group_columns: Vec<Element<'a, AppMessage>> = Vec::new();

    for (group_name, ratios) in RATIO_GROUPS.iter() {
        // 确定该分组是否应该被禁用
        let is_group_disabled = match *group_name {
            "online-wallpapers.ratio-group-wide" => is_wide_disabled,
            "online-wallpapers.ratio-group-ultrawide" => is_ultrawide_disabled,
            "online-wallpapers.ratio-group-portrait" => is_portrait_disabled,
            "online-wallpapers.ratio-group-square" => is_square_disabled,
            _ => false,
        };

        // 创建分组标题（水平居中）
        let group_header = container(text(i18n.t(group_name)).size(14).color(theme_colors.light_text))
            .width(Length::Fill)
            .center_x(Length::Fill);

        // 创建分组内的比例按钮（每一行一个）
        let mut group_column = column![].spacing(2);
        for (ratio, label) in ratios.iter() {
            let is_selected = state.selected_ratios.contains(ratio);

            let border_color = if is_selected {
                COLOR_PICKER_ACTIVE
            } else {
                Color::TRANSPARENT
            };
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                Color::TRANSPARENT
            };
            let text_color = if is_all_disabled || is_group_disabled {
                DISABLED_COLOR
            } else {
                theme_colors.light_text
            };

            let button_content = container(text(*label).size(13))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            let ratio_button: Element<'a, AppMessage> = button(button_content)
                .padding(6)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: Radius::from(4.0),
                    },
                    ..button::text(_theme, _status)
                })
                .on_press(if is_all_disabled || is_group_disabled {
                    AppMessage::None
                } else {
                    AppMessage::Online(OnlineMessage::RatioToggled(*ratio))
                })
                .into();

            group_column = group_column.push(ratio_button);
        }

        // 将分组标题和内容组合，使用固定宽度
        let group_section =
            container(column![group_header, Space::new().height(Length::Fixed(4.0)), group_column,].spacing(0))
                .width(Length::Fixed(100.0));

        group_columns.push(group_section.into());
    }

    // 将所有分组水平排列
    let table_content = row(group_columns).spacing(2);

    // 创建比例选择器容器
    let picker_content = container(
        column![option_buttons, Space::new().height(Length::Fixed(12.0)), table_content,]
            .spacing(0)
            .align_x(Alignment::Center),
    )
    .padding(6)
    .width(Length::Fixed(460.0))
    .align_x(Alignment::Center)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.light_button)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        ..Default::default()
    });

    opaque(picker_content)
}
