// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use super::common;
use crate::i18n::I18n;
use crate::utils::config::Config;
use iced;

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

        let tray_manager = super::tray::TrayManager::new(&i18n);

        // 根据配置文件中的主题配置初始化主题
        let theme_config = match config.global.theme {
            crate::utils::config::Theme::Dark => crate::ui::style::ThemeConfig::new(crate::ui::style::Theme::Dark),
            crate::utils::config::Theme::Light => crate::ui::style::ThemeConfig::new(crate::ui::style::Theme::Light),
            crate::utils::config::Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = crate::utils::window_utils::get_system_color_mode();
                tracing::info!(
                    "[启动] [主题] 自动模式，系统主题: {}",
                    if is_system_dark { "深色" } else { "浅色" }
                );

                if is_system_dark {
                    crate::ui::style::ThemeConfig::new(crate::ui::style::Theme::Dark)
                } else {
                    crate::ui::style::ThemeConfig::new(crate::ui::style::Theme::Light)
                }
            }
        };

        // 根据配置文件中的定时切换周期初始化定时任务状态
        let (auto_change_enabled, auto_change_timer, auto_change_last_time) = if matches!(
            config.wallpaper.auto_change_interval,
            crate::utils::config::WallpaperAutoChangeInterval::Off
        ) {
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
            notification_type: super::NotificationType::Success,
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
            initial_loaded: false, // 标记是否已加载初始数据
            auto_change_running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), // 初始化定时切换执行标志
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
        self.i18n.t("app-title")
    }

    // 辅助方法：显示通知
    pub fn show_notification(
        &mut self,
        message: String,
        notification_type: super::NotificationType,
    ) -> iced::Task<AppMessage> {
        self.notification_message = message;
        self.notification_type = notification_type;
        self.show_notification = true;

        iced::Task::perform(
            async {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            },
            |_| AppMessage::HideNotification,
        )
    }

    // 辅助方法：获取路径显示字符串
    fn get_path_display(&self, path_type: &str) -> &str {
        match path_type {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => "",
        }
    }

    // 渲染路径清空确认对话框
    fn path_clear_confirmation_view(&self) -> iced::Element<'_, AppMessage> {
        let path_display = self.get_path_display(&self.path_to_clear);

        // 将消息转换为字符串（简化处理）
        let message_text = format!("{}\n{}", self.i18n.t("path-clear-confirmation.message"), path_display);

        common::create_confirmation_dialog(
            self.i18n.t("path-clear-confirmation.title"),
            message_text,
            self.i18n.t("path-clear-confirmation.confirm"),
            self.i18n.t("path-clear-confirmation.cancel"),
            AppMessage::ConfirmPathClear(self.path_to_clear.clone()),
            AppMessage::CancelPathClear,
        )
    }

    // 渲染通知组件
    fn notification_view(&self) -> iced::Element<'_, AppMessage> {
        use iced::{
            Length,
            widget::{container, text},
        };

        // 根据通知类型设置颜色
        let (bg_color, text_color) = match self.notification_type {
            super::NotificationType::Success => (
                super::style::NOTIFICATION_SUCCESS_BG,
                super::style::NOTIFICATION_TEXT_COLOR,
            ),
            super::NotificationType::Error => (
                super::style::NOTIFICATION_ERROR_BG,
                super::style::NOTIFICATION_TEXT_COLOR,
            ),
            super::NotificationType::Info => (
                super::style::NOTIFICATION_INFO_BG,
                super::style::NOTIFICATION_TEXT_COLOR,
            ),
        };

        let notification_content =
            container(
                text(&self.notification_message)
                    .size(14)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(text_color),
                    }),
            )
            .padding(10)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .style(move |_theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(bg_color)),
                border: iced::border::Border {
                    radius: iced::border::Radius::from(8.0),
                    width: 1.0,
                    color: iced::Color::TRANSPARENT,
                },
                ..Default::default()
            });

        // 将通知放在窗口底部中央
        container(
            container(notification_content)
                .width(Length::Shrink)
                .height(Length::Shrink)
                .padding(10),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Bottom)
        .into()
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        // 先渲染底层内容
        let base_content = super::main::view_internal(self);

        // 如果显示任何确认对话框，则将对话框叠加在底层内容上
        let main_content = if self.show_close_confirmation {
            Self::create_stack(base_content, super::close_confirmation::close_confirmation_view(self))
        } else if self.show_path_clear_confirmation {
            Self::create_stack(base_content, self.path_clear_confirmation_view())
        } else {
            base_content
        };

        // 如果显示通知，则将通知叠加在主要内容之上
        if self.show_notification {
            Self::create_stack(main_content, self.notification_view())
        } else {
            main_content
        }
    }

    // 辅助方法：创建叠加层（底层内容 + 覆盖内容）
    fn create_stack<'a>(
        base: iced::Element<'a, AppMessage>,
        overlay: iced::Element<'a, AppMessage>,
    ) -> iced::Element<'a, AppMessage> {
        iced::widget::stack(vec![base, overlay])
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
