// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! Pixabay 图源
//!
//! 经 Pixabay 官方 API（https://pixabay.com/api/）检索图片，key 免费注册，
//! 限流 100 次/分钟。支持关键词 + 方向 + 排序 + 内容分类（category）+
//! 颜色（colors）+ 精选（editors_choice）筛选，无对应能力的维度
//! （时间范围/具体比例值）由 service 层忽略

mod client;
mod service;

pub use service::PixabayService;
