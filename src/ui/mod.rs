pub mod app;
pub mod close_confirmation;
pub mod main;
pub mod settings;
pub mod tray;
pub mod update;

use crate::i18n::I18n;
use crate::utils::config::CloseAction;
use crate::utils::config::Config;
use tray_icon::TrayIcon;

#[derive(Debug, Clone)]
pub enum CloseConfirmationAction {
    MinimizeToTray,
    CloseApp,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    LanguageSelected(String),
    WindowResized(u32, u32), // 窗口大小改变事件
    WindowMoved(i32, i32),   // 窗口位置改变事件
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
    ClearPath(String),
    RestoreDefaultPath(String),
    WallhavenApiKeyChanged(String),
    ProxyProtocolChanged(String),
    ProxyAddressChanged(String),
    ProxyPortChanged(String),
    SaveProxy,
    // 关闭确认对话框相关消息
    ShowCloseConfirmation,
    CloseConfirmationResponse(CloseConfirmationAction, bool), // (动作, 是否记住设置)
    CloseConfirmationCancelled,
    ToggleRememberSetting(bool),
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
    pub config: Config,
    active_page: ActivePage,
    pending_window_size: Option<(u32, u32)>,
    pending_window_position: Option<(i32, i32)>,
    debounce_timer: std::time::Instant,
    _tray_icon: TrayIcon,
    // 代理设置的临时状态
    pub proxy_protocol: String,
    pub proxy_address: String,
    pub proxy_port: String,
    // 关闭确认对话框状态
    pub show_close_confirmation: bool,
    pub remember_close_setting: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
