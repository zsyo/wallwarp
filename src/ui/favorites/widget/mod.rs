// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面组件：顶部工具条 / 收藏卡片 / 类型徽章

mod card;
mod toolbar;

pub use card::{create_favorite_card, create_favorite_modal_info};
pub use toolbar::create_favorites_toolbar;
