// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::SettingsState;
use crate::utils::config::Config;

impl SettingsState {
    /// 从配置文件加载设置状态
    pub fn load_from_config(config: &Config) -> Self {
        // 解析代理字符串
        let (proxy_protocol, proxy_address, proxy_port) = Self::parse_proxy_string(&config.global.proxy);

        Self {
            language_picker_expanded: false,
            proxy_protocol_picker_expanded: false,
            theme_picker_expanded: false,
            proxy_protocol,
            proxy_address,
            proxy_port,
            wallhaven_api_key: config.wallhaven.api_key.clone(),
            wallpaper_mode: config.wallpaper.mode,
            auto_change_mode: config.wallpaper.auto_change_mode,
            auto_change_interval: config.wallpaper.auto_change_interval,
            custom_interval_minutes: config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30),
            auto_change_query: config.wallpaper.auto_change_query.clone(),
            show_path_clear_confirmation: false,
            path_to_clear: String::new(),
        }
    }

    /// 解析代理字符串为协议、地址和端口
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

                if let Ok(port) = port_str.parse::<u32>() {
                    return (protocol.to_string(), address.to_string(), port);
                }
            }

            return (protocol.to_string(), remaining.to_string(), 0);
        }

        ("http".to_string(), proxy.to_string(), 0)
    }
}
