// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! wallpaperscraft 图源
//!
//! 站点为服务端渲染的 HTML 列表页，无公开 JSON API，故按页面结构正则提取
//! 壁纸条目。免密钥，支持关键词（站点 tag）、分类、分辨率、排序与分页；
//! 不支持比例/颜色/时间范围/纯净度维度，这些筛选参数由 service 层忽略

mod client;
mod service;

pub use service::WallpaperscraftService;
