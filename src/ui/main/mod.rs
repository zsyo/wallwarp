// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod close_confirm;
mod handler;
mod message;
mod tray;
mod view;
mod widget;

pub use close_confirm::close_confirm_view;
pub use message::MainMessage;
pub use tray::TrayManager;
pub use view::main_view;
