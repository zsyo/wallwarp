use crate::i18n::I18n;
use crate::services::wallhaven::{Category, ColorOption, Purity, Ratio, Resolution, Sorting, TimeRange};
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::online::{DisplayableRatio, DisplayableResolution, DisplayableSorting, DisplayableTimeRange, OnlineMessage, OnlineState};
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

    let search_button = common::create_icon_button_with_size("\u{F52A}", BUTTON_COLOR_BLUE, 16, AppMessage::Online(OnlineMessage::Search)).style(
        |_theme: &iced::Theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        },
    );

    let search_container = row![search_input, search_button].spacing(2).align_y(Alignment::Center);

    // 下拉筛选器 - 使用包装类型以支持 i18n
    let resolution_options: Vec<DisplayableResolution> = Resolution::all()
        .iter()
        .map(|r| DisplayableResolution {
            value: *r,
            display: i18n.t(r.display_name()).leak(),
        })
        .collect();
    let current_resolution = DisplayableResolution {
        value: state.resolution,
        display: i18n.t(state.resolution.display_name()).leak(),
    };

    let resolution_picker = pick_list(resolution_options.clone(), Some(current_resolution), |res| {
        AppMessage::Online(OnlineMessage::ResolutionChanged(res.value))
    })
    .padding(6)
    .width(Length::Fixed(80.0))
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

    let ratio_options: Vec<DisplayableRatio> = Ratio::all()
        .iter()
        .map(|r| DisplayableRatio {
            value: *r,
            display: i18n.t(r.display_name()).leak(),
        })
        .collect();
    let current_ratio = DisplayableRatio {
        value: state.ratio,
        display: i18n.t(state.ratio.display_name()).leak(),
    };

    let ratio_picker = pick_list(ratio_options.clone(), Some(current_ratio), |rat| {
        AppMessage::Online(OnlineMessage::RatioChanged(rat.value))
    })
    .padding(6)
    .width(Length::Fixed(80.0))
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
    let refresh_button =
        common::create_icon_button_with_size("\u{F130}", BUTTON_COLOR_GREEN, 16, AppMessage::Online(OnlineMessage::Refresh)).style(|_theme, _status| {
            iced::widget::button::Style {
                background: Some(iced::Background::Color(COLOR_LIGHT_BUTTON)),
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            }
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
                let bg_color = if is_checked { COLOR_SELECTED_BLUE } else { COLOR_LIGHT_BUTTON };
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
                let bg_color = if is_checked { COLOR_SELECTED_BLUE } else { COLOR_LIGHT_BUTTON };
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
                let bg_color = if is_checked { COLOR_SELECTED_BLUE } else { COLOR_LIGHT_BUTTON };
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
        if state.sorting == Sorting::TopList { Some(time_range_picker) } else { None },
        refresh_button,
    ]
    .spacing(4)
    .align_y(Alignment::Center);

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
        text(color_button_text).color(color_button_text_color),
        iced::widget::Space::new().width(Length::Fill),
        container(text("⏷").color(COLOR_LIGHT_TEXT_SUB)).height(Length::Fill).padding(iced::Padding {
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
            let border_color = if is_selected { COLOR_PICKER_ACTIVE } else { Color::TRANSPARENT };

            // 对于 Any 选项，显示白色底色
            let color_button: Element<'a, AppMessage> = if *color_option == ColorOption::Any {
                // 创建定制化的红线：稍微缩进 2 像素，线宽 2.0
                let line_program = DiagonalLine {
                    color: COLOR_NO_COLOR_STROKE, // 使用略深的红色，更有质感
                    width: 2.5,
                    padding: 3.0,
                };

                button(canvas(line_program).width(Length::Fixed(64.0)).height(Length::Fixed(28.0)))
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
                button(iced::widget::Space::new().width(Length::Fixed(64.0)).height(Length::Fixed(28.0)))
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
    let picker_content = container(grid).padding(12).style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_PICKER_BG)),
        border: iced::border::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(8.0),
        },
        ..Default::default()
    });

    picker_content.into()
}
