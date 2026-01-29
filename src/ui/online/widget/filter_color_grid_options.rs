// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::ColorOption;
use crate::ui::AppMessage;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{button, canvas, column, container, opaque, row};
use iced::{Color, Element, Length};

/// 创建颜色网格选择器内容（5*6 网格，包含29种颜色+1个Any）
pub fn create_color_grid_options<'a>(
    _i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
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
                let line_program = super::DiagonalLine {
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
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(*color)),
                    border: Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: Radius::from(2.0),
                    },
                    ..button::text(_theme, _status)
                })
                .on_press(OnlineMessage::ColorChanged(*color_option).into())
                .into()
            } else {
                button(
                    iced::widget::Space::new()
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(28.0)),
                )
                .padding(0)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(*color)),
                    border: Border {
                        color: border_color,
                        width: if is_selected { 2.0 } else { 0.0 },
                        radius: Radius::from(2.0),
                    },
                    ..button::text(_theme, _status)
                })
                .on_press(OnlineMessage::ColorChanged(*color_option).into())
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
