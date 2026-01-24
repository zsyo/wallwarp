// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::{AspectRatio, Category, ColorOption, Purity, Resolution, Sorting, TimeRange};
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::{DisplayableSorting, DisplayableTimeRange, OnlineMessage, OnlineState};
use crate::ui::style::*;
use crate::ui::widget::DiagonalLine;
use crate::utils::config::Config;
use iced::widget::{button, canvas, column, container, pick_list, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建筛选栏
pub fn create_filter_bar<'a>(i18n: &'a I18n, state: &'a OnlineState, config: &'a Config) -> Element<'a, AppMessage> {
    // 搜索框（放在最前面）
    let search_input = iced::widget::text_input(&i18n.t("online-wallpapers.search-placeholder"), &state.search_text)
        .on_input(|text| AppMessage::Online(OnlineMessage::SearchTextChanged(text)))
        .on_submit(AppMessage::Online(OnlineMessage::Search))
        .padding(6)
        .size(14)
        .width(Length::Fixed(160.0))
        .style(|_theme: &iced::Theme, _status| iced::widget::text_input::Style {
            background: iced::Background::Color(COLOR_LIGHT_BUTTON),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            icon: COLOR_LIGHT_TEXT_SUB,
            placeholder: COLOR_LIGHT_TEXT_SUB,
            value: COLOR_LIGHT_TEXT,
            selection: Color::from_rgba(0.098, 0.463, 0.824, 0.3),
        });

    let search_button = common::create_icon_button_with_size(
        "\u{F52A}",
        BUTTON_COLOR_BLUE,
        16,
        AppMessage::Online(OnlineMessage::Search),
    )
    .style(|_theme: &iced::Theme, _status| iced::widget::button::Style {
        background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
        ..iced::widget::button::text(_theme, _status)
    });

    let search_container = row![search_input, search_button].spacing(2).align_y(Alignment::Center);

    // 分辨率选择器 - 使用 DropDown 组件
    let resolution_picker = create_resolution_picker(i18n, state);

    // 比例选择器 - 使用 DropDown 组件（支持多选）
    let ratio_picker = create_ratio_picker(i18n, state);

    // 颜色选择器 - 使用 DropDown 组件
    let color_picker = create_color_picker(i18n, state);

    let sorting_options: Vec<DisplayableSorting> = Sorting::all()
        .iter()
        .map(|s| DisplayableSorting {
            value: *s,
            display: i18n.t(s.display_name()).leak(),
        })
        .collect();
    let current_sorting = DisplayableSorting {
        value: state.sorting,
        display: i18n.t(state.sorting.display_name()).leak(),
    };

    let sorting_picker = pick_list(sorting_options.clone(), Some(current_sorting), |sort| {
        AppMessage::Online(OnlineMessage::SortingChanged(sort.value))
    })
    .text_size(14)
    .padding(6)
    .width(Length::Fixed(100.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    let time_range_options: Vec<DisplayableTimeRange> = TimeRange::all()
        .iter()
        .map(|t| DisplayableTimeRange {
            value: *t,
            display: i18n.t(t.display_name()).leak(),
        })
        .collect();
    let current_time_range = DisplayableTimeRange {
        value: state.time_range,
        display: i18n.t(state.time_range.display_name()).leak(),
    };

    let time_range_picker = pick_list(time_range_options.clone(), Some(current_time_range), |time| {
        AppMessage::Online(OnlineMessage::TimeRangeChanged(time.value))
    })
    .text_size(14)
    .padding(6)
    .width(Length::Fixed(130.0))
    .style(|_theme, _status| iced::widget::pick_list::Style {
        text_color: COLOR_LIGHT_TEXT,
        placeholder_color: COLOR_LIGHT_TEXT_SUB,
        handle_color: COLOR_LIGHT_TEXT_SUB,
        background: iced::Background::Color(COLOR_LIGHT_BUTTON),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
    });

    // 功能按钮
    let refresh_button = common::create_icon_button_with_size(
        "\u{F130}",
        BUTTON_COLOR_GREEN,
        16,
        AppMessage::Online(OnlineMessage::Refresh),
    )
    .style(|_theme, _status| iced::widget::button::Style {
        background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(4.0),
        },
        ..iced::widget::button::text(_theme, _status)
    });

    // 组合所有元素
    let filter_row = row![
        search_container,
        iced::widget::Space::new().width(2),
        // 分类按钮（选中状态为蓝色）
        button(text(i18n.t("online-wallpapers.category-general")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::General)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::General.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.category-anime")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::Anime)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::Anime.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.category-people")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::CategoryToggled(Category::People)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.categories & Category::People.bit_value()) != 0;
                let bg_color = if is_checked {
                    COLOR_SELECTED_BLUE
                } else {
                    COLOR_LIGHT_BUTTON
                };
                let text_color = if is_checked { Color::WHITE } else { COLOR_LIGHT_TEXT };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        iced::widget::Space::new().width(2),
        // 纯净度按钮（带颜色）
        button(text(i18n.t("online-wallpapers.purity-sfw")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::SFW)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::SFW.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SFW, Color::WHITE)
                } else {
                    (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        button(text(i18n.t("online-wallpapers.purity-sketchy")).size(14))
            .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::Sketchy)))
            .padding(6)
            .style(move |_theme, _status| {
                let is_checked = (state.purities & Purity::Sketchy.bit_value()) != 0;
                let (bg_color, text_color) = if is_checked {
                    (COLOR_SKETCHY, Color::BLACK)
                } else {
                    (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                };
                iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                }
            }),
        // NSFW 按钮：只在 API Key 不为空时显示
        if !config.wallhaven.api_key.is_empty() {
            Some(
                button(text(i18n.t("online-wallpapers.purity-nsfw")).size(14))
                    .on_press(AppMessage::Online(OnlineMessage::PurityToggled(Purity::NSFW)))
                    .padding(6)
                    .style(move |_theme, _status| {
                        let is_checked = (state.purities & Purity::NSFW.bit_value()) != 0;
                        let (bg_color, text_color) = if is_checked {
                            (COLOR_NSFW, Color::WHITE)
                        } else {
                            (COLOR_LIGHT_BUTTON, COLOR_LIGHT_TEXT)
                        };
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(bg_color)),
                            text_color: text_color,
                            border: iced::border::Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: iced::border::Radius::from(4.0),
                            },
                            ..iced::widget::button::text(_theme, _status)
                        }
                    }),
            )
        } else {
            None
        },
        iced::widget::Space::new().width(2),
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
        iced::widget::Space::new().width(Length::Fixed(1.0)),
        container(filter_row)
            .width(Length::Fill)
            .height(Length::Fixed(50.0))
            .padding(8)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(COLOR_LIGHT_BG)),
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..Default::default()
            })
    ])
    .into()
}

