use super::App;
use super::AppMessage;
use super::common;
use crate::utils::config::CloseAction;
use crate::utils::startup;
use iced::window;
use tracing::error;

impl App {
    /// 处理设置相关消息
    pub fn handle_settings_message(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        match msg {
            AppMessage::LanguageSelected(lang) => self.handle_language_selected(lang),
            AppMessage::PageSelected(page) => self.handle_page_selected(page),
            AppMessage::WindowResized(width, height) => self.handle_window_resized(width, height),
            AppMessage::DebounceTimer => self.handle_debounce_timer(),
            AppMessage::AutoStartupToggled(enabled) => self.handle_auto_startup_toggled(enabled),
            AppMessage::CloseActionSelected(action) => self.handle_close_action_selected(action),
            AppMessage::WindowCloseRequested => self.handle_window_close_requested(),
            AppMessage::MinimizeToTray => self.handle_minimize_to_tray(),
            AppMessage::TrayIconClicked => self.handle_tray_icon_clicked(),
            AppMessage::TrayMenuEvent(id) => self.handle_tray_menu_event(id),
            AppMessage::OpenUrl(url) => self.handle_open_url(url),
            AppMessage::DataPathSelected(path) => self.handle_data_path_selected(path),
            AppMessage::CachePathSelected(path) => self.handle_cache_path_selected(path),
            AppMessage::OpenPath(path_type) => self.handle_open_path(path_type),
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
            AppMessage::ShowCloseConfirmation => self.handle_show_close_confirmation(),
            AppMessage::CloseConfirmationResponse(action, remember_setting) => self.handle_close_confirmation_response(action, remember_setting),
            AppMessage::CloseConfirmationCancelled => self.handle_close_confirmation_cancelled(),
            AppMessage::ToggleRememberSetting(checked) => self.handle_toggle_remember_setting(checked),
            AppMessage::ShowNotification(message, notification_type) => self.handle_show_notification(message, notification_type),
            AppMessage::HideNotification => self.handle_hide_notification(),
            _ => iced::Task::none(),
        }
    }

