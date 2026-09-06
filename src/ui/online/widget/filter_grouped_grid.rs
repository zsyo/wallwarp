// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线筛选的分组网格选择器骨架（比例/分辨率选择器共用）
//!
//! 结构：顶部模式按钮行 + 分组表格（组标题 + 组内单元格）+ 面板容器

use crate::ui::common::drop_down::{dropdown_cell_style, dropdown_panel_style};
use crate::ui::style::ThemeColors;
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Element, Length};

/// 分组网格选择器内容
///
/// - `mode_buttons`: 顶部模式/附加选项按钮（由调用方按各自选中/禁用逻辑构建）；
/// - `groups`: (分组标题文本, 组内单元格) 列表，每组固定宽度竖排；
/// - `panel_width`: 面板宽度；
/// - `panel_padding`: 面板内边距。
pub fn grouped_grid_options<'a, Message: Clone + 'a>(
    mode_buttons: Vec<Element<'a, Message>>,
    groups: Vec<(String, Vec<Element<'a, Message>>)>,
    panel_width: f32,
    panel_padding: f32,
    theme_colors: ThemeColors,
) -> Element<'a, Message> {
    let mode_row = container(row(mode_buttons).spacing(4))
        .width(Length::Fill)
        .center_x(Length::Fill);

    // 水平排列分组（组标题居中，组内单元格竖排）
    let mut group_columns: Vec<Element<'a, Message>> = Vec::new();
    for (group_title, cells) in groups {
        let group_header = container(text(group_title).size(14).color(theme_colors.light_text))
            .width(Length::Fill)
            .center_x(Length::Fill);

        let group_section = container(
            column![
                group_header,
                Space::new().height(Length::Fixed(4.0)),
                column(cells).spacing(2),
            ]
            .spacing(0),
        )
        .width(Length::Fixed(100.0));

        group_columns.push(group_section.into());
    }

    let picker_content = container(
        column![
            mode_row,
            Space::new().height(Length::Fixed(12.0)),
            row(group_columns).spacing(2),
        ]
        .spacing(0)
        .align_x(Alignment::Center),
    )
    .padding(panel_padding)
    .width(Length::Fixed(panel_width))
    .align_x(Alignment::Center)
    .style(dropdown_panel_style(theme_colors));

    opaque(picker_content)
}

/// 分组网格单元格按钮（禁用时显示灰色样式且无按压响应）
///
/// - `on_press`: 选中消息；`is_disabled` 为 true 时不响应按压
pub fn grid_cell_button<'a, Message: Clone + 'a>(
    label: &str,
    is_selected: bool,
    is_disabled: bool,
    on_press: Option<Message>,
    theme_colors: ThemeColors,
) -> Element<'a, Message> {
    let button_content = container(text(label.to_string()).size(13))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    button(button_content)
        .padding(6)
        .style(dropdown_cell_style(theme_colors, is_selected, is_disabled))
        .on_press_maybe(if is_disabled { None } else { on_press })
        .into()
}
