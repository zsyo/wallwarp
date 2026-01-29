// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use crate::services::async_task;
use crate::services::wallhaven;
use crate::utils::{config::CloseAction, helpers, startup};
use tracing::error;

impl App {
    /// 处理设置相关消息
    pub fn handle_settings_message(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        match msg {
            AppMessage::LanguageSelected(lang) => self.handle_language_selected(lang),
            AppMessage::AutoStartupToggled(enabled) => self.handle_auto_startup_toggled(enabled),
            AppMessage::LoggingToggled(enabled) => self.handle_logging_toggled(enabled),
            AppMessage::CloseActionSelected(action) => self.handle_close_action_selected(action),
            AppMessage::OpenUrl(url) => self.handle_open_url(url),
            AppMessage::DataPathSelected(path) => self.handle_data_path_selected(path),
            AppMessage::CachePathSelected(path) => self.handle_cache_path_selected(path),
            AppMessage::OpenPath(path_type) => self.handle_open_path(path_type),
            AppMessage::OpenLogsPath => self.handle_open_logs_path(),
            AppMessage::ShowPathClearConfirmation(path_type) => self.handle_show_path_clear_confirmation(path_type),
            AppMessage::ConfirmPathClear(path_type) => self.handle_confirm_path_clear(path_type),
            AppMessage::CancelPathClear => self.handle_cancel_path_clear(),
            AppMessage::RestoreDefaultPath(path_type) => self.handle_restore_default_path(path_type),
            AppMessage::WallhavenApiKeyChanged(api_key) => self.handle_wallhaven_api_key_changed(api_key),
            AppMessage::SaveWallhavenApiKey => self.handle_save_wallhaven_api_key(),
            AppMessage::ProxyProtocolChanged(protocol) => self.handle_proxy_protocol_changed(protocol),
            AppMessage::ProxyAddressChanged(address) => self.handle_proxy_address_changed(address),
            AppMessage::ProxyPortChanged(port) => self.handle_proxy_port_changed(port),
            AppMessage::SaveProxy => self.handle_save_proxy(),
            AppMessage::WallpaperModeSelected(mode) => self.handle_wallpaper_mode_selected(mode),
            AppMessage::AutoChangeModeSelected(mode) => self.handle_auto_change_mode_selected(mode),
            AppMessage::AutoChangeIntervalSelected(interval) => self.handle_auto_change_interval_selected(interval),
            AppMessage::CustomIntervalMinutesChanged(minutes) => self.handle_custom_interval_minutes_changed(minutes),
            AppMessage::AutoChangeQueryChanged(query) => self.handle_auto_change_query_changed(query),
            AppMessage::SaveAutoChangeQuery => self.handle_save_auto_change_query(),
            AppMessage::LanguagePickerExpanded => self.handle_language_picker_expanded(),
            AppMessage::LanguagePickerDismiss => self.handle_language_picker_dismiss(),
            AppMessage::ProxyProtocolPickerExpanded => self.handle_proxy_protocol_picker_expanded(),
            AppMessage::ProxyProtocolPickerDismiss => self.handle_proxy_protocol_picker_dismiss(),
            AppMessage::ThemePickerExpanded => self.handle_theme_picker_expanded(),
            AppMessage::ThemePickerDismiss => self.handle_theme_picker_dismiss(),
            _ => iced::Task::none(),
        }
    }

    fn handle_language_selected(&mut self, lang: String) -> iced::Task<AppMessage> {
        let old_lang = self.config.global.language.clone();
        tracing::info!("[设置] [语言] 修改: {} -> {}", old_lang, lang);
        self.i18n.set_language(lang.clone());
        // 同时更新配置
        self.config.set_language(lang);
        iced::Task::none()
    }

    fn handle_auto_startup_toggled(&mut self, enabled: bool) -> iced::Task<AppMessage> {
        if let Err(e) = startup::set_auto_startup(enabled) {
            error!("设置开机启动失败: {}", e);
        }
        iced::Task::none()
    }

    fn handle_logging_toggled(&mut self, enabled: bool) -> iced::Task<AppMessage> {
        let old_value = self.config.global.enable_logging;
        tracing::info!("[设置] [运行日志] 修改: {} -> {}", old_value, enabled);
        self.config.global.enable_logging = enabled;
        self.config.save_to_file();

        // 发送通知
        let message = if enabled {
            self.i18n.t("settings.logging-notice-enabled")
        } else {
            self.i18n.t("settings.logging-notice-disabled")
        };
        self.show_notification(message, super::NotificationType::Info)
    }

    fn handle_close_action_selected(&mut self, action: CloseAction) -> iced::Task<AppMessage> {
        self.config.set_close_action(action);
        iced::Task::none()
    }

