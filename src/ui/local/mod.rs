// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸管理页面模块
//!
//! 提供本地壁纸管理界面，支持查看、预览、删除和设置壁纸等功能。

pub mod handler;
pub mod message;
pub mod state;
pub mod view;
pub mod widget;

pub use message::{LocalMessage, WallpaperLoadStatus};
pub use state::LocalState;
pub use view::local_view;