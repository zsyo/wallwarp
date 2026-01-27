// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Wallhaven API 客户端模块
//!
//! 提供完整的 Wallhaven API 访问功能，包括：
//! - 数据模型（models）：Category, Sorting, Purity, Resolution, Ratio, ColorOption, TimeRange
//! - API 类型（types）：API 响应数据结构
//! - HTTP 客户端（client）：HTTP 请求处理和重试逻辑
//! - 服务层（service）：Wallhaven API 服务接口

pub mod client;
pub mod helper;
pub mod models;
pub mod service;
pub mod types;

// 重新导出常用类型
pub use helper::generate_file_name;
pub use models::{AspectRatio, AspectRatioGroup, Category, ColorOption, Purity, Ratio, Resolution, Sorting, TimeRange};
pub use service::WallhavenService;
pub use types::{OnlineWallpaper, WallpaperData};
