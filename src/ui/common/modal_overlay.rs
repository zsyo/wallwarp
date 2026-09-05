// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 模态浮层（半透明胶囊）公共样式

use iced::widget::container;

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
