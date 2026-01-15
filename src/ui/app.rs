use super::App;
use super::AppMessage;
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

        // 检查代理配置格式，如果不正确则还原为空字符串
        let (proxy_protocol, proxy_address, proxy_port) = Self::parse_proxy_string(&config.global.proxy);
        if config.global.proxy != format!("{}://{}:{}", proxy_protocol, proxy_address, proxy_port)
            && !config.global.proxy.is_empty()
        {
            // 代理格式不正确，还原为空字符串
            config.global.proxy = String::new();
            config.save_to_file();
        }

        let _tray_icon = Self::init_tray(&i18n);

        Self {
            i18n,
            config: config.clone(),
            active_page: super::ActivePage::OnlineWallpapers,
            pending_window_size: None,
            debounce_timer: std::time::Instant::now(),
            _tray_icon,
            proxy_protocol,
            proxy_address,
            proxy_port,
            wallhaven_api_key: config.wallhaven.api_key.clone(), // 初始化API KEY状态
            show_close_confirmation: false,
            remember_close_setting: false,
            show_path_clear_confirmation: false,
            path_to_clear: String::new(),
            show_notification: false,
            notification_message: String::new(),
            notification_type: super::NotificationType::Success,
            current_window_width: config.display.width,
            local_state: super::local::LocalState::default(),
            online_state: super::online::OnlineState::load_from_config(&config),
            download_state: super::download::DownloadStateFull::new(),
            initial_loaded: false, // 标记是否已加载初始数据
        }
    }

    // 获取初始任务（用于启动时加载在线壁纸）
    pub fn get_initial_tasks(&self) -> iced::Task<AppMessage> {
        iced::Task::batch(vec![
            iced::Task::perform(async {}, |_| {
                AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)
            }),
            iced::Task::perform(async {}, |_| {
                AppMessage::ScrollToTop("online_wallpapers_scroll".to_string())
            }),
        ])
    }

    // 解析代理字符串为协议、地址和端口
    pub fn parse_proxy_string(proxy: &str) -> (String, String, String) {
        if proxy.is_empty() {
            return ("http".to_string(), "".to_string(), "".to_string());
        }

        // 尝试解析代理URL格式: protocol://address:port
        if let Some(at) = proxy.find("://") {
            let protocol = &proxy[..at];
            let remaining = &proxy[at + 3..];

            if let Some(colon_index) = remaining.rfind(':') {
                let address = &remaining[..colon_index];
                let port_str = &remaining[colon_index + 1..];

                // 验证端口号是否为有效数字
                if let Ok(port) = port_str.parse::<u16>() {
                    if port != 0 {
                        // u16的范围是0-65535，所以只需检查不为0
                        return (protocol.to_string(), address.to_string(), port_str.to_string());
                    }
                }
            }
        }

        // 如果格式不正确，返回默认值
        ("http".to_string(), "".to_string(), "".to_string())
    }

    pub fn title(&self) -> String {
        self.i18n.t("app-title")
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
        use iced::{
            Alignment, Length,
            widget::{button, column, container, row, text},
        };

        let path_display = self.get_path_display(&self.path_to_clear);

        let dialog_content = column![
            text(self.i18n.t("path-clear-confirmation.title"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            text(self.i18n.t("path-clear-confirmation.message"))
                .size(14)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            text(path_display)
                .size(12)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb8(220, 53, 69)), // 红色文字
                }),
            row![
                button(text(self.i18n.t("path-clear-confirmation.confirm")).size(14))
                    .on_press(AppMessage::ConfirmPathClear(self.path_to_clear.clone()))
                    .style(|_theme: &iced::Theme, _status| {
                        let base = iced::widget::button::text(_theme, _status);
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb8(220, 53, 69))), // 红色
                            text_color: iced::Color::WHITE,
                            ..base
                        }
                    }),
                button(text(self.i18n.t("path-clear-confirmation.cancel")).size(14))
                    .on_press(AppMessage::CancelPathClear)
                    .style(|_theme: &iced::Theme, _status| {
                        let base = iced::widget::button::text(_theme, _status);
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb8(108, 117, 125))), // 灰色
                            text_color: iced::Color::WHITE,
                            ..base
                        }
                    }),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .padding(20)
        .spacing(15)
        .align_x(Alignment::Center);

        // 将对话框包装在容器中，设置样式（白色背景，边框）
        let modal_dialog = container(dialog_content)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .width(400) // 限制对话框最大宽度
            .padding(10)
            .style(|_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(iced::Color::WHITE)),
                border: iced::border::Border {
                    radius: iced::border::Radius::from(8.0),
                    width: 1.0,
                    color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                },
                ..Default::default()
            });

        // 创建完整的模态内容：使用容器包含半透明背景和居中的对话框
        let modal_content = container(
            // 使用stack将遮罩层和居中对话框叠加
            iced::widget::stack(vec![
                // 半透明背景遮罩
                container(iced::widget::Space::new())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(|_theme: &iced::Theme| iced::widget::container::Style {
                        background: Some(iced::Background::Color(iced::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.5, // 半透明背景，实现模态效果
                        })),
                        ..Default::default()
                    })
                    .into(),
                // 居中的对话框
                container(modal_dialog)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .into(),
            ]),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        // 返回使用opaque包装的模态内容
        iced::widget::opaque(modal_content).into()
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
                iced::Color::from_rgb8(40, 167, 69), // 绿色背景
                iced::Color::WHITE,                  // 白色文字
            ),
            super::NotificationType::Error => (
                iced::Color::from_rgb8(220, 53, 69), // 红色背景
                iced::Color::WHITE,                  // 白色文字
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
