use super::App;
use super::AppMessage;
use crate::ui::ActivePage;
use crate::utils::startup;
use iced::window;
use std::time::Duration;
use tray_icon::{TrayIconEvent, menu::MenuEvent};

impl App {
    // 辅助方法：初始化动态图解码器
    fn init_animated_decoder(&mut self, index: usize) {
        if let Some(path) = self.local_state.all_paths.get(index) {
            let path = std::path::PathBuf::from(path);
            match crate::utils::animated_image::AnimatedDecoder::from_path(&path) {
                Ok(decoder) => {
                    if decoder.frame_count() > 1 {
                        self.local_state.animated_decoder = Some(decoder);
                    } else {
                        self.local_state.animated_decoder = None;
                    }
                }
                Err(_) => {
                    self.local_state.animated_decoder = None;
                }
            }
        }
    }

    // 辅助方法：查找下一个有效的图片索引
    fn find_next_valid_image_index(&self, start_index: usize, direction: i32) -> Option<usize> {
        if self.local_state.all_paths.is_empty() {
            return None;
        }

        let total = self.local_state.all_paths.len();
        let mut current_index = start_index;
        let loop_count = total; // 防止无限循环

        for _ in 0..loop_count {
            // 根据方向更新索引
            if direction > 0 {
                // 向前查找
                current_index = if current_index < total - 1 {
                    current_index + 1
                } else {
                    0
                };
            } else {
                // 向后查找
                current_index = if current_index > 0 {
                    current_index - 1
                } else {
                    total - 1
                };
            }

            // 检查是否回到起始位置
            if current_index == start_index {
                return None;
            }

            // 检查当前索引是否有效
            if let Some(wallpaper_status) = self.local_state.wallpapers.get(current_index) {
                if let super::local::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                    if wallpaper.name != "加载失败" {
                        return Some(current_index);
                    }
                }
            }
        }

