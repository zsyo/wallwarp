// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::AspectRatio;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::dropdown_cell_style;
use crate::ui::online::widget::filter_grouped_grid::{grid_cell_button, grouped_grid_options};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::ThemeColors;
use iced::Element;
use iced::widget::{button, text};

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

    // 按分组顺序的禁用条件：宽屏/超宽跟随横屏模式，竖屏跟随竖屏模式，
    // 方形跟随全部模式；全部选中时所有网格单元一并禁用
    let group_disabled = [
        state.ratio_landscape_selected,
        state.ratio_landscape_selected,
        state.ratio_portrait_selected,
        state.ratio_all_selected,
    ];
    let is_all_disabled = state.ratio_all_selected;

    // 创建顶部额外选项按钮（横屏/竖屏/全部）
    let mode_options: [(bool, bool, &str, OnlineMessage); 3] = [
        (
            state.ratio_landscape_selected,
            state.ratio_all_selected,
            "online-wallpapers.ratio-mode-landscape",
            OnlineMessage::RatioLandscapeToggled,
        ),
        (
            state.ratio_portrait_selected,
            state.ratio_all_selected,
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
    let mode_buttons: Vec<Element<'a, AppMessage>> = mode_options
        .iter()
        .map(|(is_selected, is_disabled, key, msg)| {
            button(text(i18n.t(key)).size(14))
                .padding(6)
                .on_press_maybe(if *is_disabled {
                    None
                } else {
                    Some(msg.clone().into())
                })
                .style(dropdown_cell_style(
                    theme_colors,
                    *is_selected,
                    *is_disabled,
                ))
                .into()
        })
        .collect();

    // 分组内容
    let groups: Vec<(String, Vec<Element<'a, AppMessage>>)> = RATIO_GROUPS
        .iter()
        .zip(group_disabled)
        .map(|((group_name, ratios), is_group_disabled)| {
            let cells = ratios
                .iter()
                .map(|(ratio, label)| {
                    let is_selected = state.selected_ratios.contains(ratio);
                    grid_cell_button(
                        label,
                        is_selected,
                        is_all_disabled || is_group_disabled,
                        Some(OnlineMessage::RatioToggled(*ratio).into()),
                        theme_colors,
                    )
                })
                .collect();
            (i18n.t(group_name).to_string(), cells)
        })
        .collect();

    grouped_grid_options(mode_buttons, groups, 460.0, 6.0, theme_colors)
}
