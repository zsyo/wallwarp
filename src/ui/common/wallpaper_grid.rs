// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸网格布局估算：本地页与在线页共用的响应式分块公式

use crate::ui::style::{IMAGE_HEIGHT, IMAGE_SPACING, IMAGE_WIDTH};

/// 计算指定窗口宽度下每行可容纳的壁纸卡片数
pub fn grid_items_per_row(window_width: u32) -> usize {
    let available_width = (window_width as f32 - IMAGE_SPACING).max(IMAGE_WIDTH);
    let unit_width = IMAGE_WIDTH + IMAGE_SPACING;
    ((available_width / unit_width).floor() as usize).max(1)
}

/// 估算网格内容的总高度（行数 × 单元格高度），用于判断是否需要自动加载下一页
pub fn grid_estimated_height(item_count: usize, window_width: u32) -> f32 {
    let items_per_row = grid_items_per_row(window_width);
    let num_rows = item_count.div_ceil(items_per_row);
    num_rows as f32 * (IMAGE_HEIGHT + IMAGE_SPACING)
}
