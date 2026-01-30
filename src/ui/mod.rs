// Copyright (C) 2026 zsyo - GNU AGPL v3.0

pub mod app;
pub mod auto_change;
pub mod common;
pub mod download;
pub mod local;
pub mod main;
pub mod online;
pub mod settings;
pub mod style;
pub mod subscription;
pub mod update;
pub mod view;

use crate::i18n::I18n;
use crate::ui::main::TrayManager;
use crate::utils::config::CloseAction;

#[derive(Debug, Clone)]
pub enum CloseConfirmationAction {
    MinimizeToTray,
    CloseApp,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    None, // 空消息，用于某些不需要实际操作的情况
    Main(crate::ui::main::MainMessage),
    Local(crate::ui::local::LocalMessage),
    Online(crate::ui::online::OnlineMessage),
    Download(crate::ui::download::DownloadMessage),
    Settings(crate::ui::settings::SettingsMessage),
    AutoChange(crate::ui::auto_change::AutoChangeMessage),
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivePage {
    OnlineWallpapers,
    LocalList,
    DownloadProgress,
    Settings,
}

pub struct App {
    pub i18n: I18n,
    pub config: crate::utils::config::Config,
    active_page: ActivePage,
    pending_window_size: Option<(u32, u32)>,
    debounce_timer: std::time::Instant,
    tray_manager: TrayManager,
    // 主题配置
    pub theme_config: crate::ui::style::ThemeConfig,
    // 关闭确认对话框状态
    pub show_close_confirmation: bool,
    // 通知状态
    pub show_notification: bool,
    pub notification_message: String,
    pub notification_type: NotificationType,
    // 当前窗口宽度，用于响应式布局
    pub current_window_width: u32,
    // 当前窗口高度，用于判断是否需要自动加载下一页
    pub current_window_height: u32,
    // 当前每行可显示的壁纸数量（用于估算内容高度）
    pub current_items_per_row: usize,
    // 本地壁纸页面状态
    pub local_state: crate::ui::local::LocalState,
    // 在线壁纸页面状态
    pub online_state: crate::ui::online::OnlineState,
    // 设置页面状态
    pub settings_state: crate::ui::settings::SettingsState,
    // 下载管理页面状态
    pub download_state: crate::ui::download::DownloadStateFull,
    // 定时切换壁纸状态
    pub auto_change_state: crate::ui::auto_change::AutoChangeState,
    // 标记是否已加载初始数据
    pub initial_loaded: bool,
    // 定时切换执行标志，防止任务并行执行
    pub auto_change_running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    // 壁纸切换历史记录（最多50条）
    pub wallpaper_history: Vec<String>,
    pub is_visible: bool,
    // 自定义标题栏状态
    pub is_maximized: bool, // 窗口是否已最大化
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