/// 创建颜色选择器
fn create_color_picker<'a>(i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
    let color_button_text = i18n.t("online-wallpapers.color-label");

    let color_button_bg = match state.color {
        ColorOption::Any => COLOR_LIGHT_BUTTON,
        ColorOption::Color660000 => COLOR_660000,
        ColorOption::Color990000 => COLOR_990000,
        ColorOption::ColorCC0000 => COLOR_CC0000,
        ColorOption::ColorCC3333 => COLOR_CC3333,
        ColorOption::ColorEA4C88 => COLOR_EA4C88,
        ColorOption::Color993399 => COLOR_993399,
        ColorOption::Color663399 => COLOR_663399,
        ColorOption::Color333399 => COLOR_333399,
        ColorOption::Color0066CC => COLOR_0066CC,
        ColorOption::Color0099CC => COLOR_0099CC,
        ColorOption::Color66CCCC => COLOR_66CCCC,
        ColorOption::Color77CC33 => COLOR_77CC33,
        ColorOption::Color669900 => COLOR_669900,
        ColorOption::Color336600 => COLOR_336600,
        ColorOption::Color666600 => COLOR_666600,
        ColorOption::Color999900 => COLOR_999900,
        ColorOption::ColorCCCC33 => COLOR_CCCC33,
        ColorOption::ColorFFFF00 => COLOR_FFFF00,
        ColorOption::ColorFFCC33 => COLOR_FFCC33,
        ColorOption::ColorFF9900 => COLOR_FF9900,
        ColorOption::ColorFF6600 => COLOR_FF6600,
        ColorOption::ColorCC6633 => COLOR_CC6633,
        ColorOption::Color996633 => COLOR_996633,
        ColorOption::Color663300 => COLOR_663300,
        ColorOption::Color000000 => COLOR_000000,
        ColorOption::Color999999 => COLOR_999999,
        ColorOption::ColorCCCCCC => COLOR_CCCCCC,
        ColorOption::ColorFFFFFF => COLOR_FFFFFF,
        ColorOption::Color424153 => COLOR_424153,
    };

    // 对于浅色背景（白色、浅灰色等），使用深色文字
    let color_button_text_color = COLOR_LIGHT_TEXT;

    // 创建颜色选择器的触发按钮（underlay）
    let color_underlay = row![
        text(color_button_text).size(14).color(color_button_text_color),
        iced::widget::Space::new().width(Length::Fill),
        container(text("⏷").color(COLOR_LIGHT_TEXT_SUB))
            .height(Length::Fill)
            .padding(iced::Padding {
                top: -2.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: -2.0,
    });

    let color_trigger = button(color_underlay)
        .padding(6)
        .width(Length::Fixed(80.0))
        .on_press(AppMessage::Online(OnlineMessage::ColorPickerExpanded))
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(color_button_bg)),
            text_color: color_button_text_color,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建颜色网格选项（overlay）
    let color_options = create_color_grid_options(i18n, state);

    // 使用 DropDown 组件
    DropDown::new(color_trigger, color_options, state.color_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::ColorPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}

