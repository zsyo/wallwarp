// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 模态浮层（半透明胶囊）公共样式与信息浮层组件

use crate::ui::style::COLOR_OVERLAY_TEXT;
use iced::widget::{container, row, text};
use iced::{Alignment, Element};

/// 模态浮层通用样式：黑色 65% 半透明圆角胶囊
///
/// 叠加在预览图片之上使用，不随主题变化以保证对比度
pub fn modal_overlay_style(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(iced::Background::Color(iced::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.65,
        })),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(crate::ui::style::RADIUS_MD),
        },
        ..Default::default()
    }
}

/// 信息浮层的单行内容（`标签: 值`），统一使用遮罩文字色
pub fn modal_info_row<'a, Message: 'a>(label: &str, value: String) -> Element<'a, Message> {
    row![
        text(format!("{label}: "))
            .size(12)
            .color(COLOR_OVERLAY_TEXT),
        text(value).size(12).color(COLOR_OVERLAY_TEXT),
    ]
    .spacing(2)
    .align_y(Alignment::Center)
    .into()
}

/// 信息浮层胶囊容器（与底部工具栏同款半透明胶囊底色）
pub fn modal_info_pill<'a, Message: 'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    container(content)
        .padding([8, 12])
        .style(modal_overlay_style)
        .into()
}
