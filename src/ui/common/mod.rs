// Copyright (C) 2026 zsyo - GNU GPL v3.0

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

/// 各页面缩略图检查入口的公共状态接口（在线/本地/收藏夹/历史四页 check_thumbs 共用）
///
/// 各页面 State 均持有"缓存失效标志 + 预览模态"两组状态，
/// 进入页面时的公共处理为：读取并清零失效标志、预览缓存丢失时关闭模态
pub trait ThumbCheckState {
    /// 读取并清零"缩略图缓存已失效"标志
    fn take_thumbs_stale(&mut self) -> bool;
    /// 预览模态是否可见
    fn modal_visible(&self) -> bool;
    /// 预览模态当前展示的图片句柄
    fn modal_handle(&self) -> Option<&Handle>;
    /// 关闭预览模态（含各页面在途下载的取消）
    fn close_modal(&mut self);
}

/// 各页面 check_thumbs 的公共骨架
///
/// 返回 force_reload（失效标志），由调用方决定是否对全部已加载项重载
pub fn begin_thumb_check(state: &mut impl ThumbCheckState) -> bool {
    let force_reload = state.take_thumbs_stale();
    // 预览原图缓存丢失时直接关闭模态（重新打开即可重新加载）
    if state.modal_visible() && state.modal_handle().is_some_and(cached_image_missing) {
        state.close_modal();
    }
    force_reload
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
