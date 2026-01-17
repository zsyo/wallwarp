pub mod app;
pub mod async_tasks;
pub mod close_confirmation;
pub mod common;
pub mod download;
pub mod download_handlers;
pub mod local;
pub mod local_handlers;
pub mod main;
pub mod online;
pub mod online_filter;
pub mod online_list;
pub mod online_modal;
pub mod online_handlers;
pub mod settings;
pub mod settings_handlers;
pub mod style;
pub mod tray;
pub mod update;
pub mod widget;

use crate::i18n::I18n;
use crate::utils::config::CloseAction;
use tray_icon::TrayIcon;

#[derive(Debug, Clone)]
pub enum CloseConfirmationAction {
    MinimizeToTray,
    CloseApp,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    None, // 空消息，用于某些不需要实际操作的情况
    LanguageSelected(String),
    WindowResized(u32, u32), // 窗口大小改变事件
    DebounceTimer,
    PageSelected(ActivePage),
    AutoStartupToggled(bool),
    CloseActionSelected(CloseAction),
    WindowCloseRequested,
    MinimizeToTray,
    TrayIconClicked,
    TrayMenuEvent(String),
    OpenUrl(String),
    DataPathSelected(String),
    CachePathSelected(String),
    OpenPath(String),
    ShowPathClearConfirmation(String), // 显示路径清空确认对话框，参数为路径类型 ("data" 或 "cache")
    ConfirmPathClear(String),          // 确认清空路径，参数为路径类型
    CancelPathClear,                   // 取消清空路径
    RestoreDefaultPath(String),
    WallhavenApiKeyChanged(String),
    SaveWallhavenApiKey,
    ScrollToTop(String), // 滚动到顶部，参数为滚动组件ID
    ProxyProtocolChanged(String),
    ProxyAddressChanged(String),
    ProxyPortChanged(String),
    SaveProxy,
    // 通知相关消息
    ShowNotification(String, NotificationType), // 显示通知，参数：消息内容，通知类型
    HideNotification,                           // 隐藏通知（用于定时隐藏）
    // 关闭确认对话框相关消息
    ShowCloseConfirmation,
    CloseConfirmationResponse(CloseConfirmationAction, bool), // (动作, 是否记住设置)
    CloseConfirmationCancelled,
    ToggleRememberSetting(bool),
    Local(crate::ui::local::LocalMessage),
    Online(crate::ui::online::OnlineMessage),
    Download(crate::ui::download::DownloadMessage),
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
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
    _tray_icon: TrayIcon,
    // 代理设置的临时状态
    pub proxy_protocol: String,
    pub proxy_address: String,
    pub proxy_port: String,
    // API KEY设置的临时状态
    pub wallhaven_api_key: String,
    // 关闭确认对话框状态
    pub show_close_confirmation: bool,
    pub remember_close_setting: bool,
    // 路径清空确认对话框状态
    pub show_path_clear_confirmation: bool,
    pub path_to_clear: String, // "data" 或 "cache"
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
    // 下载管理页面状态
    pub download_state: crate::ui::download::DownloadStateFull,
    // 标记是否已加载初始数据
    pub initial_loaded: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
