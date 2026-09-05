// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸历史页面
//!
//! 浏览最近应用过的壁纸（SQLite 持久化，重启保留），
//! 列表式展示，支持重新应用/预览/定位/复制路径/移除/清空

mod handler;
mod message;
mod state;
mod view;
mod widget;

pub use message::HistoryMessage;
pub use state::{HistoryEntry, HistoryState};
pub use view::history_view;
