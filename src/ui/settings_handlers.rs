// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use crate::services::wallhaven;
use crate::utils::{
    config::CloseAction,
    helpers,
    single_instance::{SingleInstanceGuard, WAKE_UP},
    startup,
};
use iced::wgpu::rwh::RawWindowHandle;
use iced::window;
use tracing::error;
use tracing::info;
use windows::Win32::Foundation::HWND;

impl App {
    /// 处理设置相关消息
    pub fn handle_settings_message(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        match msg {
            AppMessage::LanguageSelected(lang) => self.handle_language_selected(lang),
            AppMessage::PageSelected(page) => self.handle_page_selected(page),
            AppMessage::WindowResized(width, height) => self.handle_window_resized(width, height),
            AppMessage::ExecutePendingSave => self.execute_pending_save(),
            AppMessage::AutoStartupToggled(enabled) => self.handle_auto_startup_toggled(enabled),
            AppMessage::LoggingToggled(enabled) => self.handle_logging_toggled(enabled),
            AppMessage::CloseActionSelected(action) => self.handle_close_action_selected(action),
            AppMessage::WindowCloseRequested => self.handle_window_close_requested(),
            AppMessage::WindowFocused => self.handle_window_focused(),
            AppMessage::MinimizeToTray => self.handle_minimize_to_tray(),
            AppMessage::TrayIconClicked => self.handle_show_window(),
            AppMessage::TrayMenuEvent(id) => self.handle_tray_menu_event(id),
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
            AppMessage::ScrollToTop(scrollable_id) => self.handle_scroll_to_top(scrollable_id),
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
            AppMessage::ThemeSelected(theme) => self.handle_theme_selected(theme),
            AppMessage::AutoDetectColorModeTick => self.handle_detect_color_mode(),
            AppMessage::ShowCloseConfirmation => self.handle_show_close_confirmation(),
            AppMessage::CloseConfirmationResponse(action, remember_setting) => {
                self.handle_close_confirmation_response(action, remember_setting)
            }
            AppMessage::CloseConfirmationCancelled => self.handle_close_confirmation_cancelled(),
            AppMessage::ToggleRememberSetting(checked) => self.handle_toggle_remember_setting(checked),
            AppMessage::ShowNotification(message, notification_type) => {
                self.show_notification(message, notification_type)
            }
            AppMessage::HideNotification => self.handle_hide_notification(),
            AppMessage::AddToWallpaperHistory(path) => self.handle_add_to_wallpaper_history(path),
            AppMessage::TraySwitchPreviousWallpaper => self.handle_tray_switch_previous_wallpaper(),
            AppMessage::TraySwitchNextWallpaper => self.handle_tray_switch_next_wallpaper(),
            AppMessage::RemoveLastFromWallpaperHistory => self.handle_remove_last_from_wallpaper_history(),
            AppMessage::ExternalInstanceTriggered(payload) => self.handle_external_instance_triggered(payload),
            // 自定义标题栏消息
            AppMessage::TitleBarDrag => self.handle_title_bar_drag(),
            AppMessage::TitleBarMinimize => self.handle_title_bar_minimize(),
            AppMessage::TitleBarMaximize => self.handle_title_bar_maximize(),
            AppMessage::TitleBarClose => self.handle_window_close_requested(),
            // 窗口边缘调整大小消息
            AppMessage::ResizeWindow(direction) => self.handle_resize_window(direction),
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

    fn handle_page_selected(&mut self, page: super::ActivePage) -> iced::Task<AppMessage> {
        // 当切换离开在线壁纸页面时，取消正在进行的请求
        if self.active_page == super::ActivePage::OnlineWallpapers && page != super::ActivePage::OnlineWallpapers {
            self.online_state.cancel_and_new_context();
        }

        self.active_page = page;

        // 当切换到设置页面时，重置设置相关的临时状态
        if page == super::ActivePage::Settings {
            // 重置代理设置相关状态
            let (proxy_protocol, proxy_address, proxy_port) = App::parse_proxy_string(&self.config.global.proxy);
            self.proxy_protocol = proxy_protocol;
            self.proxy_address = proxy_address;
            self.proxy_port = proxy_port;
            if proxy_port == 0 {
                self.proxy_port = 1080;
            }

            // 重置API KEY设置状态
            self.wallhaven_api_key = self.config.wallhaven.api_key.clone();

            // 重置壁纸模式状态
            self.wallpaper_mode = self.config.wallpaper.mode;

            // 重置定时切换模式状态
            self.auto_change_mode = self.config.wallpaper.auto_change_mode;

            // 重置定时切换周期状态
            self.auto_change_interval = self.config.wallpaper.auto_change_interval;

            // 重置自定义分钟数状态
            self.custom_interval_minutes = self.config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30);

            // 重置定时切换关键词状态
            self.auto_change_query = self.config.wallpaper.auto_change_query.clone();

            // 滚动到顶部
            return iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("settings_scroll".to_string()));
        }

        // 每次切换到本地列表页面时，都重新加载壁纸
        if page == super::ActivePage::LocalList {
            // 重置本地状态，以便重新加载壁纸
            self.local_state = super::local::LocalState::default();
            return iced::Task::batch(vec![
                iced::Task::perform(async {}, |_| {
                    AppMessage::Local(super::local::LocalMessage::LoadWallpapers)
                }),
                iced::Task::perform(async {}, |_| {
                    AppMessage::ScrollToTop("local_wallpapers_scroll".to_string())
                }),
            ]);
        }

        // 每次切换到在线壁纸页面时，不重新加载数据
        // 仅在首次启动时通过 get_initial_tasks() 自动加载数据
        // 后续通过搜索按钮和刷新按钮手动重载
        if page == super::ActivePage::OnlineWallpapers {
            // 滚动到顶部
            return iced::Task::perform(async {}, |_| {
                AppMessage::ScrollToTop("online_wallpapers_scroll".to_string())
            });
        }

        // 对于其他页面切换，返回无任务
        iced::Task::none()
    }

