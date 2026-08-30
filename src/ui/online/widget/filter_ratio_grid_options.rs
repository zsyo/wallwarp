// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::AspectRatio;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{dropdown_cell_style, dropdown_panel_style};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::ThemeColors;
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Element, Length};

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

    // 创建顶部额外选项按钮（水平居中）
    let mode_options: [(bool, bool, &str, OnlineMessage); 3] = [
        (
            state.ratio_landscape_selected,
            is_landscape_button_disabled,
            "online-wallpapers.ratio-mode-landscape",
            OnlineMessage::RatioLandscapeToggled,
        ),
        (
            state.ratio_portrait_selected,
            is_portrait_button_disabled,
            "online-wallpapers.ratio-mode-portrait",
            OnlineMessage::RatioPortraitToggled,
        ),
        (
            state.ratio_all_selected,
            false,
            "online-wallpapers.ratio-mode-all",
            OnlineMessage::RatioAllToggled,
        ),
    ];
    let option_buttons = container(
        row(mode_options
            .iter()
            .map(|(is_selected, is_disabled, key, msg)| {
                button(text(i18n.t(key)).size(14))
                    .padding(6)
                    .on_press(if *is_disabled {
                        AppMessage::None
                    } else {
                        msg.clone().into()
                    })
                    .style(dropdown_cell_style(
                        theme_colors,
                        *is_selected,
                        *is_disabled,
                    ))
                    .into()
            }))
        .spacing(4),
    )
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
        let group_header = container(
            text(i18n.t(group_name))
                .size(14)
                .color(theme_colors.light_text),
        )
        .width(Length::Fill)
        .center_x(Length::Fill);

        // 创建分组内的比例按钮（每一行一个）
        let mut group_column = column![].spacing(2);
        for (ratio, label) in ratios.iter() {
            let is_selected = state.selected_ratios.contains(ratio);

            let button_content = container(text(*label).size(13))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            let ratio_button: Element<'a, AppMessage> = button(button_content)
                .padding(6)
                .style(dropdown_cell_style(
                    theme_colors,
                    is_selected,
                    is_all_disabled || is_group_disabled,
                ))
                .on_press(if is_all_disabled || is_group_disabled {
                    AppMessage::None
                } else {
                    OnlineMessage::RatioToggled(*ratio).into()
                })
                .into();

            group_column = group_column.push(ratio_button);
        }

        // 将分组标题和内容组合，使用固定宽度
        let group_section = container(
            column![
                group_header,
                Space::new().height(Length::Fixed(4.0)),
                group_column,
            ]
            .spacing(0),
        )
        .width(Length::Fixed(100.0));

        group_columns.push(group_section.into());
    }

    // 将所有分组水平排列
    let table_content = row(group_columns).spacing(2);

    // 创建比例选择器容器
    let picker_content = container(
        column![
            option_buttons,
            Space::new().height(Length::Fixed(12.0)),
            table_content,
        ]
        .spacing(0)
        .align_x(Alignment::Center),
    )
    .padding(6)
    .width(Length::Fixed(460.0))
    .align_x(Alignment::Center)
    .style(dropdown_panel_style(theme_colors));

    opaque(picker_content)
}
