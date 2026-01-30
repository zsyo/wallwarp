// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::NotificationType;
use crate::utils::config::Config;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// 主窗口相关状态
#[derive(Debug, Clone)]
pub struct MainState {
    // 窗口状态
    pub show_close_confirmation: bool,
    pub remember_close_setting: bool,
    pub is_visible: bool,
    pub is_maximized: bool,
    pub pending_window_size: Option<(u32, u32)>,
    pub debounce_timer: std::time::Instant,

    // 响应式布局
    pub current_window_width: u32,
    pub current_window_height: u32,

    // 通知系统
    pub show_notification: bool,
    pub notification_message: String,
    pub notification_type: NotificationType,

    // 其他
    pub initial_loaded: bool,
    pub auto_change_running: Arc<AtomicBool>,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            show_close_confirmation: false,
            remember_close_setting: false,
            is_visible: false,
            is_maximized: false,
            pending_window_size: None,
            debounce_timer: std::time::Instant::now(),
            current_window_width: 1280,
            current_window_height: 800,
            show_notification: false,
            notification_message: String::new(),
            notification_type: NotificationType::Success,
            initial_loaded: false,
            auto_change_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl MainState {
    /// 创建主窗口状态
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            current_window_width: window_width,
            current_window_height: window_height,
            // 更新托盘菜单项的启用状态在调用方处理
            ..Default::default()
        }
    }
    pub fn load_from_config(config: &Config) -> Self {
        Self {
            current_window_width: config.display.width,
            current_window_height: config.display.height,
            ..Default::default()
        }
    }
}