    fn handle_window_resized(&mut self, width: u32, height: u32) -> iced::Task<AppMessage> {
        // 更新当前窗口宽度和高度，用于响应式布局和判断是否需要自动加载下一页
        self.current_window_width = width;
        self.current_window_height = height;
        // 如果宽度和高度都为 0，通常意味着窗口被最小化了
        self.is_visible = width > 0 && height > 0;
        // 暂存窗口大小，等待防抖处理
        self.pending_window_size = Some((width, height));
        // 在收到调整大小事件时，直接开启一个延迟任务
        self.debounce_timer = std::time::Instant::now();
        // 这个 Task 会在 300ms 后发出一条“执行保存”的消息
        return iced::Task::perform(tokio::time::sleep(std::time::Duration::from_millis(300)), |_| {
            AppMessage::ExecutePendingSave
        });
    }

    /// 处理延迟保存
    fn execute_pending_save(&mut self) -> iced::Task<AppMessage> {
        let elapsed = self.debounce_timer.elapsed();
        if elapsed >= std::time::Duration::from_millis(300) {
            // 只有当存在 pending 数据时才保存，保存完立即 take() 掉
            if let Some((width, height)) = self.pending_window_size.take() {
                if width > 0 && height > 0 {
                    // 同步窗口大小到配置文件
                    self.config.update_window_size(width, height);
                }
            }
        }
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

    fn handle_window_close_requested(&mut self) -> iced::Task<AppMessage> {
        // 根据配置处理关闭请求
        match self.config.global.close_action {
            CloseAction::MinimizeToTray => {
                // 最小化到托盘 - 发送一个MinimizeToTray消息到主函数
                return iced::Task::perform(async {}, |_| AppMessage::MinimizeToTray);
            }
            CloseAction::CloseApp => {
                // 直接关闭应用
                return iced::exit();
            }
            CloseAction::Ask => {
                // 显示关闭确认对话框
                return iced::Task::perform(async {}, |_| AppMessage::ShowCloseConfirmation);
            }
        }
    }

    fn handle_window_focused(&mut self) -> iced::Task<AppMessage> {
        // 更新窗口状态为已聚焦
        self.is_visible = true;
        iced::Task::none()
    }

    fn handle_minimize_to_tray(&mut self) -> iced::Task<AppMessage> {
        self.is_visible = false;
        // 获取 ID 后设置模式为隐藏
        window::oldest().and_then(|id| window::set_mode(id, window::Mode::Hidden))
    }

    fn handle_tray_menu_event(&mut self, id: String) -> iced::Task<AppMessage> {
        match id.as_str() {
            "tray_show" => {
                // 显示窗口并检测状态，如果最小化或不在前台则置顶
                return self.handle_show_window();
            }
            "tray_switch_previous" => {
                // 切换上一张壁纸
                return iced::Task::done(AppMessage::TraySwitchPreviousWallpaper);
            }
            "tray_switch_next" => {
                // 切换下一张壁纸
                return iced::Task::done(AppMessage::TraySwitchNextWallpaper);
            }
            "tray_settings" => {
                // 打开设置窗口
                self.active_page = super::ActivePage::Settings;
                return self.handle_show_window();
            }
            "tray_quit" => {
                // 真正退出程序
                return iced::exit();
            }
            _ => {}
        }

        iced::Task::none()
    }

    fn handle_show_window(&mut self) -> iced::Task<AppMessage> {
        // 显示窗口并检测状态，如果最小化或不在前台则置顶
        window::oldest().and_then(move |window_id| {
            iced::Task::batch(vec![
                // 先设置窗口为 Windowed 模式（从 Hidden 恢复）
                window::set_mode(window_id, window::Mode::Windowed),
                // 然后检测窗口状态并置顶
                window::run(window_id, move |mw| {
                    if let Ok(handle) = mw.window_handle() {
                        if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                            let hwnd = HWND(win32_handle.hwnd.get() as _);

                            // 使用 Windows API 检测窗口状态并置顶
                            let is_minimized = crate::utils::window_utils::is_window_minimized(hwnd);
                            let is_foreground = crate::utils::window_utils::is_window_foreground(hwnd);

                            // 如果窗口最小化或不在前台，则恢复并置顶
                            if is_minimized || !is_foreground {
                                tracing::info!(
                                    "[显示窗口] 窗口状态 - 最小化: {}, 前台: {}, 执行恢复和置顶",
                                    is_minimized,
                                    is_foreground
                                );
                                let success = crate::utils::window_utils::restore_and_bring_to_front(hwnd);
                                tracing::info!(" [显示窗口] 置顶操作结果: {}", success);
                            } else {
                                tracing::info!("[显示窗口] 窗口已在前台，无需置顶");
                            }
                        }
                    }
                })
                .map(|_| AppMessage::None),
            ])
        })
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
            return iced::Task::perform(super::async_tasks::select_folder_async(), |selected_path| {
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
            return iced::Task::perform(super::async_tasks::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    AppMessage::CachePathSelected(selected_path)
                } else {
                    AppMessage::CachePathSelected("".to_string()) // 用户取消选择
                }
            });
        } else if path == "SELECT_DATA_PATH" {
            // 这是用户点击数据路径输入框时的原始消息，触发异步任务
            return iced::Task::perform(super::async_tasks::select_folder_async(), |selected_path| {
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

    fn handle_scroll_to_top(&mut self, scrollable_id: String) -> iced::Task<AppMessage> {
        // 滚动到指定滚动组件的顶部
        use iced::widget::operation;
        operation::scroll_by(
            scrollable_id,
            iced::widget::scrollable::AbsoluteOffset { x: 0.0, y: 0.0 },
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

    fn handle_show_close_confirmation(&mut self) -> iced::Task<AppMessage> {
        self.show_close_confirmation = true;
        iced::Task::none()
    }

    fn handle_close_confirmation_response(
        &mut self,
        action: super::CloseConfirmationAction,
        remember_setting: bool,
    ) -> iced::Task<AppMessage> {
        // 隐藏对话框
        self.show_close_confirmation = false;

        // 如果勾选了记住设置，则更新配置
        if remember_setting {
            let new_close_action = match action {
                super::CloseConfirmationAction::MinimizeToTray => CloseAction::MinimizeToTray,
                super::CloseConfirmationAction::CloseApp => CloseAction::CloseApp,
            };
            self.config.set_close_action(new_close_action);
        }

        // 根据选择执行相应操作
        match action {
            super::CloseConfirmationAction::MinimizeToTray => {
                return iced::Task::perform(async {}, |_| AppMessage::MinimizeToTray);
            }
            super::CloseConfirmationAction::CloseApp => {
                return iced::exit();
            }
        }
    }

    fn handle_close_confirmation_cancelled(&mut self) -> iced::Task<AppMessage> {
        // 隐藏对话框，不执行任何操作
        self.show_close_confirmation = false;
        iced::Task::none()
    }

    fn handle_toggle_remember_setting(&mut self, checked: bool) -> iced::Task<AppMessage> {
        self.remember_close_setting = checked;
        iced::Task::none()
    }

    fn handle_hide_notification(&mut self) -> iced::Task<AppMessage> {
        self.show_notification = false;
        iced::Task::none()
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
        iced::Task::done(AppMessage::ShowNotification(
            success_message,
            super::NotificationType::Success,
        ))
    }

    /// 处理添加壁纸到历史记录
    fn handle_add_to_wallpaper_history(&mut self, path: String) -> iced::Task<AppMessage> {
        // 检查历史记录中是否已存在该路径，如果存在则先移除
        if let Some(pos) = self.wallpaper_history.iter().position(|p| p == &path) {
            self.wallpaper_history.remove(pos);
        }

        // 记录路径用于日志输出
        let path_for_log = path.clone();

        // 添加到历史记录末尾
        self.wallpaper_history.push(path);

        // 限制历史记录最多50条
        if self.wallpaper_history.len() > 50 {
            self.wallpaper_history.remove(0);
        }

        tracing::info!(
            "[壁纸历史] 添加记录: {}, 当前记录数: {}",
            path_for_log,
            self.wallpaper_history.len()
        );

        // 更新托盘菜单项的启用状态
        self.tray_manager
            .update_switch_previous_item(self.wallpaper_history.len());

        iced::Task::none()
    }

    /// 处理托盘菜单切换上一张壁纸
    fn handle_tray_switch_previous_wallpaper(&mut self) -> iced::Task<AppMessage> {
        // 检查历史记录是否为空
        if self.wallpaper_history.is_empty() {
            tracing::warn!("[托盘菜单] 壁纸历史记录为空，无法切换上一张");
            return iced::Task::none();
        }

        // 查找上一张壁纸（历史记录中的倒数第二条）
        if self.wallpaper_history.len() < 2 {
            tracing::warn!("[托盘菜单] 壁纸历史记录不足2条，无法切换上一张");
            return iced::Task::none();
        }

        let previous_wallpaper = self.wallpaper_history[self.wallpaper_history.len() - 2].clone();

        // 设置壁纸
        let wallpaper_mode = self.config.wallpaper.mode;

        tracing::info!("[托盘菜单] 切换上一张壁纸: {}", previous_wallpaper);

        // 提前获取翻译文本，避免线程安全问题
        let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

        iced::Task::perform(
            super::async_tasks::async_set_wallpaper(previous_wallpaper.clone(), wallpaper_mode),
            move |result| match result {
                Ok(_) => {
                    // 切换成功，将当前壁纸从历史记录末尾移除
                    AppMessage::RemoveLastFromWallpaperHistory
                }
                Err(e) => {
                    AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error)
                }
            },
        )
    }

    /// 处理托盘菜单切换下一张壁纸
    fn handle_tray_switch_next_wallpaper(&mut self) -> iced::Task<AppMessage> {
        // 提前获取翻译文本，避免线程安全问题
        let no_valid_wallpapers_message = self.i18n.t("local-list.no-valid-wallpapers").to_string();

        // 根据定时切换模式执行不同的逻辑
        match self.config.wallpaper.auto_change_mode {
            crate::utils::config::WallpaperAutoChangeMode::Local => {
                // 本地模式：获取支持的图片文件列表
                let data_path = self.config.data.data_path.clone();
                iced::Task::perform(
                    super::async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            // 获取到图片列表后，立即尝试设置随机壁纸
                            if paths.is_empty() {
                                AppMessage::ShowNotification(
                                    no_valid_wallpapers_message,
                                    super::NotificationType::Error,
                                )
                            } else {
                                // 发送一个消息来触发设置随机壁纸
                                AppMessage::AutoChange(
                                    super::auto_change::AutoChangeMessage::GetSupportedImagesSuccess(paths),
                                )
                            }
                        }
                        Err(e) => {
                            let error_message = format!("获取壁纸列表失败: {}", e);
                            AppMessage::ShowNotification(error_message, super::NotificationType::Error)
                        }
                    },
                )
            }
            crate::utils::config::WallpaperAutoChangeMode::Online => {
                // 在线模式：从Wallhaven获取随机壁纸
                let config = self.config.clone();
                let auto_change_running = self.auto_change_running.clone();
                iced::Task::perform(
                    super::async_tasks::async_set_random_online_wallpaper(config, auto_change_running),
                    |result| match result {
                        Ok(path) => AppMessage::AutoChange(
                            super::auto_change::AutoChangeMessage::SetRandomWallpaperSuccess(path),
                        ),
                        Err(e) => {
                            let error_message = format!("设置壁纸失败: {}", e);
                            AppMessage::ShowNotification(error_message, super::NotificationType::Error)
                        }
                    },
                )
            }
        }
    }

    /// 处理从历史记录末尾移除壁纸
    fn handle_remove_last_from_wallpaper_history(&mut self) -> iced::Task<AppMessage> {
        // 从历史记录末尾移除壁纸
        if let Some(removed) = self.wallpaper_history.pop() {
            tracing::info!(
                "[壁纸历史] 移除记录: {}, 当前记录数: {}",
                removed,
                self.wallpaper_history.len()
            );
        }

        // 更新托盘菜单项的启用状态
        self.tray_manager
            .update_switch_previous_item(self.wallpaper_history.len());

        iced::Task::none()
    }

    fn handle_external_instance_triggered(&mut self, payload: String) -> iced::Task<AppMessage> {
        info!("外部实例触发事件: {}", payload);

        let show_window_task = if payload.contains(WAKE_UP) {
            self.handle_show_window()
        } else {
            iced::Task::none()
        };
        let next_listen_task =
            iced::Task::perform(SingleInstanceGuard::listen(), AppMessage::ExternalInstanceTriggered);

        iced::Task::batch(vec![show_window_task, next_listen_task])
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

    fn handle_theme_selected(&mut self, theme: crate::utils::config::Theme) -> iced::Task<AppMessage> {
        let old_theme = self.config.global.theme;
        tracing::info!("[设置] [主题] 修改: {:?} -> {:?}", old_theme, theme);

        // 更新配置
        self.config.global.theme = theme;
        self.config.save_to_file();

        // 关闭选择器
        self.theme_picker_expanded = false;

        self.auto_change_state.auto_detect_color_mode = theme == crate::utils::config::Theme::Auto;

        self.toggle_theme(theme)
    }

    fn toggle_theme(&mut self, theme: crate::utils::config::Theme) -> iced::Task<AppMessage> {
        use crate::utils::config::Theme;

        // 根据主题类型决定是否需要切换
        match theme {
            Theme::Dark => {
                // 暗色主题：如果当前不是暗色，则切换
                if !self.theme_config.is_dark() {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    tracing::info!("[设置] [主题] 切换到: {}", theme_name);
                }
            }
            Theme::Light => {
                // 亮色主题：如果当前是暗色，则切换
                if self.theme_config.is_dark() {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    tracing::info!("[设置] [主题] 切换到: {}", theme_name);
                }
            }
            Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = crate::utils::window_utils::get_system_color_mode();
                tracing::info!(
                    "[设置] [主题] 自动模式，系统主题: {}",
                    if is_system_dark { "深色" } else { "浅色" }
                );

                // 如果当前主题与系统主题不一致，则切换
                if self.theme_config.is_dark() != is_system_dark {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    tracing::info!("[设置] [主题] 切换到: {}", theme_name);
                }
            }
        }
        iced::Task::none()
    }

    fn handle_detect_color_mode(&mut self) -> iced::Task<AppMessage> {
        let system_is_dark = crate::utils::window_utils::get_system_color_mode();
        if system_is_dark != self.theme_config.is_dark() {
            if system_is_dark {
                return self.toggle_theme(crate::utils::config::Theme::Dark);
            } else {
                return self.toggle_theme(crate::utils::config::Theme::Light);
            }
        }
        iced::Task::none()
    }

    // ============================================================================
    // 自定义标题栏消息处理方法
    // ============================================================================

    /// 处理标题栏拖拽消息
    fn handle_title_bar_drag(&mut self) -> iced::Task<AppMessage> {
        window::oldest().and_then(move |id| window::drag(id))
    }

    /// 启用窗口边缘调整大小功能并添加窗口阴影
    pub fn enable_window_resize(&self) -> iced::Task<AppMessage> {
        window::oldest().and_then(move |id| {
            window::run(id, move |mw| {
                if let Ok(handle) = mw.window_handle() {
                    if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                        let hwnd = HWND(win32_handle.hwnd.get() as _);

                        unsafe {
                            use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
                            use windows::Win32::UI::Controls::MARGINS;
                            use windows::Win32::UI::WindowsAndMessaging::{
                                GWL_STYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_SIZEBOX, WS_THICKFRAME,
                            };

                            // 获取当前窗口样式
                            let style = GetWindowLongPtrW(hwnd, GWL_STYLE);

                            // 添加 WS_THICKFRAME 和 WS_SIZEBOX 样式以启用窗口边缘调整大小和边框
                            let new_style = style | WS_THICKFRAME.0 as isize | WS_SIZEBOX.0 as isize;

                            // 设置新的窗口样式
                            let _ = SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);

                            // 启用窗口阴影效果
                            // 将边距设置为 -1，表示整个窗口都有阴影
                            let margins = MARGINS {
                                cxLeftWidth: -1,
                                cxRightWidth: -1,
                                cyTopHeight: -1,
                                cyBottomHeight: -1,
                            };
                            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
                        }
                    }
                }
            })
            .map(|_| AppMessage::None)
        })
    }

    /// 处理标题栏最小化消息
    fn handle_title_bar_minimize(&mut self) -> iced::Task<AppMessage> {
        window::oldest().and_then(|id: iced::window::Id| window::minimize(id, true).map(|_: ()| AppMessage::None))
    }

    /// 处理标题栏最大化/还原消息
    fn handle_title_bar_maximize(&mut self) -> iced::Task<AppMessage> {
        let is_maximized = !self.is_maximized;
        self.is_maximized = is_maximized;

        // 当窗口从最大化状态还原时,需要重新应用窗口样式以确保拖拽功能正常
        window::oldest()
            .and_then(move |id: iced::window::Id| window::maximize(id, is_maximized).map(|_: ()| AppMessage::None))
            .chain(if !is_maximized {
                // 窗口还原后,重新启用窗口调整大小样式
                self.enable_window_resize()
            } else {
                iced::Task::none()
            })
    }

    // ============================================================================
    // 窗口边缘调整大小消息处理方法
    // ============================================================================

    /// 处理窗口边缘调整大小消息
    fn handle_resize_window(&mut self, direction: iced::window::Direction) -> iced::Task<AppMessage> {
        window::oldest().and_then(move |id: iced::window::Id| window::drag_resize(id, direction))
    }
}
