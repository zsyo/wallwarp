// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod handler;
mod menu;
mod message;
mod state;
mod types;
mod view;
mod widget;

pub use message::*;
pub use state::SettingsState;
pub use types::*;
pub use view::settings_view;

pub(in crate::ui) use menu::create_category_submenu;
