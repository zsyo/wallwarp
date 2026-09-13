// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Wallhaven API 客户端模块
//!
//! 提供 Wallhaven API 访问功能，作为 [`crate::services::source`] 图源抽象的
//! 第一个实现，包括：
//! - 数据模型（models）：Category, Purity, Resolution, Ratio 及上移筛选模型
//!   Sorting/ColorOption/TimeRange 的兼容 re-export
//! - API 类型（types）：API 响应数据结构
//! - HTTP 客户端（client）：HTTP 请求处理和重试逻辑
//! - 服务层（service）：Wallhaven API 服务接口

pub mod client;
pub mod helper;
pub mod model;
pub mod service;
pub mod types;

// 重新导出常用类型
pub use crate::services::source::OnlineWallpaper;
pub use client::SearchParams;
pub use helper::*;
pub use model::*;
pub use service::WallhavenService;
pub use types::{WallhavenResponse, WallpaperData};
