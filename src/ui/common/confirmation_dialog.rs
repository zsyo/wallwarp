// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{
    BORDER_COLOR_GRAY, BUTTON_COLOR_GRAY, BUTTON_COLOR_RED, DIALOG_BORDER_RADIUS, DIALOG_BORDER_WIDTH,
    DIALOG_BUTTON_SPACING, DIALOG_INNER_PADDING, DIALOG_MAX_WIDTH, DIALOG_MESSAGE_SIZE, DIALOG_PADDING, DIALOG_SPACING,
    DIALOG_TITLE_SIZE, MASK_ALPHA,
};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建模态确认对话框
///
/// # 参数
/// - `title`: 对话框标题
/// - `message`: 对话框提示信息
/// - `confirm_label`: 确认按钮文本
/// - `cancel_label`: 取消按钮文本
/// - `confirm_msg`: 确认按钮消息
/// - `cancel_msg`: 取消按钮消息
pub fn create_confirmation_dialog<'a, Message>(
    title: String,
    message: String,
    confirm_label: String,
    cancel_label: String,
    confirm_msg: Message,
    cancel_msg: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let dialog_content = column![
        text(title)
            .size(DIALOG_TITLE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(message)
            .size(DIALOG_MESSAGE_SIZE)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        row![
            super::create_colored_button(confirm_label, BUTTON_COLOR_RED, confirm_msg),
            super::create_colored_button(cancel_label, BUTTON_COLOR_GRAY, cancel_msg),
        ]
        .spacing(DIALOG_BUTTON_SPACING)
        .align_y(Alignment::Center),
    ]
    .padding(DIALOG_PADDING)
    .spacing(DIALOG_SPACING)
    .align_x(Alignment::Center)
    .width(Length::Shrink)
    .max_width(DIALOG_MAX_WIDTH);

    let modal_dialog = container(dialog_content)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .padding(DIALOG_INNER_PADDING)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(DIALOG_BORDER_RADIUS),
                width: DIALOG_BORDER_WIDTH,
                color: iced::Color::from_rgb(BORDER_COLOR_GRAY, BORDER_COLOR_GRAY, BORDER_COLOR_GRAY),
            },
            ..Default::default()
        });

    let modal_content = container(iced::widget::stack(vec![
        container(iced::widget::Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: MASK_ALPHA,
                })),
                ..Default::default()
            })
            .into(),
        container(modal_dialog)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into(),
    ]))
    .width(Length::Fill)
    .height(Length::Fill);

    iced::widget::opaque(modal_content).into()
}