/// 创建颜色网格选择器内容（5*6 网格，包含29种颜色+1个Any）
fn create_color_grid_options<'a>(_i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
    // 定义颜色网格（5行6列，共30个位置，前29个为官方颜色，第30个为Any）
    static COLOR_GRID_DATA: [(Color, ColorOption); 30] = [
        // 第1行
        (COLOR_660000, ColorOption::Color660000),
        (COLOR_990000, ColorOption::Color990000),
        (COLOR_CC0000, ColorOption::ColorCC0000),
        (COLOR_CC3333, ColorOption::ColorCC3333),
        (COLOR_EA4C88, ColorOption::ColorEA4C88),
        (COLOR_993399, ColorOption::Color993399),
        // 第2行
        (COLOR_663399, ColorOption::Color663399),
        (COLOR_333399, ColorOption::Color333399),
        (COLOR_0066CC, ColorOption::Color0066CC),
        (COLOR_0099CC, ColorOption::Color0099CC),
        (COLOR_66CCCC, ColorOption::Color66CCCC),
        (COLOR_77CC33, ColorOption::Color77CC33),
        // 第3行
        (COLOR_669900, ColorOption::Color669900),
        (COLOR_336600, ColorOption::Color336600),
        (COLOR_666600, ColorOption::Color666600),
        (COLOR_999900, ColorOption::Color999900),
        (COLOR_CCCC33, ColorOption::ColorCCCC33),
        (COLOR_FFFF00, ColorOption::ColorFFFF00),
        // 第4行
        (COLOR_FFCC33, ColorOption::ColorFFCC33),
        (COLOR_FF9900, ColorOption::ColorFF9900),
        (COLOR_FF6600, ColorOption::ColorFF6600),
        (COLOR_CC6633, ColorOption::ColorCC6633),
        (COLOR_996633, ColorOption::Color996633),
        (COLOR_663300, ColorOption::Color663300),
        // 第5行
        (COLOR_000000, ColorOption::Color000000),
        (COLOR_999999, ColorOption::Color999999),
        (COLOR_CCCCCC, ColorOption::ColorCCCCCC),
        (COLOR_FFFFFF, ColorOption::ColorFFFFFF),
        (COLOR_424153, ColorOption::Color424153),
        (COLOR_LIGHT_BUTTON, ColorOption::Any), // Any（浅灰色）
    ];

    // 创建颜色网格
    let mut grid_rows = Vec::new();
    for row in COLOR_GRID_DATA.chunks(6) {
        let mut row_items: Vec<Element<'a, AppMessage>> = Vec::new();
        for (color, color_option) in row {
            let is_selected = state.color == *color_option;
            let border_color = if is_selected {
                COLOR_PICKER_ACTIVE
            } else {
                Color::TRANSPARENT
            };

            // 对于 Any 选项，显示白色底色
            let color_button: Element<'a, AppMessage> = if *color_option == ColorOption::Any {
                // 创建定制化的红线：稍微缩进 2 像素，线宽 2.0
                let line_program = DiagonalLine {
                    color: COLOR_NO_COLOR_STROKE, // 使用略深的红色，更有质感
                    width: 2.5,
                    padding: 3.0,
                };

                button(
                    canvas(line_program)
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(28.0)),
                )
                .padding(0)
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(*color)),
                    border: iced::border::Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: iced::border::Radius::from(2.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                })
                .on_press(AppMessage::Online(OnlineMessage::ColorChanged(*color_option)))
                .into()
            } else {
                button(
                    iced::widget::Space::new()
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(28.0)),
                )
                .padding(0)
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(*color)),
                    border: iced::border::Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: iced::border::Radius::from(2.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                })
                .on_press(AppMessage::Online(OnlineMessage::ColorChanged(*color_option)))
                .into()
            };

            row_items.push(color_button);
        }
        grid_rows.push(row_items);
    }

    // 将行组合成网格
    let mut grid = column![].spacing(2);
    for row_items in grid_rows {
        let row = row(row_items).spacing(2);
        grid = grid.push(row);
    }

    // 创建颜色选择器容器
    let picker_content = container(grid)
        .padding(12)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(COLOR_PICKER_BG)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(8.0),
            },
            ..Default::default()
        });

    iced::widget::opaque(picker_content)
}

