// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Unsplash 图源
//!
//! 经 Unsplash 官方 API（https://api.unsplash.com）检索摄影图片，key 免费
//! 注册（新注册 demo key 限流 50 次/小时）。支持关键词 + 方向 + 排序 +
//! 颜色筛选，其余筛选维度由 service 层忽略

mod client;
mod service;

pub use service::UnsplashService;
