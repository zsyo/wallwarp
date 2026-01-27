// Copyright (C) 2026 zsyo - GNU AGPL v3.0

pub mod handlers;
pub mod message;
pub mod state;
pub mod view;

pub use message::{LocalMessage, WallpaperLoadStatus};
pub use state::LocalState;
pub use view::local_view;