/// 创建分辨率选择器
fn create_resolution_picker<'a>(i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
    // 计算按钮显示文本和字体大小
    let button_text = match state.resolution_mode {
        crate::ui::online::ResolutionMode::All => i18n.t("online-wallpapers.resolution-label").to_string(),
        crate::ui::online::ResolutionMode::AtLeast => {
            if let Some(res) = state.atleast_resolution {
                format!(">={}", res.value())
            } else {
                i18n.t("online-wallpapers.resolution-label").to_string()
            }
        }
        crate::ui::online::ResolutionMode::Exactly => {
            if state.selected_resolutions.is_empty() {
                i18n.t("online-wallpapers.resolution-label").to_string()
            } else {
                // 找到最小尺寸的分辨率
                let min_res = state
                    .selected_resolutions
                    .iter()
                    .min_by_key(|r| r.get_pixel_count())
                    .unwrap();
                let extra_count = state.selected_resolutions.len() - 1;
                if extra_count > 0 {
                    format!("{}+{}", min_res.value(), extra_count)
                } else {
                    min_res.value().to_string()
                }
            }
        }
    };

    // 根据文本类型确定字体大小
    let is_label_text = button_text == i18n.t("online-wallpapers.resolution-label");
    let font_size = if is_label_text { 14 } else { 12 };

    // 创建触发按钮（underlay）
    let resolution_underlay = row![
        text(button_text).size(font_size),
        iced::widget::Space::new().width(Length::Fill),
        container(text("⏷").color(COLOR_LIGHT_TEXT_SUB))
            .height(Length::Fill)
            .padding(iced::Padding {
                top: -2.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: -2.0,
    });

    let resolution_trigger = button(resolution_underlay)
        .padding(6)
        .width(Length::Fixed(110.0))
        .on_press(AppMessage::Online(OnlineMessage::ResolutionPickerExpanded))
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            text_color: COLOR_LIGHT_TEXT,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建分辨率选项（overlay）
    let resolution_options = create_resolution_grid_options(i18n, state);

    // 使用 DropDown 组件
    DropDown::new(resolution_trigger, resolution_options, state.resolution_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::ResolutionPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}

/// 创建分辨率网格选择器内容
fn create_resolution_grid_options<'a>(i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
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
    let is_list_disabled = state.resolution_mode == crate::ui::online::ResolutionMode::All;

    // 创建顶部模式切换按钮（水平居中）
    let atleast_button = button(text(i18n.t("online-wallpapers.resolution-mode-atleast")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            crate::ui::online::ResolutionMode::AtLeast,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == crate::ui::online::ResolutionMode::AtLeast;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_selected { Color::WHITE } else { COLOR_LIGHT_TEXT };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            }
        });

    let exactly_button = button(text(i18n.t("online-wallpapers.resolution-mode-exactly")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            crate::ui::online::ResolutionMode::Exactly,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == crate::ui::online::ResolutionMode::Exactly;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_selected { Color::WHITE } else { COLOR_LIGHT_TEXT };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            }
        });

    let all_button = button(text(i18n.t("online-wallpapers.resolution-mode-all")).size(14))
        .padding(6)
        .on_press(AppMessage::Online(OnlineMessage::ResolutionModeChanged(
            crate::ui::online::ResolutionMode::All,
        )))
        .style(move |_theme, _status| {
            let is_selected = state.resolution_mode == crate::ui::online::ResolutionMode::All;
            let bg_color = if is_selected {
                COLOR_SELECTED_BLUE
            } else {
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_selected { Color::WHITE } else { COLOR_LIGHT_TEXT };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            }
        });

    let mode_buttons = container(row![atleast_button, exactly_button, all_button].spacing(4))
        .width(Length::Fill)
        .center_x(Length::Fill);

    // 创建分辨率表格（水平排列分组）
    let mut group_columns: Vec<Element<'a, AppMessage>> = Vec::new();

    for (group_name, resolutions) in RESOLUTION_GROUPS.iter() {
        // 创建分组标题（水平居中）
        let group_header = container(text(i18n.t(group_name)).size(14).color(COLOR_LIGHT_TEXT))
            .width(Length::Fill)
            .center_x(Length::Fill);

        // 创建分组内的分辨率按钮（每一行一个）
        let mut group_column = column![].spacing(2);
        for (resolution, label) in resolutions.iter() {
            let is_selected = if state.resolution_mode == crate::ui::online::ResolutionMode::AtLeast {
                state.atleast_resolution == Some(*resolution)
            } else if state.resolution_mode == crate::ui::online::ResolutionMode::Exactly {
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
                Color::from_rgba(0.5, 0.5, 0.5, 1.0) // 禁用状态使用灰色
            } else {
                COLOR_LIGHT_TEXT
            };

            let button_content = container(text(*label).size(13))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            let res_button: Element<'a, AppMessage> = button(button_content)
                .padding(6)
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                })
                .on_press(if is_list_disabled {
                    AppMessage::None
                } else if state.resolution_mode == crate::ui::online::ResolutionMode::AtLeast {
                    AppMessage::Online(OnlineMessage::ResolutionAtLeastSelected(*resolution))
                } else {
                    AppMessage::Online(OnlineMessage::ResolutionToggled(*resolution))
                })
                .into();

            group_column = group_column.push(res_button);
        }

        // 将分组标题和内容组合，使用固定宽度
        let group_section = container(
            column![
                group_header,
                iced::widget::Space::new().height(Length::Fixed(4.0)),
                group_column,
            ]
            .spacing(0),
        )
        .width(Length::Fixed(100.0));

        group_columns.push(group_section.into());
    }

    // 将所有分组水平排列
    let table_content = row(group_columns).spacing(2);

    // 创建分辨率选择器容器
    let picker_content = container(
        column![
            mode_buttons,
            iced::widget::Space::new().height(Length::Fixed(12.0)),
            table_content,
        ]
        .spacing(0)
        .align_x(Alignment::Center),
    )
    .padding(12)
    .width(Length::Fixed(530.0))
    .align_x(Alignment::Center)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_PICKER_BG)),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(8.0),
        },
        ..Default::default()
    });

    iced::widget::opaque(picker_content)
}

