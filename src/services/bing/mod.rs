// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Bing 每日一图图源
//!
//! 经 Bing 官方 HPImageArchive 接口获取最近的每日精选壁纸，免密钥、
//! 国内可达。无搜索/筛选维度，`search()` 忽略筛选参数，按页码翻阅历史图集

mod client;
mod service;

pub use service::BingService;
