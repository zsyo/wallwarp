// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面
//!
//! 收藏的在线壁纸与本地文件统一网格展示（SQLite 持久化，重启保留），
//! 支持类型标识/分组管理/设为壁纸/下载/预览/移除

mod handler;
mod message;
mod state;
mod view;
mod widget;

pub use message::FavoritesMessage;
pub use state::{FavoriteEntry, FavoritesState};
pub use view::favorites_view;