/// 创建比例选择器
fn create_ratio_picker<'a>(i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
    // 计算按钮显示文本和字体大小
    let button_text = if state.ratio_all_selected {
        i18n.t("online-wallpapers.ratio-label").to_string()
    } else {
        // 计算总选中数量
        let extra_count = state.selected_ratios.len() as i32;
        let landscape_count = if state.ratio_landscape_selected { 1 } else { 0 };
        let portrait_count = if state.ratio_portrait_selected { 1 } else { 0 };
        let total_count = extra_count + landscape_count + portrait_count;

        if total_count == 0 {
            i18n.t("online-wallpapers.ratio-label").to_string()
        } else {
            // 确定第一项的显示文本
            let first_item = if state.ratio_landscape_selected {
                i18n.t("online-wallpapers.ratio-mode-landscape").to_string()
            } else if state.ratio_portrait_selected {
                i18n.t("online-wallpapers.ratio-mode-portrait").to_string()
            } else if let Some(first_ratio) = state.selected_ratios.first() {
                first_ratio.value().to_string()
            } else {
                String::new()
            };

            // 计算额外数量（总数减1）
            let extra_display = total_count - 1;
            if extra_display > 0 {
                format!("{}+{}", first_item, extra_display)
            } else {
                first_item
            }
        }
    };

    // 根据文本类型确定字体大小
    let is_label_text = button_text == i18n.t("online-wallpapers.ratio-label");
    let font_size = if is_label_text { 14 } else { 12 };

    // 创建触发按钮（underlay）
    let ratio_underlay = row![
        text(button_text).size(font_size),
        iced::widget::Space::new().width(Length::Fill),
        container(text("⏷").color(COLOR_LIGHT_TEXT_SUB))
            .height(Length::Fill)
            .padding(iced::Padding {
                top: -2.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: -2.0,
    });

    let ratio_trigger = button(ratio_underlay)
        .padding(6)
        .width(Length::Fixed(120.0))
        .on_press(AppMessage::Online(OnlineMessage::RatioPickerExpanded))
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            text_color: COLOR_LIGHT_TEXT,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建比例选项（overlay）
    let ratio_options = create_ratio_grid_options(i18n, state);

    // 使用 DropDown 组件
    DropDown::new(ratio_trigger, ratio_options, state.ratio_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::RatioPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}

/// 创建比例网格选择器内容
fn create_ratio_grid_options<'a>(i18n: &'a I18n, state: &'a OnlineState) -> Element<'a, AppMessage> {
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
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_landscape_button_disabled {
                Color::from_rgba(0.5, 0.5, 0.5, 1.0) // 禁用状态使用灰色
            } else if is_selected {
                Color::WHITE
            } else {
                COLOR_LIGHT_TEXT
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
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
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_portrait_button_disabled {
                Color::from_rgba(0.5, 0.5, 0.5, 1.0) // 禁用状态使用灰色
            } else if is_selected {
                Color::WHITE
            } else {
                COLOR_LIGHT_TEXT
            };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
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
                COLOR_LIGHT_BUTTON
            };
            let text_color = if is_selected { Color::WHITE } else { COLOR_LIGHT_TEXT };
            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: text_color,
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
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
        let group_header = container(text(i18n.t(group_name)).size(14).color(COLOR_LIGHT_TEXT))
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
                Color::from_rgba(0.5, 0.5, 0.5, 1.0) // 禁用状态使用灰色
            } else {
                COLOR_LIGHT_TEXT
            };

            let button_content = container(text(*label).size(13))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .width(Length::Fill);

            let ratio_button: Element<'a, AppMessage> = button(button_content)
                .padding(6)
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: text_color,
                    border: iced::border::Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
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
        let group_section = container(
            column![
                group_header,
                iced::widget::Space::new().height(Length::Fixed(4.0)),
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
            iced::widget::Space::new().height(Length::Fixed(12.0)),
            table_content,
        ]
        .spacing(0)
        .align_x(Alignment::Center),
    )
    .padding(6)
    .width(Length::Fixed(460.0))
    .align_x(Alignment::Center)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_PICKER_BG)),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(8.0),
        },
        ..Default::default()
    });

    iced::widget::opaque(picker_content)
}
