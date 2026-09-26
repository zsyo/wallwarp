// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Danbooru 图源
//!
//! 经 Danbooru 公开 JSON API（https://danbooru.donmai.us/posts.json）检索
//! 图片，免密钥。接口限流读请求 10 次/秒（建议约 1 次/秒），UA 标识由
//! 统一 HTTP 客户端提供。仅支持 tags 关键词 + 排序 metatag 的检索，
//! rating 分级映射为统一 purity 参数

mod client;
mod service;

pub use service::DanbooruService;
