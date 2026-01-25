// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 阴影样式定义
//!
//! 所有UI相关的阴影样式常量应在此文件中定义。

use iced::{Color, Shadow, Vector};

// ============================================================================
// 卡片阴影
// ============================================================================

/// 卡片默认阴影
pub const CARD_SHADOW: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
    offset: Vector { x: 0.0, y: 2.0 },
    blur_radius: 8.0,
};

/// 卡片悬停阴影
pub const CARD_SHADOW_HOVER: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
    offset: Vector { x: 0.0, y: 4.0 },
    blur_radius: 12.0,
};

// ============================================================================
// 筛选栏阴影
// ============================================================================

/// 筛选栏阴影（更深，更明显）
pub const FILTER_BAR_SHADOW: Shadow = Shadow {
    color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),  // 黑色，20% 透明度（更深）
    offset: Vector { x: 0.0, y: 3.0 },              // 向下偏移 3px（更大）
    blur_radius: 10.0,                               // 模糊半径 10px（更大）
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