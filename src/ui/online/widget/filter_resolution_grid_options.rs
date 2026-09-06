// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::Resolution;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::dropdown_option_style;
use crate::ui::online::widget::filter_grouped_grid::{grid_cell_button, grouped_grid_options};
use crate::ui::online::{OnlineMessage, OnlineState, ResolutionMode};
use crate::ui::style::ThemeColors;
use iced::Element;
use iced::widget::{button, text};

/// 创建分辨率网格选择器内容
pub fn create_resolution_grid_options<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    // 定义分辨率分组（按尺寸从小到大排序）
    static RESOLUTION_GROUPS: [(&str, &[(Resolution, &str)]); 5] = [
        (
            "online-wallpapers.resolution-group-ultrawide",
            &[
                (Resolution::R2560x1080, "2560x1080"),
                (Resolution::R2560x1440U, "2560x1440"),
                (Resolution::R3840x1600, "3840x1600"),
            ],
        ),
        (
            "online-wallpapers.resolution-group-16-9",
            &[
                (Resolution::R1280x720, "1280x720"),
                (Resolution::R1600x900, "1600x900"),
                (Resolution::R1920x1080, "1920x1080"),
                (Resolution::R2560x1440, "2560x1440"),
                (Resolution::R3840x2160, "3840x2160"),
            ],
        ),
        (
            "online-wallpapers.resolution-group-16-10",
            &[
                (Resolution::R1280x800, "1280x800"),
                (Resolution::R1600x1000, "1600x1000"),
                (Resolution::R1920x1200, "1920x1200"),
                (Resolution::R2560x1600, "2560x1600"),
                (Resolution::R3840x2400, "3840x2400"),
            ],
        ),
        (
            "online-wallpapers.resolution-group-4-3",
            &[
                (Resolution::R1280x960, "1280x960"),
                (Resolution::R1600x1200_4_3, "1600x1200"),
                (Resolution::R1920x1440, "1920x1440"),
                (Resolution::R2560x1920, "2560x1920"),
                (Resolution::R3840x2880, "3840x2880"),
            ],
        ),
        (
            "online-wallpapers.resolution-group-5-4",
            &[
                (Resolution::R1280x1024, "1280x1024"),
                (Resolution::R1600x1280, "1600x1280"),
                (Resolution::R1920x1536, "1920x1536"),
                (Resolution::R2560x2048, "2560x2048"),
                (Resolution::R3840x2880, "3840x2880"),
            ],
        ),
    ];

    // 分辨率列表在"全部"模式下禁用
    let is_list_disabled = state.resolution_mode == ResolutionMode::All;

    // 创建顶部模式切换按钮（至少/精确/全部）
    let mode_options = [
        (
            ResolutionMode::AtLeast,
            "online-wallpapers.resolution-mode-atleast",
        ),
        (
            ResolutionMode::Exactly,
            "online-wallpapers.resolution-mode-exactly",
        ),
        (ResolutionMode::All, "online-wallpapers.resolution-mode-all"),
    ];
    let mode_buttons: Vec<Element<'a, AppMessage>> = mode_options
        .iter()
        .map(|(mode, key)| {
            let is_selected = state.resolution_mode == *mode;
            button(text(i18n.t(key)).size(14))
                .padding(6)
                .on_press(OnlineMessage::ResolutionModeChanged(*mode).into())
                .style(dropdown_option_style(theme_colors, is_selected))
                .into()
        })
        .collect();

    // 分组内容
    let groups: Vec<(String, Vec<Element<'a, AppMessage>>)> = RESOLUTION_GROUPS
        .iter()
        .map(|(group_name, resolutions)| {
            let cells = resolutions
                .iter()
                .map(|(resolution, label)| {
                    // 选中判断与选中消息均随模式变化
                    let (is_selected, message) = if state.resolution_mode == ResolutionMode::AtLeast
                    {
                        (
                            state.atleast_resolution == Some(*resolution),
                            OnlineMessage::ResolutionAtLeastSelected(*resolution),
                        )
                    } else {
                        (
                            state.selected_resolutions.contains(resolution),
                            OnlineMessage::ResolutionToggled(*resolution),
                        )
                    };
                    grid_cell_button(
                        label,
                        is_selected,
                        is_list_disabled,
                        Some(message.into()),
                        theme_colors,
                    )
                })
                .collect();
            (i18n.t(group_name).to_string(), cells)
        })
        .collect();

    grouped_grid_options(mode_buttons, groups, 530.0, 12.0, theme_colors)
}
