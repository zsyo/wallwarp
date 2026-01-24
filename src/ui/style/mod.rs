// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! UI样式模块
//!
//! 此模块包含所有UI相关的样式常量和主题配置。
//! 所有样式常量按类型拆分到不同的子模块中，便于维护和主题切换。

// 重新导出所有子模块
pub mod colors;
pub mod dimensions;
pub mod shadows;
pub mod theme;

// 重新导出颜色常量
pub use colors::*;

// 重新导出尺寸常量
pub use dimensions::*;

// 重新导出阴影样式
pub use shadows::*;

// 重新导出主题相关类型
pub use theme::*;