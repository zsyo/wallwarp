// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{ROW_SPACING, SETTING_ROW_DESC_SIZE, SETTING_ROW_PADDING_Y, ThemeColors};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建设置行
///
/// 左列为标签（14px 主文字色）与可选说明文字（12px 次要色），
/// 右列为控件，垂直居中；行高随控件自适应。
///
/// # 参数
/// - `label`: 标签文本
/// - `description`: 可选说明文本（展示在标签下方）
/// - `widget`: 控件
/// - `theme_colors`: 主题颜色
pub fn create_setting_row<'a, Message: 'a>(
    label: String,
    description: Option<String>,
    widget: impl Into<Element<'a, Message>>,
    theme_colors: ThemeColors,
) -> Element<'a, Message> {
    let mut label_column = column![text(label).color(theme_colors.text)].spacing(2);

    if let Some(description) = description {
        label_column = label_column.push(
            text(description)
                .size(SETTING_ROW_DESC_SIZE)
                .color(theme_colors.light_text_sub),
        );
    }

    row![
        label_column.width(Length::FillPortion(2)),
        container(widget.into())
            .width(Length::FillPortion(3))
            .align_x(Alignment::End)
            .align_y(Alignment::Center),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}

/// 创建整行控件行（控件占满行宽，无右列）
///
/// 用于无法塞进右列的复杂控件组合（如路径行的输入框+按钮组）。
///
/// # 参数
/// - `label`: 标签文本
/// - `description`: 可选说明文本
/// - `widget`: 占满整行的控件
/// - `theme_colors`: 主题颜色
pub fn create_full_width_row<'a, Message: 'a>(
    label: String,
    description: Option<String>,
    widget: impl Into<Element<'a, Message>>,
    theme_colors: ThemeColors,
) -> Element<'a, Message> {
    let mut label_column = column![text(label).color(theme_colors.text)].spacing(2);

    if let Some(description) = description {
        label_column = label_column.push(
            text(description)
                .size(SETTING_ROW_DESC_SIZE)
                .color(theme_colors.light_text_sub),
        );
    }

    let wrapped = container(widget.into())
        .width(Length::Fill)
        .padding([SETTING_ROW_PADDING_Y, 0.0]);

    column![label_column, wrapped]
        .spacing(4)
        .width(Length::Fill)
        .into()
}
