// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_proxy_protocol_changed(&mut self, protocol: String) -> Task<AppMessage> {
        self.settings_state.proxy_protocol = protocol;
        self.settings_state.proxy_protocol_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_address_changed(&mut self, address: String) -> Task<AppMessage> {
        self.settings_state.proxy_address = address;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_port_changed(&mut self, port: u32) -> Task<AppMessage> {
        // 数字输入框已经限制了范围为 1-65535
        self.settings_state.proxy_port = port;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_proxy_toggled(&mut self, enabled: bool) -> Task<AppMessage> {
        self.settings_state.proxy_enabled = enabled;
        self.config.global.proxy_enabled = enabled;
        self.config.save_to_file();
        info!(
            "[设置] [代理] 开关状态已切换: {}",
            if enabled { "开启" } else { "关闭" }
        );
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_proxy(&mut self) -> Task<AppMessage> {
        // 检查地址和端口是否都设置且端口格式正确
        let is_address_valid = !self.settings_state.proxy_address.trim().is_empty();
        let is_port_valid = self.settings_state.proxy_port >= 1 && self.settings_state.proxy_port <= 65535;

        if self.settings_state.proxy_enabled {
            // 代理开关已开启
            if is_address_valid && is_port_valid {
                // 地址和端口都有效，保存代理设置和开关状态
                let proxy_url = format!(
                    "{}://{}:{}",
                    self.settings_state.proxy_protocol,
                    self.settings_state.proxy_address,
                    self.settings_state.proxy_port
                );
                let old_proxy = self.config.global.proxy.clone();
                info!("[设置] [代理] 保存（已启用）: {} -> {}", old_proxy, proxy_url);
                self.config.set_proxy(proxy_url);
                self.config.global.proxy_enabled = true;
                self.config.save_to_file();
                // 显示成功通知
                self.show_notification(self.i18n.t("settings.proxy-save-success"), NotificationType::Success)
            } else {
                // 地址或端口无效，显示错误通知
                info!("[设置] [代理] 保存失败（开关已开启但配置无效）");
                self.show_notification(self.i18n.t("settings.proxy-save-failed"), NotificationType::Error)
            }
        } else {
            // 代理开关已关闭，保存开关状态为 false（保留代理配置但不使用）
            let old_proxy = self.config.global.proxy.clone();
            info!("[设置] [代理] 保存（已禁用）: proxy_enabled=false, proxy={}", old_proxy);
            self.config.global.proxy_enabled = false;
            self.config.save_to_file();
            // 显示成功通知
            self.show_notification(self.i18n.t("settings.proxy-disabled"), NotificationType::Success)
        }
    }
}
