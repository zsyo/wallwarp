// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::{Category, Purity, Sorting};
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use crate::utils::config::Config;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

/// 创建筛选栏
pub fn create_filter_bar<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    config: &'a Config,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    // 搜索框（放在最前面）
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    let search_input = text_input(&i18n.t("online-wallpapers.search-placeholder"), &state.search_text)
        .on_input(|text| OnlineMessage::SearchTextChanged(text).into())
        .on_submit(OnlineMessage::Search.into())
        .padding(6)
        .size(14)
        .width(Length::Fixed(160.0))
        .style(move |_theme: &iced::Theme, _status| text_input::Style {
            background: iced::Background::Color(theme_colors.light_button),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            icon: theme_colors.light_text_sub,
            placeholder: theme_colors.light_text_sub,
            value: theme_colors.light_text,
            selection: theme_colors.text_input_selection_color,
        });

    let search_button =
        common::create_icon_button_with_size("\u{F52A}", BUTTON_COLOR_BLUE, 17, OnlineMessage::Search.into()).style(
            move |_theme: &iced::Theme, _status| button::Style {
                background: Some(iced::Background::Color(theme_colors.light_button)),
                text_color: theme_colors.light_text,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            },
        );

    let search_container = row![search_input, search_button].spacing(2).align_y(Alignment::Center);

    // 分辨率选择器 - 使用 DropDown 组件
    let resolution_picker = super::create_resolution_picker(i18n, state, theme_colors);

    // 比例选择器 - 使用 DropDown 组件（支持多选）
    let ratio_picker = super::create_ratio_picker(i18n, state, theme_colors);

    // 颜色选择器 - 使用 DropDown 组件
    let color_picker = super::create_color_picker(i18n, state, theme_colors);

    let sorting_picker = super::create_sorting_picker(i18n, state, theme_colors);

    let time_range_picker = super::create_time_range_picker(i18n, state, theme_colors);

    // 功能按钮
    let refresh_button =
        common::create_icon_button_with_size("\u{F130}", BUTTON_COLOR_GREEN, 20, OnlineMessage::Refresh.into()).style(
            move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(theme_colors.light_button)),
                text_color: theme_colors.light_text,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            },
        );

    // 组合所有元素
    let filter_row = row![
        search_container,
        Space::new().width(2),
        // 分类按钮（选中状态为蓝色）
        button(text(i18n.t("online-wallpapers.category-general")).size(14))
            .on_press(OnlineMessage::CategoryToggled(Category::General).into())
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::General.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    theme_colors.light_button
                };
                let text_color = if is_checked {
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
            }),
        button(text(i18n.t("online-wallpapers.category-anime")).size(14))
            .on_press(OnlineMessage::CategoryToggled(Category::Anime).into())
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::Anime.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    theme_colors.light_button
                };
                let text_color = if is_checked {
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
            }),
        button(text(i18n.t("online-wallpapers.category-people")).size(14))
            .on_press(OnlineMessage::CategoryToggled(Category::People).into())
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::People.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    theme_colors.light_button
                };
                let text_color = if is_checked {
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
            }),
        Space::new().width(2),
        // 纯净度按钮（带颜色）
        button(text(i18n.t("online-wallpapers.purity-sfw")).size(14))
            .on_press(OnlineMessage::PurityToggled(Purity::SFW).into())
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::SFW.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SFW, Color::WHITE)
                } else {
                    (theme_colors.light_button, theme_colors.light_text)
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
            }),
        button(text(i18n.t("online-wallpapers.purity-sketchy")).size(14))
            .on_press(OnlineMessage::PurityToggled(Purity::Sketchy).into())
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::Sketchy.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SKETCHY, Color::BLACK)
                } else {
                    (theme_colors.light_button, theme_colors.light_text)
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
            }),
        // NSFW 按钮：只在 API Key 不为空时显示
        if !config.wallhaven.api_key.is_empty() {
            Some(
                button(text(i18n.t("online-wallpapers.purity-nsfw")).size(14))
                    .on_press(OnlineMessage::PurityToggled(Purity::NSFW).into())
                    .padding(6)
                    .style(move |_theme, _status| {
                        let is_checked = (state.purities & Purity::NSFW.bit_value()) != 0;
                        let (bg_color, text_color) = if is_checked {
                            (COLOR_NSFW, Color::WHITE)
                        } else {
                            (theme_colors.light_button, theme_colors.light_text)
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
                    }),
            )
        } else {
            None
        },
        Space::new().width(2),
        resolution_picker,
        ratio_picker,
        color_picker,
        sorting_picker,
        // 时间范围选择器：仅在排序为 TopList 时显示
        if state.sorting == Sorting::TopList {
            Some(time_range_picker)
        } else {
            None
        },
        refresh_button,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    container(row![
        Space::new().width(Length::Fixed(2.0)),
        container(filter_row)
            .width(Length::Fill)
            .height(Length::Fixed(50.0))
            .padding(8)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(theme_colors.light_bg)),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                shadow: shadows::FILTER_BAR_SHADOW,
                ..Default::default()
            })
    ])
    .into()
}
