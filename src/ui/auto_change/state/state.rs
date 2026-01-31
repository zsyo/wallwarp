// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// 定时切换壁纸相关状态
#[derive(Debug)]
pub struct AutoChangeState {
    /// 是否启用定时切换
    pub auto_change_enabled: bool,
    /// 下次执行时间
    pub next_execute_time: Option<chrono::DateTime<chrono::Local>>,
    /// 上次执行时间
    pub last_executed_time: Option<chrono::DateTime<chrono::Local>>,
    /// 是否自动检测颜色模式
    pub auto_detect_color_mode: bool,
    /// 定时切换执行标志，防止任务并行执行
    pub auto_change_running: Arc<AtomicBool>,
}

impl Default for AutoChangeState {
    fn default() -> Self {
        Self {
            auto_change_enabled: false,
            next_execute_time: None,
            last_executed_time: None,
            auto_detect_color_mode: false,
            auto_change_running: Arc::new(AtomicBool::new(false)),
        }
    }
}
