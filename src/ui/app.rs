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
        if config.global.proxy != format!("{}://{}:{}", proxy_protocol, proxy_address, proxy_port) &&
           !config.global.proxy.is_empty() {
            // 代理格式不正确，还原为空字符串
            config.global.proxy = String::new();
            config.save_to_file();
        }

        let _tray_icon = Self::init_tray(&i18n);

        Self {
            i18n,
            config,
            active_page: super::ActivePage::OnlineWallpapers,
            pending_window_size: None,
            pending_window_position: None,
            debounce_timer: std::time::Instant::now(),
            _tray_icon,
            proxy_protocol,
            proxy_address,
            proxy_port,
            show_close_confirmation: false,
            remember_close_setting: false,
        }
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
                    if port != 0 {  // u16的范围是0-65535，所以只需检查不为0
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

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        // 如果显示关闭确认对话框，则只显示对话框，不显示底层内容
        if self.show_close_confirmation {
            super::close_confirmation::close_confirmation_view(self)
        } else {
            // 否则显示正常界面
            super::main::view_internal(self)
        }
    }
}
