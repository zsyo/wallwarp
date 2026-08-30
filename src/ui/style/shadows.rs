// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 阴影样式定义
//!
//! 所有UI相关的阴影样式常量应在此文件中定义。
//! 阴影统一偏柔和低透明度，营造悬浮层次感而非生硬投影。

use iced::{Color, Shadow, Vector};

// ============================================================================
// 卡片阴影
// ============================================================================

/// 卡片默认阴影（柔和、贴近表面）
pub const CARD_SHADOW: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
    offset: Vector { x: 0.0, y: 2.0 },
    blur_radius: 8.0,
};

/// 卡片悬停阴影（抬升感）
pub const CARD_SHADOW_HOVER: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
    offset: Vector { x: 0.0, y: 6.0 },
    blur_radius: 16.0,
};

// ============================================================================
// 对话框阴影
// ============================================================================

/// 对话框/下拉面板阴影（远层悬浮）
pub const DIALOG_SHADOW: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
    offset: Vector { x: 0.0, y: 8.0 },
    blur_radius: 24.0,
};

// ============================================================================
// 筛选栏阴影
// ============================================================================

/// 筛选栏阴影（只显示在下边）
pub const FILTER_BAR_SHADOW: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
    offset: Vector { x: 0.0, y: 4.0 },
    blur_radius: 6.0,
};

/// 获取卡片默认阴影
#[inline]
pub fn get_card_shadow() -> Shadow {
    CARD_SHADOW
}

/// 获取卡片悬停阴影
#[inline]
pub fn get_card_shadow_hover() -> Shadow {
    CARD_SHADOW_HOVER
}

/// 根据状态获取卡片阴影
#[inline]
pub fn get_card_shadow_by_status(is_hovered: bool) -> Shadow {
    if is_hovered {
        CARD_SHADOW_HOVER
    } else {
        CARD_SHADOW
    }
}
