// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 通用工具函数

use iced::widget::image::Handle;

/// 判断缓存图片句柄指向的文件是否已丢失
///
/// 文件句柄按路径检查存在性；内存字节/像素句柄直接视为有效。
/// 用于缓存目录被清空（应用内清空或外部删除）后触发缩略图重载。
pub fn cached_image_missing(handle: &Handle) -> bool {
    match handle {
        Handle::Path(_, path) => !path.exists(),
        _ => false,
    }
}

mod bordered_container;
mod card_style;
mod checkbox;
mod colored_button;
mod confirmation_dialog;
pub mod icon_button;
mod input_style;
pub mod modal_overlay;
pub mod preview_modal;
pub mod progress_ring;
pub mod rotated_icon;
mod tooltip_button;
mod tooltip_radio;
mod wallpaper_card;
mod wallpaper_grid;

pub mod drop_down;

pub use bordered_container::*;
pub use card_style::*;
pub use checkbox::*;
pub use colored_button::*;
pub use confirmation_dialog::*;
pub use icon_button::*;
pub use input_style::*;
pub use modal_overlay::*;
pub use preview_modal::*;
pub use progress_ring::*;
pub use rotated_icon::*;
pub use tooltip_button::*;
pub use tooltip_radio::*;
pub use wallpaper_card::*;
pub use wallpaper_grid::*;