        None
    }

    // 辅助方法：获取绝对路径
    fn get_absolute_path(&self, relative_path: &str) -> String {
        let path = std::path::PathBuf::from(relative_path);
        if path.is_absolute() {
            relative_path.to_string()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .join(relative_path)
                .to_string_lossy()
                .to_string()
        }
    }

    // 辅助方法：显示通知
    fn show_notification(&mut self, message: String, notification_type: super::NotificationType) -> iced::Task<AppMessage> {
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
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        use iced::event;
        use iced::time;
        use iced::window;

        iced::Subscription::batch([
            event::listen_with(|event, _status, _loop_status| match event {
                iced::Event::Window(window::Event::Resized(size)) => {
                    Some(AppMessage::WindowResized(size.width as u32, size.height as u32))
                }
                iced::Event::Window(window::Event::CloseRequested) => {
                    // 发送一个关闭请求消息，让App处理
                    Some(AppMessage::WindowCloseRequested)
                }
                _ => None,
            }),
            time::every(Duration::from_millis(50)).map(|_| AppMessage::DebounceTimer),
            // 添加动画定时器 - 每100毫秒更新一次旋转角度
            time::every(Duration::from_millis(100))
                .map(|_| AppMessage::Local(super::local::LocalMessage::AnimationTick)),
            // 添加动态图帧更新定时器 - 每50毫秒更新一次
            time::every(Duration::from_millis(50))
                .map(|_| AppMessage::Local(super::local::LocalMessage::AnimatedFrameUpdate)),
        ])
    }

    pub fn update(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        match msg {
            AppMessage::LanguageSelected(lang) => {
                self.i18n.set_language(lang.clone());
                // 同时更新配置
                self.config.set_language(lang);
                return iced::Task::none();
            }
            AppMessage::PageSelected(page) => {
                self.active_page = page;

                // 当切换到设置页面时，重置设置相关的临时状态
                if page == super::ActivePage::Settings {
                    // 重置代理设置相关状态
                    let (proxy_protocol, proxy_address, proxy_port) =
                        App::parse_proxy_string(&self.config.global.proxy);
                    self.proxy_protocol = proxy_protocol;
                    self.proxy_address = proxy_address;
                    self.proxy_port = proxy_port;

                    // 重置API KEY设置状态
                    self.wallhaven_api_key = self.config.api.wallhaven_api_key.clone();

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

                // 对于其他页面切换，返回无任务
                return iced::Task::none();
            }
            AppMessage::WindowResized(width, height) => {
                // 更新当前窗口宽度，用于响应式布局
                self.current_window_width = width;
                // 暂存窗口大小，等待防抖处理
                self.pending_window_size = Some((width, height));
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
                return iced::Task::none();
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
                    return window::oldest().and_then(|id| window::set_mode(id, window::Mode::Windowed));
                }
                "tray_settings" => {
                    // 打开设置窗口
                    self.active_page = ActivePage::Settings;
                    return window::oldest().and_then(|id| window::set_mode(id, window::Mode::Windowed));
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
                    return iced::Task::perform(select_folder_async(), |selected_path| {
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
                    return iced::Task::perform(select_folder_async(), |selected_path| {
                        if !selected_path.is_empty() {
                            AppMessage::CachePathSelected(selected_path)
                        } else {
                            AppMessage::CachePathSelected("".to_string()) // 用户取消选择
                        }
                    });
                } else if path == "SELECT_DATA_PATH" {
                    // 这是用户点击数据路径输入框时的原始消息，触发异步任务
                    return iced::Task::perform(select_folder_async(), |selected_path| {
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

                let full_path = self.get_absolute_path(path_to_open);

                if let Err(e) = open::that(&full_path) {
                    eprintln!("Failed to open path {}: {}", full_path, e);
                }
            }
            AppMessage::ShowPathClearConfirmation(path_type) => {
                // 显示路径清空确认对话框
                self.show_path_clear_confirmation = true;
                self.path_to_clear = path_type;
            }
            AppMessage::ConfirmPathClear(path_type) => {
                // 隐藏确认对话框
                self.show_path_clear_confirmation = false;

                // 执行清空操作
                let path_to_clear = match path_type.as_str() {
                    "data" => &self.config.data.data_path,
                    "cache" => &self.config.data.cache_path,
                    _ => return iced::Task::none(),
                };

                // 获取绝对路径
                let full_path = self.get_absolute_path(path_to_clear);

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
            AppMessage::CancelPathClear => {
                // 隐藏确认对话框，不执行清空操作
                self.show_path_clear_confirmation = false;
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
                return iced::Task::none();
            }
            AppMessage::WallhavenApiKeyChanged(api_key) => {
                self.wallhaven_api_key = api_key;
            }
            AppMessage::SaveWallhavenApiKey => {
                // 保存API KEY到配置文件
                self.config.set_wallhaven_api_key(self.wallhaven_api_key.clone());
                // 显示成功通知
                return self.show_notification("WallHeven API KEY 保存成功".to_string(), super::NotificationType::Success);
            }
            AppMessage::ScrollToTop(_scrollable_id) => {
                // 返回无任务，目前滚动到顶部功能依赖于不同的ID来实现隔离
                return iced::Task::none();
            }
            AppMessage::ProxyProtocolChanged(protocol) => {
                self.proxy_protocol = protocol;
                return iced::Task::none();
            }
            AppMessage::ProxyAddressChanged(address) => {
                self.proxy_address = address;
                return iced::Task::none();
            }
            AppMessage::ProxyPortChanged(port) => {
                self.proxy_port = port;
                return iced::Task::none();
            }
            AppMessage::SaveProxy => {
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
                    return self.show_notification("代理设置保存成功".to_string(), super::NotificationType::Success);
                } else {
                    // 地址或端口无效，保存为空字符串（相当于关闭代理）
                    self.config.set_proxy(String::new());
                    // 同时清空地址和端口输入框
                    self.proxy_address = String::new();
                    self.proxy_port = String::new();
                    // 显示错误通知
                    return self.show_notification("格式错误，代理设置保存失败".to_string(), super::NotificationType::Error);
                }
            }
            AppMessage::ShowCloseConfirmation => {
                self.show_close_confirmation = true;
                return iced::Task::none();
            }
            AppMessage::CloseConfirmationResponse(action, remember_setting) => {
                // 隐藏对话框
                self.show_close_confirmation = false;

                // 如果勾选了记住设置，则更新配置
                if remember_setting {
                    let new_close_action = match action {
                        super::CloseConfirmationAction::MinimizeToTray => {
                            crate::utils::config::CloseAction::MinimizeToTray
                        }
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
                return iced::Task::none();
            }
            AppMessage::ToggleRememberSetting(checked) => {
                self.remember_close_setting = checked;
                return iced::Task::none();
            }
            AppMessage::ShowNotification(message, notification_type) => {
                return self.show_notification(message, notification_type);
            }
            AppMessage::HideNotification => {
                self.show_notification = false;
            }
            AppMessage::Local(local_message) => {
                match local_message {
                    super::local::LocalMessage::LoadWallpapers => {
                        // 异步加载壁纸路径列表
                        let data_path = self.config.data.data_path.clone();
                        return iced::Task::perform(async_load_wallpaper_paths(data_path), |result| match result {
                            Ok(paths) => AppMessage::Local(super::local::LocalMessage::LoadWallpapersSuccess(paths)),
                            Err(e) => {
                                AppMessage::Local(super::local::LocalMessage::LoadWallpapersFailed(e.to_string()))
                            }
                        });
                    }
                    super::local::LocalMessage::LoadWallpapersSuccess(paths) => {
                        // 更新本地状态，初始化壁纸加载状态列表
                        self.local_state.all_paths = paths;
                        self.local_state.total_count = self.local_state.all_paths.len();

                        // 初始化壁纸状态为Loading，并加载第一页
                        let page_end = std::cmp::min(self.local_state.page_size, self.local_state.total_count);
                        self.local_state.wallpapers = vec![super::local::WallpaperLoadStatus::Loading; page_end];

                        // 触发第一页加载
                        return iced::Task::perform(async {}, |_| {
                            AppMessage::Local(super::local::LocalMessage::LoadPage)
                        });
                    }
                    super::local::LocalMessage::LoadWallpapersFailed(error) => {
                        // 由于现在使用WallpaperLoadStatus处理单个壁纸的错误，整体错误处理已不再需要
                        // 可以考虑显示一个通知或者在UI的其他地方显示错误
                        println!("加载壁纸列表失败: {}", error);
                    }
                    super::local::LocalMessage::LoadPage => {
                        if self.local_state.current_page * self.local_state.page_size >= self.local_state.total_count {
                            // 没有更多壁纸可加载
                            self.local_state.loading_page = false;
                            return iced::Task::none();
                        }

                        // 设置加载状态
                        self.local_state.loading_page = true;

                        // 获取当前页需要加载的壁纸路径
                        let start_idx = self.local_state.current_page * self.local_state.page_size;
                        let end_idx =
                            std::cmp::min(start_idx + self.local_state.page_size, self.local_state.total_count);

                        // 为每个壁纸启动单独的异步任务
                        let mut tasks = Vec::new();
                        for (i, path) in self.local_state.all_paths[start_idx..end_idx].iter().enumerate() {
                            let path = path.clone();
                            let cache_path = self.config.data.cache_path.clone();
                            let absolute_idx = start_idx + i;

                            tasks.push(iced::Task::perform(
                                async_load_single_wallpaper_with_fallback(path.clone(), cache_path),
                                move |result| match result {
                                    Ok(wallpaper) => AppMessage::Local(super::local::LocalMessage::LoadPageSuccess(
                                        vec![(absolute_idx, wallpaper)],
                                    )),
                                    Err(_) => AppMessage::Local(super::local::LocalMessage::LoadPageSuccess(vec![(
                                        absolute_idx,
                                        crate::services::local::Wallpaper::new(path, "加载失败".to_string(), 0, 0, 0),
                                    )])), // 创建失败状态
                                },
                            ));
                        }

                        // 更新当前页面的壁纸状态为加载中
                        let page_start = self.local_state.current_page * self.local_state.page_size;
                        let page_end =
                            std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

                        if self.local_state.current_page == 0 {
                            // 第一页：初始化wallpapers数组
                            self.local_state.wallpapers = vec![super::local::WallpaperLoadStatus::Loading; page_end];
                        } else {
                            // 后续页面：扩展wallpapers数组
                            for _ in 0..(page_end - self.local_state.wallpapers.len()) {
                                self.local_state
                                    .wallpapers
                                    .push(super::local::WallpaperLoadStatus::Loading);
                            }
                        }

                        self.local_state.current_page += 1;
                        return iced::Task::batch(tasks);
                    }
                    super::local::LocalMessage::LoadPageSuccess(wallpapers_with_idx) => {
                        // 为每个加载完成的壁纸更新状态
                        for (idx, wallpaper) in wallpapers_with_idx {
                            if idx < self.local_state.wallpapers.len() {
                                self.local_state.wallpapers[idx] = super::local::WallpaperLoadStatus::Loaded(wallpaper);
                            }
                        }

                        // 检查是否所有壁纸都已加载完成，如果是则更新loading_page状态
                        let page_start = (self.local_state.current_page - 1) * self.local_state.page_size; // 上一页的起始位置
                        let page_end =
                            std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

                        let all_loaded = (page_start..page_end).all(|i| {
                            i < self.local_state.wallpapers.len()
                                && matches!(
                                    self.local_state.wallpapers[i],
                                    super::local::WallpaperLoadStatus::Loaded(_)
                                )
                        });

                        if all_loaded {
                            self.local_state.loading_page = false;
                        }
                    }
                    super::local::LocalMessage::LoadPageFailed(error) => {
                        // 更新加载状态
                        self.local_state.loading_page = false;
                        // 由于现在使用WallpaperLoadStatus处理单个壁纸的错误，整体错误处理已不再需要
                        println!("加载页面壁纸失败: {}", error);
                    }
                    super::local::LocalMessage::WallpaperSelected(wallpaper) => {
                        // 处理壁纸选择
                        println!("选择了壁纸: {}", wallpaper.path);
                    }
                    super::local::LocalMessage::ShowModal(index) => {
                        // 检查要显示的图片是否为失败状态
                        if let Some(wallpaper_status) = self.local_state.wallpapers.get(index) {
                            if let super::local::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                                if wallpaper.name == "加载失败" {
                                    // 如果是失败的图片，不显示模态窗口
                                    return iced::Task::none();
                                }
                            }
                        }

                        // 显示模态窗口，设置当前图片索引
                        self.local_state.current_image_index = index;
                        self.local_state.modal_visible = true;

                        // 使用辅助方法初始化动态图解码器
                        self.init_animated_decoder(index);
                    }
                    super::local::LocalMessage::CloseModal => {
                        // 关闭模态窗口
                        self.local_state.modal_visible = false;
                        // 清理动态图解码器
                        self.local_state.animated_decoder = None;
                    }
                    super::local::LocalMessage::NextImage => {
                        // 显示下一张图片，跳过加载失败的图片
                        if let Some(next_index) = self.find_next_valid_image_index(self.local_state.current_image_index, 1) {
                            self.local_state.current_image_index = next_index;
                            self.init_animated_decoder(next_index);
                        }
                    }
                    super::local::LocalMessage::PreviousImage => {
                        // 显示上一张图片，跳过加载失败的图片
                        if let Some(prev_index) = self.find_next_valid_image_index(self.local_state.current_image_index, -1) {
                            self.local_state.current_image_index = prev_index;
                            self.init_animated_decoder(prev_index);
                        }
                    }
                    super::local::LocalMessage::ScrollToBottom => {
                        // 滚动到底部，如果还有更多壁纸则加载下一页
                        if self.local_state.current_page * self.local_state.page_size < self.local_state.total_count
                            && !self.local_state.loading_page
                        {
                            return iced::Task::perform(async {}, |_| {
                                AppMessage::Local(super::local::LocalMessage::LoadPage)
                            });
                        }
                    }
                    super::local::LocalMessage::AnimationTick => {
                        // 更新旋转角度以创建动画效果
                        // 每次增加15度，如果超过360度则重置
                        self.local_state.rotation_angle = (self.local_state.rotation_angle + 15.0) % 360.0;
                    }
                    super::local::LocalMessage::AnimatedFrameUpdate => {
                        // 更新动态图帧
                        if let Some(ref mut decoder) = self.local_state.animated_decoder {
                            decoder.update();
                        }
                    }
                    super::local::LocalMessage::ViewInFolder(index) => {
                        // 查看文件夹并选中文件
                        if let Some(path) = self.local_state.all_paths.get(index) {
                            let full_path = self.get_absolute_path(path);

                            // Windows: 使用 explorer /select,路径
                            #[cfg(target_os = "windows")]
                            {
                                let _ = std::process::Command::new("explorer")
                                    .args(["/select,", &full_path])
                                    .spawn();
                            }
                            // macOS: 使用 open -R 路径
                            #[cfg(target_os = "macos")]
                            {
                                let _ = std::process::Command::new("open")
                                    .args(["-R", &full_path])
                                    .spawn();
                            }
                            // Linux: 使用 dbus 调用文件管理器（需要支持）
                            #[cfg(target_os = "linux")]
                            {
                                // 尝试使用 xdg-open 打开文件所在目录
                                if let Some(parent) = std::path::Path::new(&full_path).parent() {
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg(parent)
                                        .spawn();
                                }
                            }
                        }
                    }
                    super::local::LocalMessage::ShowDeleteConfirm(index) => {
                        // 显示删除确认对话框
                        self.local_state.delete_confirm_visible = true;
                        self.local_state.delete_target_index = Some(index);
                    }
                    super::local::LocalMessage::CloseDeleteConfirm => {
                        // 关闭删除确认对话框
                        self.local_state.delete_confirm_visible = false;
                        self.local_state.delete_target_index = None;
                    }
                    super::local::LocalMessage::ConfirmDelete(index) => {
                        // 确认删除壁纸
                        self.local_state.delete_confirm_visible = false;
                        self.local_state.delete_target_index = None;

                        // 删除壁纸
                        if let Some(path) = self.local_state.all_paths.get(index) {
                            let full_path = self.get_absolute_path(path);

                            // 尝试删除文件
                            match std::fs::remove_file(&full_path) {
                                Ok(_) => {
                                    // 删除成功，从列表中移除
                                    self.local_state.all_paths.remove(index);
                                    self.local_state.wallpapers.remove(index);
                                    self.local_state.total_count -= 1;

                                    // 如果删除的是当前显示的图片，关闭模态窗口
                                    if self.local_state.modal_visible && self.local_state.current_image_index == index {
                                        self.local_state.modal_visible = false;
                                        self.local_state.animated_decoder = None;
                                    } else if self.local_state.modal_visible && self.local_state.current_image_index > index {
                                        // 如果删除的图片在当前显示图片之前，调整索引
                                        self.local_state.current_image_index -= 1;
                                    }

                                    // 显示成功通知
                                    return self.show_notification(
                                        self.i18n.t("local-list.delete-success"),
                                        super::NotificationType::Success
                                    );
                                }
                                Err(e) => {
                                    // 删除失败，显示错误通知
                                    return self.show_notification(
                                        format!("{}: {}", self.i18n.t("local-list.delete-failed"), e),
                                        super::NotificationType::Error
                                    );
                                }
                            }
                        }
                    }
                    super::local::LocalMessage::SetWallpaper(index) => {
                        // 设置壁纸
                        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
                            let full_path = self.get_absolute_path(&path);

                            // 提前获取翻译文本，避免线程安全问题
                            let success_message = self.i18n.t("local-list.set-wallpaper-success").to_string();
                            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                            // 异步设置壁纸
                            return iced::Task::perform(
                                async_set_wallpaper(full_path),
                                move |result| match result {
                                    Ok(_) => AppMessage::ShowNotification(
                                        success_message,
                                        super::NotificationType::Success
                                    ),
                                    Err(e) => AppMessage::ShowNotification(
                                        format!("{}: {}", failed_message, e),
                                        super::NotificationType::Error
                                    ),
                                }
                            );
                        }
                    }
                }
            }
        }
        iced::Task::none()
    }
}

// 异步加载壁纸路径列表函数
async fn async_load_wallpaper_paths(
    data_path: String,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    // 在这里调用同步的获取壁纸路径函数
    tokio::task::spawn_blocking(move || crate::services::local::LocalWallpaperService::get_wallpaper_paths(&data_path))
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

// 异步加载单个壁纸函数（带降级处理，即使图片加载失败也能获取文件大小）
async fn async_load_single_wallpaper_with_fallback(
    wallpaper_path: String,
    cache_path: String,
) -> Result<crate::services::local::Wallpaper, Box<dyn std::error::Error + Send + Sync>> {
    let full_cache_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(&cache_path);

    // 使用spawn_blocking在阻塞线程池中运行
    tokio::task::spawn_blocking(move || {
        // 先获取文件大小（这个操作通常不会失败）
        let file_size = std::fs::metadata(&wallpaper_path)
            .map(|metadata| metadata.len())
            .unwrap_or(0);

        // 尝试加载图片
        let result = (|| -> Result<crate::services::local::Wallpaper, Box<dyn std::error::Error + Send + Sync>> {
            let thumbnail_path = crate::services::local::LocalWallpaperService::generate_thumbnail_for_path(
                &wallpaper_path,
                &full_cache_path.to_string_lossy(),
            )?;

            // 获取图片尺寸
            let (width, height) = image::image_dimensions(&wallpaper_path)
                .unwrap_or((0, 0));

            Ok(crate::services::local::Wallpaper::with_thumbnail(
                wallpaper_path.clone(),
                std::path::Path::new(&wallpaper_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                thumbnail_path,
                file_size,
                width,
                height,
            ))
        })();

        match result {
            Ok(wallpaper) => Ok(wallpaper),
            Err(_) => {
                // 如果加载失败，返回一个带有文件大小的失败状态
                Ok(crate::services::local::Wallpaper::new(
                    wallpaper_path.clone(),
                    "加载失败".to_string(),
                    file_size,
                    0,
                    0,
                ))
            }
        }
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

// 异步设置壁纸函数
async fn async_set_wallpaper(
    wallpaper_path: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || {
        crate::services::local::LocalWallpaperService::set_wallpaper(&wallpaper_path)
    })
    .await
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
}

// 异步函数用于打开目录选择对话框
async fn select_folder_async() -> String {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        path.to_string_lossy().to_string()
    } else {
        "".to_string() // 用户取消选择
    }
}
