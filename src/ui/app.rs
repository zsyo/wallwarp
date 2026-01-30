// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, NotificationType};
use crate::i18n::I18n;
use crate::ui::main::TrayManager;
use crate::ui::style;
use crate::utils::config::{Config, Theme, WallpaperAutoChangeInterval};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

impl App {
    pub fn new() -> Self {
        let i18n = I18n::new();
        let config = Config::new(&i18n.current_lang);
        Self::new_with_config(i18n, config)
    }

    pub fn new_with_config(mut i18n: I18n, mut config: Config) -> Self {
        // 根据配置设置语言
        i18n.set_language(config.global.language.clone());

        // 初始化窗口最大化状态（默认为 false）
        let is_maximized = false;

        // 检查代理配置格式，如果不正确则还原为空字符串
        let (proxy_protocol, proxy_address, mut proxy_port) = Self::parse_proxy_string(&config.global.proxy);
        if proxy_port > 0 {
            let expected_proxy = format!("{}://{}:{}", proxy_protocol, proxy_address, proxy_port);
            if config.global.proxy != expected_proxy {
                // 代理格式不正确，还原为空字符串
                config.global.proxy = String::new();
                config.save_to_file();
            }
        } else {
            proxy_port = 1080;
        }

        let tray_manager = TrayManager::new(&i18n);

        // 根据配置文件中的主题配置初始化主题
        let theme_config = match config.global.theme {
            Theme::Dark => style::ThemeConfig::new(style::Theme::Dark),
            Theme::Light => style::ThemeConfig::new(style::Theme::Light),
            Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = crate::utils::window_utils::get_system_color_mode();
                tracing::info!(
                    "[启动] [主题] 自动模式，系统主题: {}",
                    if is_system_dark { "深色" } else { "浅色" }
                );

                if is_system_dark {
                    style::ThemeConfig::new(style::Theme::Dark)
                } else {
                    style::ThemeConfig::new(style::Theme::Light)
                }
            }
        };

        // 根据配置文件中的定时切换周期初始化定时任务状态
        let (auto_change_enabled, auto_change_timer, auto_change_last_time) =
            if matches!(config.wallpaper.auto_change_interval, WallpaperAutoChangeInterval::Off) {
                // 配置为关闭状态，不启动定时任务
                tracing::info!("[定时切换] [启动] 配置为关闭状态，定时任务未启动");
                (false, None, None)
            } else {
                // 配置为开启状态，自动启动定时任务
                let now = std::time::Instant::now();
                if let Some(minutes) = config.wallpaper.auto_change_interval.get_minutes() {
                    let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                    tracing::info!(
                        "[定时切换] [启动] 配置为开启状态，间隔: {}分钟, 下次执行时间: {}",
                        minutes,
                        next_time.format("%Y-%m-%d %H:%M:%S")
                    );
                }
                (true, Some(now), Some(now))
            };

        Self {
            i18n,
            config: config.clone(),
            active_page: super::ActivePage::OnlineWallpapers,
            pending_window_size: None,
            debounce_timer: std::time::Instant::now(),
            tray_manager,
            theme_config,
            proxy_protocol,
            proxy_address,
            proxy_port,
            language_picker_expanded: false,
            proxy_protocol_picker_expanded: false,
            theme_picker_expanded: false,
            wallhaven_api_key: config.wallhaven.api_key.clone(), // 初始化API KEY状态
            wallpaper_mode: config.wallpaper.mode,               // 初始化壁纸模式状态
            auto_change_mode: config.wallpaper.auto_change_mode, // 初始化定时切换模式状态
            auto_change_interval: config.wallpaper.auto_change_interval, // 初始化定时切换周期状态
            custom_interval_minutes: config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30), // 初始化自定义分钟数，默认为30
            auto_change_query: config.wallpaper.auto_change_query.clone(), // 初始化定时切换关键词
            show_close_confirmation: false,
            remember_close_setting: false,
            show_path_clear_confirmation: false,
            path_to_clear: String::new(),
            show_notification: false,
            notification_message: String::new(),
            notification_type: NotificationType::Success,
            current_window_width: config.display.width,
            current_window_height: config.display.height,
            current_items_per_row: 1, // 初始值为1
            local_state: super::local::LocalState::default(),
            online_state: super::online::OnlineState::load_from_config(&config),
            auto_change_state: super::auto_change::AutoChangeState {
                auto_change_enabled,
                auto_change_timer,
                auto_change_last_time,
                auto_detect_color_mode: config.global.theme == crate::utils::config::Theme::Auto,
            },
            download_state: super::download::DownloadStateFull::new(),
            initial_loaded: false,                                 // 标记是否已加载初始数据
            auto_change_running: Arc::new(AtomicBool::new(false)), // 初始化定时切换执行标志
            wallpaper_history: {
                // 初始化壁纸切换历史记录，获取当前壁纸路径并添加到记录中
                let mut history = Vec::new();
                if let Ok(current_wallpaper) = wallpaper::get() {
                    if !current_wallpaper.is_empty() {
                        tracing::info!("[壁纸历史] 初始化，添加当前壁纸: {}", current_wallpaper);
                        history.push(current_wallpaper);
                    }
                }
                history
            },
            is_visible: false,
            is_maximized, // 初始化窗口最大化状态
        }
    }

    // 解析代理字符串为协议、地址和端口
    pub fn parse_proxy_string(proxy: &str) -> (String, String, u32) {
        if proxy.is_empty() {
            return ("http".to_string(), "".to_string(), 0);
        }

        if let Some(at) = proxy.find("://") {
            let protocol = &proxy[..at];
            let remaining = &proxy[at + 3..];

            if let Some(colon_index) = remaining.rfind(':') {
                let address = &remaining[..colon_index];
                let port_str = &remaining[colon_index + 1..];

                // 验证端口号是否为有效数字
                if let Ok(port) = port_str.parse::<u32>() {
                    if port >= 1 && port <= 65535 {
                        return (protocol.to_string(), address.to_string(), port);
                    }
                }
            }
        }

        // 如果格式不正确，返回默认值（端口显示为 1080，但实际代理为空）
        ("http".to_string(), "".to_string(), 1080)
    }

    pub fn title(&self) -> String {
        self.i18n.t("wallwarp")
    }
}
