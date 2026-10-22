// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wp.upx8 壁纸 API 图源
//!
//! 站点提供免密钥 JSON 接口（双数据源自动切换，birdpaper 主源 + 必应兜底），
//! 支持关键词、分类与分辨率三档筛选；无分页、无排序、无比例/颜色维度，
//! 这些筛选参数由 service 层忽略。接口限流 60 次/分钟/IP

mod client;
mod service;

pub use service::Upx8Service;
