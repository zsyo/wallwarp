// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 颜色工具函数
//!
//! 提供按钮/控件交互态所需的颜色变体计算（调暗、调亮、调整透明度）。

use iced::Color;

/// 向黑色方向调整颜色（amount: 0.0~1.0）
#[inline]
pub fn darken(color: Color, amount: f32) -> Color {
    Color {
        r: color.r * (1.0 - amount),
        g: color.g * (1.0 - amount),
        b: color.b * (1.0 - amount),
        a: color.a,
    }
}

/// 向白色方向调整颜色（amount: 0.0~1.0）
#[inline]
pub fn lighten(color: Color, amount: f32) -> Color {
    Color {
        r: color.r + (1.0 - color.r) * amount,
        g: color.g + (1.0 - color.g) * amount,
        b: color.b + (1.0 - color.b) * amount,
        a: color.a,
    }
}

/// 调整颜色的透明度
#[inline]
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

/// 计算带透明度的颜色淡染（用于选中底色等强调色浅底）
#[inline]
pub fn tint(color: Color, alpha: f32) -> Color {
    with_alpha(color, alpha)
}
