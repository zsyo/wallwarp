// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! Pixabay 图源
//!
//! 经 Pixabay 官方 API（https://pixabay.com/api/）检索图片，key 免费注册，
//! 限流 100 次/分钟。仅支持关键词 + 方向 + 排序的轻量筛选，其余筛选维度
//! 由 service 层忽略

mod client;
mod service;

pub use service::PixabayService;
