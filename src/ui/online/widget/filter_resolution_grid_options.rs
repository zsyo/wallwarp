// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::Resolution;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState, ResolutionMode};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Color, Element, Length};

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
                (Resolution::R3840x3072, "3840x3072"),
            ],
        ),
    ];

    // 判断分辨率列表是否禁用
    let is_list_disabled = state.resolution_mode == ResolutionMode::All;

    // 创建顶部模式切换按钮（水平居中）
    let atleast_button = button(text(i18n.t("online-wallpapers.resolution-mode-atleast")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            ResolutionMode::AtLeast,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == ResolutionMode::AtLeast;
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

    let exactly_button = button(text(i18n.t("online-wallpapers.resolution-mode-exactly")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            ResolutionMode::Exactly,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == ResolutionMode::Exactly;
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

    let all_button = button(text(i18n.t("online-wallpapers.resolution-mode-all")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            ResolutionMode::All,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == ResolutionMode::All;
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

    let mode_buttons = container(row![atleast_button, exactly_button, all_button].spacing(4))
        .width(Length::Fill)
        .center_x(Length::Fill);

    // 创建分辨率表格（水平排列分组）
    let mut group_columns: Vec<Element<'a, AppMessage>> = Vec::new();

    for (group_name, resolutions) in RESOLUTION_GROUPS.iter() {
        // 创建分组标题（水平居中）
        let group_header = container(text(i18n.t(group_name)).size(14).color(theme_colors.light_text))
            .width(Length::Fill)
            .center_x(Length::Fill);

        // 创建分组内的分辨率按钮（每一行一个）
        let mut group_column = column![].spacing(2);
        for (resolution, label) in resolutions.iter() {
            let is_selected = if state.resolution_mode == ResolutionMode::AtLeast {
                state.atleast_resolution == Some(*resolution)
            } else if state.resolution_mode == ResolutionMode::Exactly {
                state.selected_resolutions.contains(resolution)
            } else {
                false
            };

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
            let text_color = if is_list_disabled {
                DISABLED_COLOR
            } else {
                theme_colors.light_text
            };

            let button_content = container(text(*label).size(13))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            let res_button: Element<'a, AppMessage> = button(button_content)
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
                .on_press(if is_list_disabled {
                    AppMessage::None
                } else if state.resolution_mode == ResolutionMode::AtLeast {
                    AppMessage::Online(OnlineMessage::ResolutionAtLeastSelected(*resolution))
                } else {
                    AppMessage::Online(OnlineMessage::ResolutionToggled(*resolution))
                })
                .into();

            group_column = group_column.push(res_button);
        }

        // 将分组标题和内容组合，使用固定宽度
        let group_section =
            container(column![group_header, Space::new().height(Length::Fixed(4.0)), group_column].spacing(0))
                .width(Length::Fixed(100.0));

        group_columns.push(group_section.into());
    }

    // 将所有分组水平排列
    let table_content = row(group_columns).spacing(2);

    // 创建分辨率选择器容器
    let picker_content = container(
        column![mode_buttons, Space::new().height(Length::Fixed(12.0)), table_content,]
            .spacing(0)
            .align_x(Alignment::Center),
    )
    .padding(12)
    .width(Length::Fixed(530.0))
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
