// Copyright (C) 2026 zsyo - GNU AGPL v3.0

pub mod app;
pub mod async_tasks;
pub mod auto_change;
pub mod close_confirmation;
pub mod common;
pub mod download;
pub mod download_handlers;
pub mod local;
pub mod local_handlers;
pub mod main;
pub mod online;
pub mod online_filter;
pub mod online_handlers;
pub mod online_list;
pub mod online_modal;
pub mod settings;
pub mod settings_handlers;
pub mod style;
pub mod tray;
pub mod update;
pub mod widget;

use crate::i18n::I18n;
use crate::utils::config::CloseAction;

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
    ExecutePendingSave,
    PageSelected(ActivePage),
    AutoStartupToggled(bool),
    LoggingToggled(bool),
    CloseActionSelected(CloseAction),
    WindowCloseRequested,
    WindowFocused,
    MinimizeToTray,
    TrayIconClicked,
    TrayMenuEvent(String),
    OpenUrl(String),
    DataPathSelected(String),
    CachePathSelected(String),
    OpenPath(String),
    OpenLogsPath,                      // 打开日志目录
    ShowPathClearConfirmation(String), // 显示路径清空确认对话框，参数为路径类型 ("data" 或 "cache")
    ConfirmPathClear(String),          // 确认清空路径，参数为路径类型
    CancelPathClear,                   // 取消清空路径
    RestoreDefaultPath(String),
    WallhavenApiKeyChanged(String),
    SaveWallhavenApiKey,
    ScrollToTop(String), // 滚动到顶部，参数为滚动组件ID
    ProxyProtocolChanged(String),
    ProxyAddressChanged(String),
    ProxyPortChanged(u32),
    SaveProxy,
    WallpaperModeSelected(crate::utils::config::WallpaperMode), // 壁纸模式选择
    AutoChangeModeSelected(crate::utils::config::WallpaperAutoChangeMode), // 定时切换模式选择
    AutoChangeIntervalSelected(crate::utils::config::WallpaperAutoChangeInterval), // 定时切换周期选择
    CustomIntervalMinutesChanged(u32),                          // 自定义切换周期分钟数变化
    AutoChangeQueryChanged(String),                             // 定时切换关键词变化
    SaveAutoChangeQuery,                                        // 保存定时切换关键词
    // 语言和代理协议下拉框相关消息
    LanguagePickerExpanded,      // 展开语言选择器
    LanguagePickerDismiss,       // 关闭语言选择器
    ProxyProtocolPickerExpanded, // 展开代理协议选择器
    ProxyProtocolPickerDismiss,  // 关闭代理协议选择器
    // 主题下拉框相关消息
    ThemePickerExpanded,                        // 展开主题选择器
    ThemePickerDismiss,                         // 关闭主题选择器
    ThemeSelected(crate::utils::config::Theme), // 主题选择
    AutoDetectColorModeTick,                    // 自动检测颜色模式
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
    AutoChange(crate::ui::auto_change::AutoChangeMessage), // 定时切换壁纸相关消息
    // 托盘菜单切换壁纸相关消息
    TraySwitchPreviousWallpaper,
    TraySwitchNextWallpaper,
    AddToWallpaperHistory(String),  // 添加壁纸到历史记录
    RemoveLastFromWallpaperHistory, // 从历史记录末尾移除壁纸
    ExternalInstanceTriggered(String),
    // 自定义标题栏相关消息
    TitleBarDrag,     // 拖拽标题栏
    TitleBarMinimize, // 最小化窗口
    TitleBarMaximize, // 最大化/还原窗口
    TitleBarClose,    // 关闭窗口
    // 窗口边缘调整大小相关消息
    ResizeWindow(iced::window::Direction), // 调整窗口大小（包含所有方向）
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
    tray_manager: tray::TrayManager,
    // 主题配置
    pub theme_config: crate::ui::style::ThemeConfig,
    // 代理设置的临时状态
    pub proxy_protocol: String,
    pub proxy_address: String,
    pub proxy_port: u32,
    // 语言下拉框展开状态
    pub language_picker_expanded: bool,
    // 代理协议下拉框展开状态
    pub proxy_protocol_picker_expanded: bool,
    // 主题下拉框展开状态
    pub theme_picker_expanded: bool,
    // API KEY设置的临时状态
    pub wallhaven_api_key: String,
    // 壁纸模式设置的临时状态
    pub wallpaper_mode: crate::utils::config::WallpaperMode,
    // 定时切换模式设置的临时状态
    pub auto_change_mode: crate::utils::config::WallpaperAutoChangeMode,
    // 定时切换周期设置的临时状态
    pub auto_change_interval: crate::utils::config::WallpaperAutoChangeInterval,
    // 自定义切换周期分钟数
    pub custom_interval_minutes: u32,
    // 定时切换关键词
    pub auto_change_query: String,
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