    fn handle_open_url(&mut self, url: String) -> iced::Task<AppMessage> {
        if let Err(e) = open::that(&url) {
            error!("Failed to open URL {}: {}", url, e);
        }
        iced::Task::none()
    }

    fn handle_data_path_selected(&mut self, path: String) -> iced::Task<AppMessage> {
        if !path.is_empty() && path != "SELECT_DATA_PATH" {
            // 这是异步任务返回的实际路径
            let old_path = self.config.data.data_path.clone();
            tracing::info!("[设置] [数据路径] 修改: {} -> {}", old_path, path);
            self.config.set_data_path(path);
        } else if path == "SELECT_DATA_PATH" {
            // 这是用户点击按钮时的原始消息，触发异步任务
            return iced::Task::perform(async_task::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    AppMessage::DataPathSelected(selected_path)
                } else {
                    AppMessage::DataPathSelected("".to_string()) // 用户取消选择
                }
            });
        }

        iced::Task::none()
    }

    fn handle_cache_path_selected(&mut self, path: String) -> iced::Task<AppMessage> {
        if !path.is_empty() && path != "SELECT_CACHE_PATH" && path != "SELECT_DATA_PATH" {
            // 这是异步任务返回的实际路径
            let old_path = self.config.data.cache_path.clone();
            tracing::info!("[设置] [缓存路径] 修改: {} -> {}", old_path, path);
            self.config.set_cache_path(path);
        } else if path == "SELECT_CACHE_PATH" {
            // 这是用户点击按钮时的原始消息，触发异步任务
            return iced::Task::perform(async_task::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    AppMessage::CachePathSelected(selected_path)
                } else {
                    AppMessage::CachePathSelected("".to_string()) // 用户取消选择
                }
            });
        } else if path == "SELECT_DATA_PATH" {
            // 这是用户点击数据路径输入框时的原始消息，触发异步任务
            return iced::Task::perform(async_task::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    AppMessage::DataPathSelected(selected_path)
                } else {
                    AppMessage::DataPathSelected("".to_string()) // 用户取消选择
                }
            });
        }

        iced::Task::none()
    }

    fn handle_open_path(&mut self, path_type: String) -> iced::Task<AppMessage> {
        let path_to_open = match path_type.as_str() {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => return iced::Task::none(),
        };

        let full_path = helpers::get_absolute_path(path_to_open);

        if let Err(e) = open::that(&full_path) {
            error!("Failed to open path {}: {}", full_path, e);
        }

        iced::Task::none()
    }

    fn handle_open_logs_path(&mut self) -> iced::Task<AppMessage> {
        let logs_path = "logs";
        let full_path = helpers::get_absolute_path(logs_path);

        if let Err(e) = open::that(&full_path) {
            error!("Failed to open logs path {}: {}", full_path, e);
        }

        iced::Task::none()
    }

    fn handle_show_path_clear_confirmation(&mut self, path_type: String) -> iced::Task<AppMessage> {
        // 显示路径清空确认对话框
        self.show_path_clear_confirmation = true;
        self.path_to_clear = path_type;
        iced::Task::none()
    }

    fn handle_confirm_path_clear(&mut self, path_type: String) -> iced::Task<AppMessage> {
        // 隐藏确认对话框
        self.show_path_clear_confirmation = false;

        // 执行清空操作
        let path_to_clear = match path_type.as_str() {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => return iced::Task::none(),
        };

        // 获取绝对路径
        let full_path = helpers::get_absolute_path(path_to_clear);

        // 尝试清空目录内容
        let result = if let Ok(entries) = std::fs::read_dir(&full_path) {
            let mut success_count = 0;
            let mut error_count = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let result = if path.is_file() {
                        std::fs::remove_file(&path)
                    } else if path.is_dir() {
                        std::fs::remove_dir_all(&path)
                    } else {
                        Ok(())
                    };

                    if result.is_ok() {
                        success_count += 1;
                    } else {
                        error_count += 1;
                    }
                }
            }

            if error_count == 0 {
                Ok(success_count)
            } else {
                Err(error_count)
            }
        } else {
            Err(0) // 目录不存在或无法访问
        };

        match result {
            Ok(count) => {
                // 清空成功，显示成功通知
                let message = if path_type == "data" {
                    format!("数据路径清空成功，删除了{}个项目", count)
                } else {
                    format!("缓存路径清空成功，删除了{}个项目", count)
                };
                return self.show_notification(message, super::NotificationType::Success);
            }
            Err(error_count) => {
                // 清空失败，显示错误通知
                let message = if path_type == "data" {
                    format!("数据路径清空失败，{}个项目未删除", error_count)
                } else {
                    format!("缓存路径清空失败，{}个项目未删除", error_count)
                };
                return self.show_notification(message, super::NotificationType::Error);
            }
        }
    }

    fn handle_cancel_path_clear(&mut self) -> iced::Task<AppMessage> {
        // 隐藏确认对话框，不执行清空操作
        self.show_path_clear_confirmation = false;
        iced::Task::none()
    }

    fn handle_restore_default_path(&mut self, path_type: String) -> iced::Task<AppMessage> {
        match path_type.as_str() {
            "data" => {
                // 恢复默认的数据路径 "data"
                self.config.set_data_path("data".to_string());
            }
            "cache" => {
                // 恢复默认的缓存路径 "cache"
                self.config.set_cache_path("cache".to_string());
            }
            _ => {}
        }
        iced::Task::none()
    }

    fn handle_wallhaven_api_key_changed(&mut self, api_key: String) -> iced::Task<AppMessage> {
        self.wallhaven_api_key = api_key;
        iced::Task::none()
    }

    fn handle_save_wallhaven_api_key(&mut self) -> iced::Task<AppMessage> {
        // 保存API KEY到配置文件
        let old_api_key = self.config.wallhaven.api_key.clone();
        let new_api_key = self.wallhaven_api_key.clone();

        // 对 API key 进行脱敏处理
        let mask_key = |key: &str| -> String {
            if key.is_empty() {
                "(空)".to_string()
            } else if key.len() >= 8 {
                format!("{}****{}", &key[..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            }
        };

        tracing::info!(
            "[设置] [Wallhaven API Key] 保存: {} -> {}",
            mask_key(&old_api_key),
            mask_key(&new_api_key)
        );
        self.config.set_wallhaven_api_key(new_api_key);

        // 如果 API Key 被清空，移除 NSFW 选项
        if self.wallhaven_api_key.is_empty() {
            // 移除 NSFW 位（第0位）
            self.online_state.purities &= !wallhaven::Purity::NSFW.bit_value();
            // 保存到配置文件
            self.online_state.save_to_config(&mut self.config);
        }

        // 显示成功通知
        self.show_notification(
            "WallHeven API KEY 保存成功".to_string(),
            super::NotificationType::Success,
        )
    }

    fn handle_proxy_protocol_changed(&mut self, protocol: String) -> iced::Task<AppMessage> {
        self.proxy_protocol = protocol;
        iced::Task::none()
    }

    fn handle_proxy_address_changed(&mut self, address: String) -> iced::Task<AppMessage> {
        self.proxy_address = address;
        iced::Task::none()
    }

    fn handle_proxy_port_changed(&mut self, port: u32) -> iced::Task<AppMessage> {
        // 数字输入框已经限制了范围为 1-65535
        self.proxy_port = port;
        iced::Task::none()
    }

    fn handle_save_proxy(&mut self) -> iced::Task<AppMessage> {
        // 检查地址和端口是否都设置且端口格式正确
        let is_address_valid = !self.proxy_address.trim().is_empty();
        let is_port_valid = self.proxy_port >= 1 && self.proxy_port <= 65535;

        if is_address_valid && is_port_valid {
            // 地址和端口都有效，保存代理设置
            let proxy_url = format!("{}://{}:{}", self.proxy_protocol, self.proxy_address, self.proxy_port);
            let old_proxy = self.config.global.proxy.clone();
            tracing::info!("[设置] [代理] 保存: {} -> {}", old_proxy, proxy_url);
            self.config.set_proxy(proxy_url);
            // 显示成功通知
            self.show_notification("代理设置保存成功".to_string(), super::NotificationType::Success)
        } else {
            // 地址或端口无效，保存为空字符串（相当于关闭代理）
            self.config.set_proxy(String::new());
            // 同时清空地址和端口输入框（端口显示为 1080）
            self.proxy_address = String::new();
            self.proxy_port = 1080;
            // 显示错误通知
            self.show_notification("格式错误，代理设置保存失败".to_string(), super::NotificationType::Error)
        }
    }

    fn handle_wallpaper_mode_selected(&mut self, mode: crate::utils::config::WallpaperMode) -> iced::Task<AppMessage> {
        let old_mode = self.config.wallpaper.mode;
        tracing::info!("[设置] [壁纸模式] 修改: {:?} -> {:?}", old_mode, mode);
        self.wallpaper_mode = mode;
        self.config.set_wallpaper_mode(mode);
        iced::Task::none()
    }

    fn handle_auto_change_mode_selected(
        &mut self,
        mode: crate::utils::config::WallpaperAutoChangeMode,
    ) -> iced::Task<AppMessage> {
        let old_mode = self.config.wallpaper.auto_change_mode;
        tracing::info!("[设置] [定时切换模式] 修改: {:?} -> {:?}", old_mode, mode);
        self.auto_change_mode = mode;
        self.config.wallpaper.auto_change_mode = mode;
        self.config.save_to_file();
        iced::Task::none()
    }

    fn handle_auto_change_interval_selected(
        &mut self,
        interval: crate::utils::config::WallpaperAutoChangeInterval,
    ) -> iced::Task<AppMessage> {
        let old_interval = self.config.wallpaper.auto_change_interval.clone();
        tracing::info!("[设置] [定时切换周期] 修改: {:?} -> {:?}", old_interval, interval);
        self.auto_change_interval = interval.clone();
        self.config.wallpaper.auto_change_interval = interval;

        // 根据选择的间隔启动或停止定时任务
        if matches!(
            self.auto_change_interval,
            crate::utils::config::WallpaperAutoChangeInterval::Off
        ) {
            // 选择关闭，停止定时任务
            self.auto_change_state.auto_change_enabled = false;
            self.auto_change_state.auto_change_timer = None;
            self.auto_change_state.auto_change_last_time = None;
            tracing::info!("[定时切换] [停止] 定时任务已停止");
        } else {
            // 选择其他选项，启动定时任务
            self.auto_change_state.auto_change_enabled = true;
            self.auto_change_state.auto_change_timer = Some(std::time::Instant::now());
            self.auto_change_state.auto_change_last_time = Some(std::time::Instant::now());

            // 计算并记录下次执行时间
            if let Some(minutes) = self.auto_change_interval.get_minutes() {
                let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                tracing::info!(
                    "[定时切换] [启动] 间隔: {}分钟, 下次执行时间: {}",
                    minutes,
                    next_time.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }

        self.config.save_to_file();
        iced::Task::none()
    }

    fn handle_custom_interval_minutes_changed(&mut self, minutes: u32) -> iced::Task<AppMessage> {
        // 限制最小值为1
        let minutes = if minutes < 1 { 1 } else { minutes };
        self.custom_interval_minutes = minutes;

        // 如果当前选中的是自定义选项，立即更新配置
        if matches!(
            self.auto_change_interval,
            crate::utils::config::WallpaperAutoChangeInterval::Custom(_)
        ) {
            // 同时更新 UI 状态和配置文件
            self.auto_change_interval = crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes);
            self.config.wallpaper.auto_change_interval =
                crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes);
            self.config.save_to_file();

            // 重置定时任务并记录下次执行时间
            if self.auto_change_state.auto_change_enabled {
                self.auto_change_state.auto_change_last_time = Some(std::time::Instant::now());
                let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                tracing::info!(
                    "[定时切换] [重置] 自定义间隔: {}分钟, 下次执行时间: {}",
                    minutes,
                    next_time.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }
        iced::Task::none()
    }

    fn handle_auto_change_query_changed(&mut self, query: String) -> iced::Task<AppMessage> {
        // 只更新临时状态，不保存到配置文件
        self.auto_change_query = query;
        iced::Task::none()
    }

    fn handle_save_auto_change_query(&mut self) -> iced::Task<AppMessage> {
        // 保存到配置文件
        let old_query = self.config.wallpaper.auto_change_query.clone();
        let new_query = self.auto_change_query.clone();
        tracing::info!(
            "[设置] [定时切换关键词] 保存: {} -> {}",
            if old_query.is_empty() { "(空)" } else { &old_query },
            if new_query.is_empty() { "(空)" } else { &new_query }
        );
        self.config.wallpaper.auto_change_query = new_query;
        self.config.save_to_file();

        // 显示保存成功通知
        let success_message = self.i18n.t("settings.save-success").to_string();
        self.show_notification(success_message, super::NotificationType::Success)
    }

    fn handle_language_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 切换语言选择器的展开/收起状态
        self.language_picker_expanded = !self.language_picker_expanded;
        iced::Task::none()
    }

    fn handle_language_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭语言选择器
        self.language_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_proxy_protocol_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 切换代理协议选择器的展开/收起状态
        self.proxy_protocol_picker_expanded = !self.proxy_protocol_picker_expanded;
        iced::Task::none()
    }

    fn handle_proxy_protocol_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭代理协议选择器
        self.proxy_protocol_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_theme_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 切换主题选择器的展开/收起状态
        self.theme_picker_expanded = !self.theme_picker_expanded;
        iced::Task::none()
    }

    fn handle_theme_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭主题选择器
        self.theme_picker_expanded = false;
        iced::Task::none()
    }
}
