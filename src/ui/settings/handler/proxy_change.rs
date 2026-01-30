// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_proxy_protocol_changed(&mut self, protocol: String) -> Task<AppMessage> {
        self.proxy_protocol = protocol;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_address_changed(&mut self, address: String) -> Task<AppMessage> {
        self.proxy_address = address;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_port_changed(&mut self, port: u32) -> Task<AppMessage> {
        // 数字输入框已经限制了范围为 1-65535
        self.proxy_port = port;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_proxy(&mut self) -> Task<AppMessage> {
        // 检查地址和端口是否都设置且端口格式正确
        let is_address_valid = !self.proxy_address.trim().is_empty();
        let is_port_valid = self.proxy_port >= 1 && self.proxy_port <= 65535;

        if is_address_valid && is_port_valid {
            // 地址和端口都有效，保存代理设置
            let proxy_url = format!("{}://{}:{}", self.proxy_protocol, self.proxy_address, self.proxy_port);
            let old_proxy = self.config.global.proxy.clone();
            info!("[设置] [代理] 保存: {} -> {}", old_proxy, proxy_url);
            self.config.set_proxy(proxy_url);
            // 显示成功通知
            self.show_notification("代理设置保存成功".to_string(), NotificationType::Success)
        } else {
            // 地址或端口无效，保存为空字符串（相当于关闭代理）
            self.config.set_proxy(String::new());
            // 同时清空地址和端口输入框（端口显示为 1080）
            self.proxy_address = String::new();
            self.proxy_port = 1080;
            // 显示错误通知
            self.show_notification("格式错误，代理设置保存失败".to_string(), NotificationType::Error)
        }
    }
}
