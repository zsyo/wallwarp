// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::AppMessage;
use crate::ui::common::create_colored_button_with_text;
use iced::widget::{row, text};
use iced::{Alignment, Element};

/// 创建批量操作按钮（统一彩色按钮样式，含悬停/按下/禁用态）
pub fn create_batch_button(
    label: String,
    icon: &'static str,
    enabled: bool,
    message: AppMessage,
    button_color: iced::Color,
) -> Element<'static, AppMessage> {
    let button_content = row![
        text(icon)
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(14),
        text(label).size(13),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    create_colored_button_with_text(button_content.into(), button_color, message.clone())
        .on_press_maybe(enabled.then_some(message))
        .padding([6, 12])
        .into()
}
