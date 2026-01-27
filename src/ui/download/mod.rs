// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 下载管理页面模块
//!
//! 提供下载任务管理界面，支持查看下载进度、管理下载任务等功能。

mod handler;
mod message;
mod state;
mod view;
mod widget;

pub use message::*;
pub use state::*;
pub use view::download_view;
