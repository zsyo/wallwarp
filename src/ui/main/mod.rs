// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod close_confirm;
mod floating_ball;
mod handler;
mod main_window;
mod menu_defs;
mod message;
mod state;
mod tray;
mod view;
mod widget;

pub use close_confirm::close_confirm_view;
pub use floating_ball::{
    FloatingBallManager, FloatingBallState, SnapEdge, SnapState, floating_ball_view,
    window_settings,
};
pub use main_window::main_window_settings;
pub use message::MainMessage;
pub use state::MainState;
pub use tray::TrayManager;
pub use view::main_view;
