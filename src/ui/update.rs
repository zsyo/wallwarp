use super::App;
use super::AppMessage;
use crate::ui::ActivePage;
use crate::utils::startup;
use iced::window;
use std::time::Duration;
use tray_icon::{TrayIconEvent, menu::MenuEvent};

impl App {
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        use iced::event;
        use iced::time;
        use iced::window;
        use std::time::Duration;

        iced::Subscription::batch([
            event::listen_with(|event, _status, _loop_status| match event {
                iced::Event::Window(window::Event::Resized(size)) => Some(
                    AppMessage::WindowResized(size.width as u32, size.height as u32),
                ),
                iced::Event::Window(window::Event::Moved(position)) => Some(
                    AppMessage::WindowMoved(position.x as i32, position.y as i32),
                ),
                iced::Event::Window(window::Event::CloseRequested) => {
                    // 发送一个关闭请求消息，让App处理
                    Some(AppMessage::WindowCloseRequested)
                }
                _ => None,
            }),
            time::every(Duration::from_millis(50)).map(|_| AppMessage::DebounceTimer),
        ])
    }

    pub fn update(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        match msg {
            AppMessage::LanguageSelected(lang) => {
                self.i18n.set_language(lang.clone());
                // 同时更新配置
                self.config.set_language(lang);
            }
            AppMessage::PageSelected(page) => self.active_page = page,
            AppMessage::WindowResized(width, height) => {
                // 暂存窗口大小，等待防抖处理
                self.pending_window_size = Some((width, height));
                self.debounce_timer = std::time::Instant::now();
            }
            AppMessage::WindowMoved(x, y) => {
                // 暂存窗口位置，等待防抖处理
                self.pending_window_position = Some((x, y));
                self.debounce_timer = std::time::Instant::now();
            }
            AppMessage::DebounceTimer => {
                // 托盘事件轮询
                // 检查菜单点击
                if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
                    // 这里通过 Task::done 立即在下一个 loop 处理具体的菜单逻辑
                    println!("menu_event: received, {menu_event:?}");
                    return iced::Task::done(AppMessage::TrayMenuEvent(menu_event.id.0));
                }

                // 检查图标双击
                if let Ok(tray_event) = TrayIconEvent::receiver().try_recv() {
                    if let TrayIconEvent::DoubleClick { .. } = tray_event {
                        println!("tray_event: received, {tray_event:?}");
                        return iced::Task::done(AppMessage::TrayIconClicked);
                    }
                }

                // 检查是否需要执行延迟的保存操作
                let elapsed = self.debounce_timer.elapsed();
                if elapsed >= Duration::from_millis(300) {
                    // 保存窗口大小
                    if let Some((width, height)) = self.pending_window_size.take() {
                        if width > 0 && height > 0 {
                            // 同步窗口大小到配置文件
                            self.config.update_window_size(width, height);
                            println!("窗口尺寸已同步至配置文件, 宽: {width}, 高: {height}");
                        }
                    }

                    // 保存窗口位置
                    if let Some((x, y)) = self.pending_window_position.take() {
                        if x >= 0 || y >= 0 {
                            self.config.update_window_position(x, y);
                            println!("窗口位置已同步至配置文件, X: {x}, Y: {y}");
                        }
                    }
                }
            }
            AppMessage::AutoStartupToggled(enabled) => {
                self.config.set_auto_startup(enabled);
                if let Err(e) = startup::set_auto_startup(enabled) {
                    eprintln!("设置开机启动失败: {}", e);
                }
            }
            AppMessage::CloseActionSelected(action) => {
                self.config.set_close_action(action);
            }
            AppMessage::WindowCloseRequested => {
                // 根据配置处理关闭请求
                match self.config.global.close_action {
                    crate::utils::config::CloseAction::MinimizeToTray => {
                        // 最小化到托盘 - 发送一个MinimizeToTray消息到主函数
                        return iced::Task::perform(async {}, |_| AppMessage::MinimizeToTray);
                    }
                    crate::utils::config::CloseAction::CloseApp => {
                        // 直接关闭应用
                        return iced::exit();
                    }
                    crate::utils::config::CloseAction::Ask => {
                        // 显示关闭确认对话框
                        return iced::Task::perform(async {}, |_| AppMessage::ShowCloseConfirmation);
                    }
                }
            }
            AppMessage::MinimizeToTray => {
                // 获取 ID 后设置模式为隐藏
                return window::oldest().and_then(|id| window::set_mode(id, window::Mode::Hidden));
            }
            AppMessage::TrayIconClicked => {
                return window::oldest().and_then(|id| {
                    iced::Task::batch(vec![
                        window::set_mode(id, window::Mode::Windowed), // 显示程序窗口
                        window::gain_focus(id),                       // 强制拉回前台
                    ])
                });
            }
            AppMessage::TrayMenuEvent(id) => match id.as_str() {
                "tray_show" => {
                    return window::oldest()
                        .and_then(|id| window::set_mode(id, window::Mode::Windowed));
                }
                "tray_settings" => {
                    // 打开设置窗口
                    self.active_page = ActivePage::Settings;
                    return window::oldest()
                        .and_then(|id| window::set_mode(id, window::Mode::Windowed));
                }
                "tray_quit" => {
                    // 真正退出程序
                    return iced::exit();
                }
                _ => {}
            },
            AppMessage::OpenUrl(url) => {
                if let Err(e) = open::that(&url) {
                    eprintln!("Failed to open URL {}: {}", url, e);
                }
            }
            AppMessage::DataPathSelected(path) => {
                if !path.is_empty() && path != "SELECT_DATA_PATH" {
                    // 这是异步任务返回的实际路径
                    self.config.set_data_path(path);
                } else if path == "SELECT_DATA_PATH" {
                    // 这是用户点击按钮时的原始消息，触发异步任务
                    return iced::Task::perform(select_data_path_async(), |selected_path| {
                        if !selected_path.is_empty() {
                            AppMessage::DataPathSelected(selected_path)
                        } else {
                            AppMessage::DataPathSelected("".to_string()) // 用户取消选择
                        }
                    });
                }
            }
            AppMessage::CachePathSelected(path) => {
                if !path.is_empty() && path != "SELECT_CACHE_PATH" && path != "SELECT_DATA_PATH" {
                    // 这是异步任务返回的实际路径
                    self.config.set_cache_path(path);
                } else if path == "SELECT_CACHE_PATH" {
                    // 这是用户点击按钮时的原始消息，触发异步任务
                    return iced::Task::perform(select_cache_path_async(), |selected_path| {
                        if !selected_path.is_empty() {
                            AppMessage::CachePathSelected(selected_path)
                        } else {
                            AppMessage::CachePathSelected("".to_string()) // 用户取消选择
                        }
                    });
                } else if path == "SELECT_DATA_PATH" {
                    // 这是用户点击数据路径输入框时的原始消息，触发异步任务
                    return iced::Task::perform(select_data_path_async(), |selected_path| {
                        if !selected_path.is_empty() {
                            AppMessage::DataPathSelected(selected_path)
                        } else {
                            AppMessage::DataPathSelected("".to_string()) // 用户取消选择
                        }
                    });
                }
            }
            AppMessage::OpenPath(path_type) => {
                let path_to_open = match path_type.as_str() {
                    "data" => &self.config.data.data_path,
                    "cache" => &self.config.data.cache_path,
                    _ => return iced::Task::none(),
                };

                // 获取绝对路径并打开
                let current_dir =
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let full_path = if std::path::PathBuf::from(path_to_open).is_absolute() {
                    path_to_open.clone()
                } else {
                    current_dir.join(path_to_open).to_string_lossy().to_string()
                };

                if let Err(e) = open::that(&full_path) {
                    eprintln!("Failed to open path {}: {}", full_path, e);
                }
            }
            AppMessage::ClearPath(path_type) => {
                let path_to_clear = match path_type.as_str() {
                    "data" => &self.config.data.data_path,
                    "cache" => &self.config.data.cache_path,
                    _ => return iced::Task::none(),
                };

                // 获取绝对路径并清空内容
                let current_dir =
                    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let full_path = if std::path::PathBuf::from(path_to_clear).is_absolute() {
                    path_to_clear.clone()
                } else {
                    current_dir
                        .join(path_to_clear)
                        .to_string_lossy()
                        .to_string()
                };

                // 尝试清空目录内容
                if let Ok(entries) = std::fs::read_dir(&full_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                let _ = std::fs::remove_file(&path);
                            } else if path.is_dir() {
                                let _ = std::fs::remove_dir_all(&path);
                            }
                        }
                    }
                }
            }
            AppMessage::RestoreDefaultPath(path_type) => {
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
            }
            AppMessage::WallhavenApiKeyChanged(api_key) => {
                self.config.set_wallhaven_api_key(api_key);
            }
            AppMessage::ProxyProtocolChanged(protocol) => {
                self.proxy_protocol = protocol;
            }
            AppMessage::ProxyAddressChanged(address) => {
                self.proxy_address = address;
            }
            AppMessage::ProxyPortChanged(port) => {
                self.proxy_port = port;
            }
            AppMessage::SaveProxy => {
                // 检查地址和端口是否都设置且端口格式正确
                let is_address_valid = !self.proxy_address.trim().is_empty();
                let is_port_valid = if let Ok(port) = self.proxy_port.parse::<u16>() {
                    port != 0  // u16的范围是0-65535，所以只需检查不为0
                } else {
                    false  // 端口不是有效数字
                };

                if is_address_valid && is_port_valid {
                    // 地址和端口都有效，保存代理设置
                    let proxy_url = format!("{}://{}:{}", self.proxy_protocol, self.proxy_address, self.proxy_port);
                    self.config.set_proxy(proxy_url);
                } else {
                    // 地址或端口无效，保存为空字符串（相当于关闭代理）
                    self.config.set_proxy(String::new());
                    // 同时清空地址和端口输入框
                    self.proxy_address = String::new();
                    self.proxy_port = String::new();
                }
            }
            AppMessage::ShowCloseConfirmation => {
                self.show_close_confirmation = true;
            }
            AppMessage::CloseConfirmationResponse(action, remember_setting) => {
                // 隐藏对话框
                self.show_close_confirmation = false;
                
                // 如果勾选了记住设置，则更新配置
                if remember_setting {
                    let new_close_action = match action {
                        super::CloseConfirmationAction::MinimizeToTray => crate::utils::config::CloseAction::MinimizeToTray,
                        super::CloseConfirmationAction::CloseApp => crate::utils::config::CloseAction::CloseApp,
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
            AppMessage::CloseConfirmationCancelled => {
                // 隐藏对话框，不执行任何操作
                self.show_close_confirmation = false;
            }
            AppMessage::ToggleRememberSetting(checked) => {
                self.remember_close_setting = checked;
            }
        }
        iced::Task::none()
    }
}

// 异步函数用于打开目录选择对话框
async fn select_data_path_async() -> String {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        path.to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}

async fn select_cache_path_async() -> String {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        path.to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}