    fn handle_language_selected(&mut self, lang: String) -> iced::Task<AppMessage> {
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

            // 重置API KEY设置状态
            self.wallhaven_api_key = self.config.wallhaven.api_key.clone();

            // 滚动到顶部
            return iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("settings_scroll".to_string()));
        }

        // 每次切换到本地列表页面时，都重新加载壁纸
        if page == super::ActivePage::LocalList {
            // 重置本地状态，以便重新加载壁纸
            self.local_state = super::local::LocalState::default();
            return iced::Task::batch(vec![
                iced::Task::perform(async {}, |_| AppMessage::Local(super::local::LocalMessage::LoadWallpapers)),
                iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("local_wallpapers_scroll".to_string())),
            ]);
        }

        // 每次切换到在线壁纸页面时，不重新加载数据
        // 仅在首次启动时通过 get_initial_tasks() 自动加载数据
        // 后续通过搜索按钮和刷新按钮手动重载
        if page == super::ActivePage::OnlineWallpapers {
            // 滚动到顶部
            return iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("online_wallpapers_scroll".to_string()));
        }

        // 对于其他页面切换，返回无任务
        iced::Task::none()
    }

    fn handle_window_resized(&mut self, width: u32, height: u32) -> iced::Task<AppMessage> {
        // 更新当前窗口宽度和高度，用于响应式布局和判断是否需要自动加载下一页
        self.current_window_width = width;
        self.current_window_height = height;
        // 暂存窗口大小，等待防抖处理
        self.pending_window_size = Some((width, height));
        self.debounce_timer = std::time::Instant::now();
        iced::Task::none()
    }

    fn handle_debounce_timer(&mut self) -> iced::Task<AppMessage> {
        use tray_icon::{TrayIconEvent, menu::MenuEvent};

        // 托盘事件轮询
        // 检查菜单点击
        if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            // 这里通过 Task::done 立即在下一个 loop 处理具体的菜单逻辑
            return iced::Task::done(AppMessage::TrayMenuEvent(menu_event.id.0));
        }

        // 检查图标双击
        if let Ok(tray_event) = TrayIconEvent::receiver().try_recv() {
            if let TrayIconEvent::DoubleClick { .. } = tray_event {
                return iced::Task::done(AppMessage::TrayIconClicked);
            }
        }

        // 检查是否需要执行延迟的保存操作
        let elapsed = self.debounce_timer.elapsed();
        if elapsed >= std::time::Duration::from_millis(300) {
            // 保存窗口大小
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
        self.config.set_auto_startup(enabled);
        if let Err(e) = startup::set_auto_startup(enabled) {
            error!("设置开机启动失败: {}", e);
        }
        iced::Task::none()
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

    fn handle_minimize_to_tray(&mut self) -> iced::Task<AppMessage> {
        // 获取 ID 后设置模式为隐藏
        window::oldest().and_then(|id| window::set_mode(id, window::Mode::Hidden))
    }

    fn handle_tray_icon_clicked(&mut self) -> iced::Task<AppMessage> {
        window::oldest().and_then(|id| {
            iced::Task::batch(vec![
                window::set_mode(id, window::Mode::Windowed), // 显示程序窗口
                window::gain_focus(id),                       // 强制拉回前台
            ])
        })
    }

    fn handle_tray_menu_event(&mut self, id: String) -> iced::Task<AppMessage> {
        match id.as_str() {
            "tray_show" => {
                return window::oldest().and_then(|id| window::set_mode(id, window::Mode::Windowed));
            }
            "tray_settings" => {
                // 打开设置窗口
                self.active_page = super::ActivePage::Settings;
                return window::oldest().and_then(|id| window::set_mode(id, window::Mode::Windowed));
            }
            "tray_quit" => {
                // 真正退出程序
                return iced::exit();
            }
            _ => {}
        }

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

        let full_path = common::get_absolute_path(path_to_open);

        if let Err(e) = open::that(&full_path) {
            error!("Failed to open path {}: {}", full_path, e);
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
        let full_path = common::get_absolute_path(path_to_clear);

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

            if error_count == 0 { Ok(success_count) } else { Err(error_count) }
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
        self.config.set_wallhaven_api_key(self.wallhaven_api_key.clone());
        // 显示成功通知
        self.show_notification("WallHeven API KEY 保存成功".to_string(), super::NotificationType::Success)
    }

    fn handle_scroll_to_top(&mut self, _scrollable_id: String) -> iced::Task<AppMessage> {
        // 返回无任务，目前滚动到顶部功能依赖于不同的ID来实现隔离
        iced::Task::none()
    }

    fn handle_proxy_protocol_changed(&mut self, protocol: String) -> iced::Task<AppMessage> {
        self.proxy_protocol = protocol;
        iced::Task::none()
    }

    fn handle_proxy_address_changed(&mut self, address: String) -> iced::Task<AppMessage> {
        self.proxy_address = address;
        iced::Task::none()
    }

    fn handle_proxy_port_changed(&mut self, port: String) -> iced::Task<AppMessage> {
        self.proxy_port = port;
        iced::Task::none()
    }

    fn handle_save_proxy(&mut self) -> iced::Task<AppMessage> {
        // 检查地址和端口是否都设置且端口格式正确
        let is_address_valid = !self.proxy_address.trim().is_empty();
        let is_port_valid = if let Ok(port) = self.proxy_port.parse::<u16>() {
            port != 0 // u16的范围是0-65535，所以只需检查不为0
        } else {
            false // 端口不是有效数字
        };

        if is_address_valid && is_port_valid {
            // 地址和端口都有效，保存代理设置
            let proxy_url = format!("{}://{}:{}", self.proxy_protocol, self.proxy_address, self.proxy_port);
            self.config.set_proxy(proxy_url);
            // 显示成功通知
            self.show_notification("代理设置保存成功".to_string(), super::NotificationType::Success)
        } else {
            // 地址或端口无效，保存为空字符串（相当于关闭代理）
            self.config.set_proxy(String::new());
            // 同时清空地址和端口输入框
            self.proxy_address = String::new();
            self.proxy_port = String::new();
            // 显示错误通知
            self.show_notification("格式错误，代理设置保存失败".to_string(), super::NotificationType::Error)
        }
    }

    fn handle_show_close_confirmation(&mut self) -> iced::Task<AppMessage> {
        self.show_close_confirmation = true;
        iced::Task::none()
    }

    fn handle_close_confirmation_response(&mut self, action: super::CloseConfirmationAction, remember_setting: bool) -> iced::Task<AppMessage> {
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

    fn handle_show_notification(&mut self, message: String, notification_type: super::NotificationType) -> iced::Task<AppMessage> {
        self.show_notification(message, notification_type)
    }

    fn handle_hide_notification(&mut self) -> iced::Task<AppMessage> {
        self.show_notification = false;
        iced::Task::none()
    }
}
