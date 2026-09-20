// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 日志展示窗口
//!
//! 独立窗口实时滚动展示应用日志。数据源为内存层
//! （[`crate::utils::log_window_layer`]），与文件层/控制台层链路隔离：
//! 本窗口的展示等级与最大保留行数独立于日志文件配置，互不影响。

pub mod message;
pub mod view;

pub use message::LogWindowMessage;
pub use view::log_window_view;

use std::collections::VecDeque;
use std::time::Duration;

use iced::{Size, window};

use crate::utils::assets;
use crate::utils::log_window_layer::LogLine;

/// 日志窗口默认尺寸
const LOG_WINDOW_WIDTH: f32 = 900.0;
const LOG_WINDOW_HEIGHT: f32 = 600.0;
/// 日志窗口最小尺寸
const LOG_WINDOW_MIN_WIDTH: f32 = 480.0;
const LOG_WINDOW_MIN_HEIGHT: f32 = 360.0;
/// 窗口图标尺寸
const LOG_WINDOW_ICON_SIZE: u32 = 64;
/// 内存层日志拉取间隔
pub const TICK_INTERVAL: Duration = Duration::from_millis(200);

/// 日志滚动区域 id（自动滚动到底的 `scroll_to` 操作目标）
pub const SCROLL_ID: &str = "log-window-scroll";

/// 构建日志展示窗口的 [`window::Settings`]
///
/// 普通辅助窗口：原生标题栏、可调整大小。`exit_on_close_request: false`
/// 由事件处理器统一关窗并清理状态（daemon 下手动管理窗口生命周期）
pub fn log_window_settings() -> window::Settings {
    let (rgba, width, height) = assets::get_logo(LOG_WINDOW_ICON_SIZE);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    window::Settings {
        position: window::Position::Centered,
        size: Size::new(LOG_WINDOW_WIDTH, LOG_WINDOW_HEIGHT),
        min_size: Some(Size::new(LOG_WINDOW_MIN_WIDTH, LOG_WINDOW_MIN_HEIGHT)),
        icon: Some(icon),
        exit_on_close_request: false,
        visible: true,
        ..window::Settings::default()
    }
}

/// 日志展示窗口状态
pub struct LogWindowState {
    /// 展示缓冲（结构化行，最多 config.global.log_window_max_lines 条）
    pub lines: VecDeque<LogLine>,
    /// 内存层拉取游标（已渲染到的序号）
    pub cursor_seq: u64,
    /// 是否自动滚动到底部
    pub auto_scroll: bool,
    /// 等级下拉框展开状态
    pub level_picker_expanded: bool,
    /// 最大行数输入框内容（允许中间态，提交时解析）
    pub max_lines_input: String,
    /// 筛选关键字（空 = 显示全部；只影响显示，后台照常全量记录）
    pub filter_text: String,
}

impl Default for LogWindowState {
    fn default() -> Self {
        Self {
            lines: VecDeque::new(),
            cursor_seq: 0,
            auto_scroll: true,
            level_picker_expanded: false,
            max_lines_input: String::new(),
            filter_text: String::new(),
        }
    }
}
