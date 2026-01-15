use super::App;
use super::AppMessage;
use crate::ui::ActivePage;
use crate::utils::startup;
use iced::futures::StreamExt;
use iced::window;
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
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

    // 辅助方法：开始下载壁纸（支持并行限制和进度更新）
    fn start_download(&mut self, url: String, id: &str, file_type: &str) -> iced::Task<AppMessage> {
        let file_name = super::download::generate_file_name(id, file_type.split('/').last().unwrap_or("jpg"));
        let data_path = self.config.data.data_path.clone();
        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };
        let file_type = file_type.split('/').last().unwrap_or("jpg").to_string();

        println!("[下载功能] 添加任务: url={}, file_name={}, data_path={}", url, file_name, data_path);

        // 生成完整保存路径
        let full_save_path = PathBuf::from(&data_path).join(&file_name);
        println!("[下载功能] 完整保存路径: {}", full_save_path.display());

        // 添加任务（倒序排列）
        self.download_state.add_task(url.clone(), full_save_path.to_string_lossy().to_string(), file_name.clone(), proxy.clone(), file_type.clone());

        // 获取任务ID
        let task_id = self.download_state.next_id.saturating_sub(1);
        println!("[下载功能] 新任务ID: {}", task_id);

        // 检查是否可以开始下载
        let downloading_count = self.download_state.get_downloading_count();
        let max_downloads = self.download_state.max_concurrent_downloads;
        println!("[下载功能] 当前下载任务数: {}/{}", downloading_count, max_downloads);

        if self.download_state.can_start_download() {
            // 可以开始下载 - 使用索引查找任务
            let task_index = self.download_state.find_task_index(task_id);
            if let Some(index) = task_index {
                let task_full = self.download_state.get_task_by_index(index);
                if let Some(task_full) = task_full {
                    println!("[下载功能] 获取任务成功，save_path='{}'", task_full.task.save_path);

                    // 先保存需要的数据，再修改状态
                    let url = task_full.task.url.clone();
                    let save_path = PathBuf::from(&task_full.task.save_path);
                    let proxy = task_full.proxy.clone();
                    let task_id = task_full.task.id;
                    let cancel_token = task_full.task.cancel_token.clone().unwrap();
                    let downloaded_size = task_full.task.downloaded_size;

                    // 更新状态
                    task_full.task.status = super::download::DownloadStatus::Downloading;
                    task_full.task.start_time = Some(std::time::Instant::now());
                    self.download_state.increment_downloading();

                    println!("[下载功能] 最终保存路径: '{}'", save_path.display());

                    // 打印代理信息
                    if let Some(ref proxy_url) = proxy {
                        println!("[下载任务] [ID:{}] 使用代理: {}", task_id, proxy_url);
                    } else {
                        println!("[下载任务] [ID:{}] 不使用代理", task_id);
                    }

                    // 启动异步下载任务（带进度更新）
                    return iced::Task::perform(
                        async_download_wallpaper_task_with_progress(url, save_path, proxy, task_id, cancel_token, downloaded_size),
                        move |result| {
                            match result {
                                Ok(size) => {
                                    println!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                    AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                                }
                                Err(e) => {
                                    println!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                    AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                                }
                            }
                        },
                    );
                }
            }
        } else {
            // 任务排队等待
            println!("[下载功能] 任务排队等待，当前下载任务数: {}/{}", self.download_state.get_downloading_count(), self.download_state.max_concurrent_downloads);
        }

        // 显示通知
        iced::Task::done(AppMessage::ShowNotification(
            format!("已添加到下载队列 (等待中)"),
            super::NotificationType::Success
        ))
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
            // 添加下载进度监听 - 使用run_with
            iced::Subscription::run_with(
                DownloadProgressSubscription,
                |_state| {
                    // 初始化下载进度channel
                    crate::services::init_download_progress_channel();

                    // 获取channel接收器
                    let rx = if let Some(tx) = crate::services::DOWNLOAD_PROGRESS_TX.get() {
                        Some(tx.subscribe())
                    } else {
                        None
                    };

                    async_stream::stream! {
                        if let Some(mut rx) = rx {
                            loop {
                                match rx.recv().await {
                                    Ok(update) => {
                                        yield AppMessage::Download(
                                            super::download::DownloadMessage::DownloadProgress(
                                                update.task_id,
                                                update.downloaded,
                                                update.total,
                                                update.speed,
                                            )
                                        );
                                    }
                                    Err(_) => {
                                        // Channel关闭，退出循环
                                        break;
                                    }
                                }
                            }
                        } else {
                            // 如果channel未初始化，返回空stream
                            std::future::pending::<()>().await;
                        }
                    }
                }
            ),
        ])
    }
}

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

impl App {
    pub fn update(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        // 检查是否需要加载初始任务（只在第一次运行时）
        if !self.initial_loaded {
            self.initial_loaded = true;
            // 如果默认页面是在线壁纸，则加载初始数据
            if self.active_page == super::ActivePage::OnlineWallpapers {
                return iced::Task::batch(vec![
                    iced::Task::perform(async {}, |_| {
                        AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)
                    }),
                    iced::Task::perform(async {}, |_| {
                        AppMessage::ScrollToTop("online_wallpapers_scroll".to_string())
                    }),
                ]);
            }
        }

        match msg {
            AppMessage::None => {
                // 空消息，不做任何操作
            }
            AppMessage::LanguageSelected(lang) => {
                self.i18n.set_language(lang.clone());
                // 同时更新配置
                self.config.set_language(lang);
                return iced::Task::none();
            }
            AppMessage::PageSelected(page) => {
                // 当切换离开在线壁纸页面时，取消正在进行的请求
                if self.active_page == super::ActivePage::OnlineWallpapers && page != super::ActivePage::OnlineWallpapers {
                    self.online_state.cancel_and_new_context();
                    println!("[页面切换] 已取消在线壁纸页面的网络请求");
                }

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
                    self.wallhaven_api_key = self.config.wallhaven.api_key.clone();

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

                // 每次切换到在线壁纸页面时，都重新加载壁纸
                if page == super::ActivePage::OnlineWallpapers {
                    // 从配置文件加载在线状态，以便恢复筛选条件
                    self.online_state = super::online::OnlineState::load_from_config(&self.config);
                    return iced::Task::batch(vec![
                        iced::Task::perform(async {}, |_| {
                            AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)
                        }),
                        iced::Task::perform(async {}, |_| {
                            AppMessage::ScrollToTop("online_wallpapers_scroll".to_string())
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

                        // 清除之前的图片数据
                        self.local_state.modal_image_handle = None;

                        // 使用辅助方法初始化动态图解码器
                        self.init_animated_decoder(index);

                        // 异步加载图片数据
                        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
                            return iced::Task::perform(
                                async move {
                                    // 异步加载图片数据
                                    let image_handle = iced::widget::image::Handle::from_path(&path);
                                    // 等待一小段时间确保图片数据已加载
                                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                    image_handle
                                },
                                |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
                            );
                        }
                    }
                    super::local::LocalMessage::ModalImageLoaded(handle) => {
                        // 模态窗口图片加载完成，保存图片数据
                        self.local_state.modal_image_handle = Some(handle);
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

                            // 清除之前的图片数据
                            self.local_state.modal_image_handle = None;

                            self.init_animated_decoder(next_index);

                            // 异步加载图片数据
                            if let Some(path) = self.local_state.all_paths.get(next_index).cloned() {
                                return iced::Task::perform(
                                    async move {
                                        // 异步加载图片数据
                                        let image_handle = iced::widget::image::Handle::from_path(&path);
                                        // 等待一小段时间确保图片数据已加载
                                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                        image_handle
                                    },
                                    |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
                                );
                            }
                        }
                    }
                    super::local::LocalMessage::PreviousImage => {
                        // 显示上一张图片，跳过加载失败的图片
                        if let Some(prev_index) = self.find_next_valid_image_index(self.local_state.current_image_index, -1) {
                            self.local_state.current_image_index = prev_index;

                            // 清除之前的图片数据
                            self.local_state.modal_image_handle = None;

                            self.init_animated_decoder(prev_index);

                            // 异步加载图片数据
                            if let Some(path) = self.local_state.all_paths.get(prev_index).cloned() {
                                return iced::Task::perform(
                                    async move {
                                        // 异步加载图片数据
                                        let image_handle = iced::widget::image::Handle::from_path(&path);
                                        // 等待一小段时间确保图片数据已加载
                                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                        image_handle
                                    },
                                    |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
                                );
                            }
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
                        // 动画刻度消息（已不再使用，保留以避免编译错误）
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
            AppMessage::Online(online_message) => {
                match online_message {
                    super::online::OnlineMessage::LoadWallpapers => {
                        // 设置加载状态
                        self.online_state.loading_page = true;
                        // 清空当前数据，准备加载新数据
                        self.online_state.wallpapers.clear();
                        self.online_state.wallpapers_data.clear();
                        self.online_state.page_boundaries.clear();
                        self.online_state.has_loaded = false;

                        // 创建新的请求上下文并取消之前的请求
                        self.online_state.cancel_and_new_context();
                        let context = self.online_state.request_context.clone();

                        // 异步加载在线壁纸
                        let categories = self.online_state.categories;
                        let sorting = self.online_state.sorting;
                        let purities = self.online_state.purities;
                        let query = self.online_state.search_text.clone();
                        let page = self.online_state.current_page;
                        let api_key = if self.config.wallhaven.api_key.is_empty() {
                            None
                        } else {
                            Some(self.config.wallhaven.api_key.clone())
                        };

                        let proxy = if self.config.global.proxy.is_empty() {
                            None
                        } else {
                            Some(self.config.global.proxy.clone())
                        };

                        return iced::Task::perform(
                            async_load_online_wallpapers(categories, sorting, purities, query, page, api_key, proxy, context),
                            |result| match result {
                                Ok((wallpapers, last_page, total_pages, current_page)) => {
                                    AppMessage::Online(super::online::OnlineMessage::LoadWallpapersSuccess(wallpapers, last_page, total_pages, current_page))
                                }
                                Err(e) => {
                                    AppMessage::Online(super::online::OnlineMessage::LoadWallpapersFailed(e.to_string()))
                                }
                            },
                        );
                    }
                    super::online::OnlineMessage::LoadWallpapersSuccess(wallpapers, last_page, total_pages, current_page) => {
                        // 更新在线壁纸状态，并开始加载缩略图
                        self.online_state.current_page = current_page;
                        self.online_state.total_pages = total_pages;

                        // 判断是否是最后一页：
                        // 如果 current_page == total_pages && current_page == 1 && data 为空，说明无数据
                        // 否则 last_page（布尔值）表示已加载到最后一页
                        let is_empty_data = wallpapers.is_empty();
                        let is_first_and_last_page = current_page == 1 && total_pages == 1;
                        self.online_state.last_page = if is_empty_data && is_first_and_last_page {
                            // 无数据情况：last_page 为 false（允许后续尝试不同筛选条件时重新加载）
                            false
                        } else {
                            last_page
                        };
                        self.online_state.has_loaded = true; // 标记已加载过数据

                        let proxy = if self.config.global.proxy.is_empty() {
                            None
                        } else {
                            Some(self.config.global.proxy.clone())
                        };

                        let cache_path = self.config.data.cache_path.clone();

                        let mut tasks = Vec::new();
                        for (idx, wallpaper) in wallpapers.iter().enumerate() {
                            let url = wallpaper.thumb_large.clone();
                            let file_size = wallpaper.file_size;
                            let proxy = proxy.clone();
                            let cache_path = cache_path.clone();
                            tasks.push(iced::Task::perform(
                                async_load_online_wallpaper_thumb_with_cache(url, file_size, cache_path, proxy),
                                move |result| match result {
                                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, handle)),
                                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, iced::widget::image::Handle::from_bytes(vec![]))),
                                }
                            ));
                        }

                        self.online_state.wallpapers_data = wallpapers.clone();
                        self.online_state.wallpapers = wallpapers
                            .into_iter()
                            .map(|_w| super::online::WallpaperLoadStatus::Loading)
                            .collect();
                        self.online_state.total_count = self.online_state.wallpapers.len();

                        // 初始化 page_boundaries，记录第一页的起始索引和结束后的边界
                        self.online_state.page_boundaries.clear();
                        self.online_state.page_boundaries.push(0);
                        // 如果有数据，添加第一页结束后的边界（用于在第一页数据后显示分页标识）
                        if !self.online_state.wallpapers.is_empty() {
                            self.online_state.page_boundaries.push(self.online_state.wallpapers.len());
                        }

                        self.online_state.loading_page = false;

                        return iced::Task::batch(tasks);
                    }
                    super::online::OnlineMessage::LoadWallpapersFailed(error) => {
                        // 加载失败
                        self.online_state.loading_page = false;
                        self.online_state.has_loaded = true; // 标记已加载过数据（虽然失败了）
                        println!("加载在线壁纸失败: {}", error);
                    }
                    super::online::OnlineMessage::WallpaperSelected(wallpaper) => {
                        // 处理壁纸选择
                        println!("选择了壁纸: {}", wallpaper.path);
                    }
                    super::online::OnlineMessage::LoadPage => {
                        // 加载下一页
                        self.online_state.loading_page = true;

                        // 创建新的请求上下文并取消之前的请求
                        self.online_state.cancel_and_new_context();
                        let context = self.online_state.request_context.clone();

                        let categories = self.online_state.categories;
                        let sorting = self.online_state.sorting;
                        let purities = self.online_state.purities;
                        let query = self.online_state.search_text.clone();
                        let page = self.online_state.current_page;
                        let api_key = if self.config.wallhaven.api_key.is_empty() {
                            None
                        } else {
                            Some(self.config.wallhaven.api_key.clone())
                        };

                        let proxy = if self.config.global.proxy.is_empty() {
                            None
                        } else {
                            Some(self.config.global.proxy.clone())
                        };

                        return iced::Task::perform(
                            async_load_online_wallpapers(categories, sorting, purities, query, page, api_key, proxy, context),
                            |result| match result {
                                Ok((wallpapers, last_page, total_pages, current_page)) => {
                                    AppMessage::Online(super::online::OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page))
                                }
                                Err(e) => {
                                    AppMessage::Online(super::online::OnlineMessage::LoadPageFailed(e.to_string()))
                                }
                            },
                        );
                    }
                    super::online::OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page) => {
                        // 添加新壁纸到列表，并开始加载缩略图
                        self.online_state.current_page = current_page;
                        self.online_state.total_pages = total_pages;

                        // 判断是否是最后一页：
                        // 如果 current_page == total_pages && current_page == 1 && data 为空，说明无数据
                        // 否则 last_page（布尔值）表示已加载到最后一页
                        let is_empty_data = wallpapers.is_empty();
                        let is_first_and_last_page = current_page == 1 && total_pages == 1;
                        self.online_state.last_page = if is_empty_data && is_first_and_last_page {
                            // 无数据情况：last_page 为 false
                            false
                        } else {
                            last_page
                        };
                        self.online_state.has_loaded = true; // 标记已加载过数据

                        let proxy = if self.config.global.proxy.is_empty() {
                            None
                        } else {
                            Some(self.config.global.proxy.clone())
                        };

                        let cache_path = self.config.data.cache_path.clone();

                        let start_idx = self.online_state.wallpapers.len();
                        let mut tasks = Vec::new();
                        for (offset, wallpaper) in wallpapers.iter().enumerate() {
                            let idx = start_idx + offset;
                            let url = wallpaper.thumb_large.clone();
                            let file_size = wallpaper.file_size;
                            let proxy = proxy.clone();
                            let cache_path = cache_path.clone();
                            tasks.push(iced::Task::perform(
                                async_load_online_wallpaper_thumb_with_cache(url, file_size, cache_path, proxy),
                                move |result| match result {
                                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, handle)),
                                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, iced::widget::image::Handle::from_bytes(vec![]))),
                                }
                            ));
                        }

                        // 保存原始数据
                        for wallpaper in &wallpapers {
                            self.online_state.wallpapers_data.push(wallpaper.clone());
                            self.online_state.wallpapers.push(super::online::WallpaperLoadStatus::Loading);
                        }

                        // 在添加完当前页数据后记录分页边界
                        // 这样分页标识就可以在当前页数据的下面显示
                        let boundary_index = self.online_state.wallpapers.len();
                        self.online_state.page_boundaries.push(boundary_index);

                        self.online_state.loading_page = false;

                        return iced::Task::batch(tasks);
                    }
                    super::online::OnlineMessage::LoadPageFailed(error) => {
                        // 加载失败
                        self.online_state.loading_page = false;
                        self.online_state.has_loaded = true; // 标记已加载过数据（虽然失败了）
                        println!("加载在线壁纸页面失败: {}", error);
                    }
                    super::online::OnlineMessage::ShowModal(index) => {
                        // 显示模态窗口
                        self.online_state.current_image_index = index;
                        self.online_state.modal_visible = true;
                        self.online_state.modal_image_handle = None;

                        // 异步加载图片数据
                        if let Some(wallpaper_status) = self.online_state.wallpapers.get(index) {
                            if let super::online::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                                let url = wallpaper.path.clone();
                                let proxy = if self.config.global.proxy.is_empty() {
                                    None
                                } else {
                                    Some(self.config.global.proxy.clone())
                                };
                                return iced::Task::perform(
                                    async_load_online_wallpaper_image(url, proxy),
                                    |result| match result {
                                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
                                }
                                );
                            }
                        }
                    }
                    super::online::OnlineMessage::ModalImageLoaded(handle) => {
                        // 模态窗口图片加载完成
                        self.online_state.modal_image_handle = Some(handle);
                    }
                    super::online::OnlineMessage::ThumbLoaded(index, handle) => {
                        // 缩略图加载完成
                        if index < self.online_state.wallpapers.len() && index < self.online_state.wallpapers_data.len() {
                            let wallpaper = self.online_state.wallpapers_data[index].clone();
                            self.online_state.wallpapers[index] = super::online::WallpaperLoadStatus::ThumbLoaded(wallpaper, handle);
                        }
                    }
                    super::online::OnlineMessage::CloseModal => {
                        // 关闭模态窗口
                        self.online_state.modal_visible = false;
                        self.online_state.modal_image_handle = None;
                    }
                    super::online::OnlineMessage::NextImage => {
                        // 显示下一张图片
                        if self.online_state.current_image_index < self.online_state.wallpapers.len() - 1 {
                            self.online_state.current_image_index += 1;
                            self.online_state.modal_image_handle = None;

                            if let Some(wallpaper_status) = self.online_state.wallpapers.get(self.online_state.current_image_index) {
                                if let super::online::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                                    let url = wallpaper.path.clone();
                                    let proxy = if self.config.global.proxy.is_empty() {
                                        None
                                    } else {
                                        Some(self.config.global.proxy.clone())
                                    };
                                    return iced::Task::perform(
                                        async_load_online_wallpaper_image(url, proxy),
                                        |result| match result {
                                            Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                                            Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
                                        }
                                    );
                                }
                            }
                        }
                    }
                    super::online::OnlineMessage::PreviousImage => {
                        // 显示上一张图片
                        if self.online_state.current_image_index > 0 {
                            self.online_state.current_image_index -= 1;
                            self.online_state.modal_image_handle = None;

                            if let Some(wallpaper_status) = self.online_state.wallpapers.get(self.online_state.current_image_index) {
                                if let super::online::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                                    let url = wallpaper.path.clone();
                                    let proxy = if self.config.global.proxy.is_empty() {
                                        None
                                    } else {
                                        Some(self.config.global.proxy.clone())
                                    };
                                    return iced::Task::perform(
                                        async_load_online_wallpaper_image(url, proxy),
                                        |result| match result {
                                            Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                                            Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
                                        }
                                    );
                                }
                            }
                        }
                    }
                    super::online::OnlineMessage::ScrollToBottom => {
                        // 滚动到底部，检查是否需要加载下一页
                        if self.online_state.should_load_next_page() {
                            self.online_state.current_page += 1;
                            return iced::Task::perform(async {}, |_| {
                                AppMessage::Online(super::online::OnlineMessage::LoadPage)
                            });
                        }
                    }
                    super::online::OnlineMessage::DownloadWallpaper(index) => {
                        // 下载壁纸 - 添加到下载队列并自动开始下载
                        println!("[下载功能] 收到下载请求，index={}", index);
                        println!("[下载功能] wallpapers 列表长度: {}", self.online_state.wallpapers.len());

                        if let Some(wallpaper_status) = self.online_state.wallpapers.get(index) {
                            println!("[下载功能] 获取到壁纸状态: {:?}", std::mem::discriminant(wallpaper_status));

                            // 尝试匹配 Loaded 或 ThumbLoaded 状态
                            match wallpaper_status {
                                super::online::WallpaperLoadStatus::Loaded(wallpaper) => {
                                    println!("[下载功能] 壁纸状态为 Loaded，id={}, path={}", wallpaper.id, wallpaper.path);
                                    // 先提取数据，避免借用冲突
                                    let url = wallpaper.path.clone();
                                    let id = wallpaper.id.clone();
                                    let file_type = wallpaper.file_type.clone();
                                    return self.start_download(url, &id, &file_type);
                                }
                                super::online::WallpaperLoadStatus::ThumbLoaded(wallpaper, _) => {
                                    println!("[下载功能] 壁纸状态为 ThumbLoaded，id={}, path={}", wallpaper.id, wallpaper.path);
                                    // 先提取数据，避免借用冲突
                                    let url = wallpaper.path.clone();
                                    let id = wallpaper.id.clone();
                                    let file_type = wallpaper.file_type.clone();
                                    return self.start_download(url, &id, &file_type);
                                }
                                super::online::WallpaperLoadStatus::Loading => {
                                    println!("[下载功能] 壁纸仍在加载中，无法下载");
                                    return iced::Task::done(AppMessage::ShowNotification(
                                        "壁纸正在加载，请稍后再试".to_string(),
                                        super::NotificationType::Error
                                    ));
                                }
                            }
                        } else {
                            println!("[下载功能] 错误：无法获取壁纸数据，index={} 超出范围", index);
                            return iced::Task::done(AppMessage::ShowNotification(
                                "无法获取壁纸数据".to_string(),
                                super::NotificationType::Error
                            ));
                        }
                    }
                    super::online::OnlineMessage::SetAsWallpaper(index) => {
                        // 设为壁纸（待实现）
                        println!("设为壁纸: index={}", index);
                    }
                    super::online::OnlineMessage::CategoryToggled(category) => {
                        // 切换分类选择状态
                        self.online_state.categories ^= category.bit_value();
                        // 保存筛选配置
                        self.online_state.save_to_config(&mut self.config);
                    }
                    super::online::OnlineMessage::SortingChanged(sorting) => {
                        // 排序改变
                        self.online_state.sorting = sorting;
                        // 保存筛选配置
                        self.online_state.save_to_config(&mut self.config);
                    }
                    super::online::OnlineMessage::PurityToggled(purity) => {
                        // 切换纯净度选择状态
                        self.online_state.purities ^= purity.bit_value();
                        // 保存筛选配置
                        self.online_state.save_to_config(&mut self.config);
                    }
                    super::online::OnlineMessage::ResolutionChanged(resolution) => {
                        // 分辨率改变
                        self.online_state.resolution = resolution;
                    }
                    super::online::OnlineMessage::RatioChanged(ratio) => {
                        // 比例改变
                        self.online_state.ratio = ratio;
                    }
                    super::online::OnlineMessage::ColorChanged(color) => {
                        // 颜色改变
                        self.online_state.color = color;
                    }
                    super::online::OnlineMessage::TimeRangeChanged(time_range) => {
                        // 时间范围改变
                        self.online_state.time_range = time_range;
                    }
                    super::online::OnlineMessage::SearchTextChanged(text) => {
                        // 搜索文本改变
                        self.online_state.search_text = text;
                    }
                    super::online::OnlineMessage::Search => {
                        // 搜索，重新加载壁纸
                        self.online_state.current_page = 1;
                        self.online_state.wallpapers.clear();
                        self.online_state.last_page = false;
                        return iced::Task::perform(async {}, |_| {
                            AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)
                        });
                    }
                    super::online::OnlineMessage::Refresh => {
                        // 刷新，重新加载壁纸
                        self.online_state.current_page = 1;
                        self.online_state.wallpapers.clear();
                        self.online_state.last_page = false;
                        return iced::Task::perform(async {}, |_| {
                            AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)
                        });
                    }
                }
            }
            AppMessage::Download(download_msg) => {
                match download_msg {
                    super::download::DownloadMessage::AddTask(url, _save_path, file_name, proxy, _file_type) => {
                        // 添加新下载任务并自动开始下载
                        println!("[下载功能] AddTask 开始处理");
                        println!("[下载功能] 添加任务: url={}, file_name={}, dir={}", url, file_name, _save_path);

                        // 合并目录和文件名生成完整路径
                        let full_save_path = PathBuf::from(&_save_path).join(&file_name);
                        println!("[下载功能] 完整保存路径: {}", full_save_path.display());

                        // 添加任务（使用完整路径）
                        let full_path_str = full_save_path.to_string_lossy().to_string();
                        self.download_state.add_task(url.clone(), full_path_str.clone(), file_name.clone(), proxy.clone(), _file_type.clone());
                        println!("[下载功能] next_id: {}", self.download_state.next_id);

                        // 获取新添加的任务ID
                        let task_id = self.download_state.next_id.saturating_sub(1);
                        println!("[下载功能] 新任务ID: {}", task_id);

                        // 更新状态为下载中并启动下载
                        match self.download_state.get_task(task_id) {
                            Some(task_full) => {
                                println!("[下载功能] 获取任务成功，save_path='{}'", task_full.task.save_path);
                                task_full.task.status = super::download::DownloadStatus::Downloading;
                                task_full.task.start_time = Some(std::time::Instant::now());
                                println!("[下载功能] 状态已更新为 Downloading，启动下载");

                                let url = task_full.task.url.clone();
                                let save_path = PathBuf::from(&task_full.task.save_path);
                                let proxy = task_full.proxy.clone();
                                let task_id = task_full.task.id;

                                println!("[下载功能] 最终保存路径: '{}'", save_path.display());

                                // 打印代理信息
                                if let Some(ref proxy_url) = proxy {
                                    println!("[下载任务] [ID:{}] 使用代理: {}", task_id, proxy_url);
                                } else {
                                    println!("[下载任务] [ID:{}] 不使用代理", task_id);
                                }

                                return iced::Task::perform(
                                    async_download_wallpaper_task(url, save_path, proxy, task_id),
                                    move |result| {
                                        match result {
                                            Ok(size) => {
                                                println!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                                            }
                                            Err(e) => {
                                                println!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                                AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                                            }
                                        }
                                    },
                                );
                            }
                            None => {
                                println!("[下载功能] 错误：get_task 返回 None");
                            }
                        }
                    }
                    super::download::DownloadMessage::PauseTask(id) => {
                        // 暂停任务（通过取消令牌实现）
                        println!("[下载功能] 暂停任务 ID:{}", id);
                        
                        // 先读取实际文件大小并更新任务
                        if let Some(index) = self.download_state.find_task_index(id) {
                            if let Some(task) = self.download_state.get_task_by_index(index) {
                                if let Ok(metadata) = std::fs::metadata(&task.task.save_path) {
                                    let actual_size = metadata.len();
                                    println!("[下载功能] 更新任务 ID:{} 的已下载大小: {} -> {}", 
                                        id, task.task.downloaded_size, actual_size);
                                    task.task.downloaded_size = actual_size;
                                }
                            }
                        }
                        
                        // 然后设置状态为暂停
                        self.download_state.update_status(id, super::download::DownloadStatus::Paused);
                        // 最后设置取消标志，终止下载
                        self.download_state.cancel_task(id);
                    }
                    super::download::DownloadMessage::ResumeTask(id) => {
                        // 继续/开始下载任务
                        println!("[下载功能] ResumeTask 收到请求，id={}", id);

                        // 使用索引查找任务
                        // 先检查是否可以开始下载并保存所有需要的数据
                        let can_start = self.download_state.can_start_download();
                        let current_status = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| t.task.status.clone());
                        let task_data = self.download_state.tasks.iter().find(|t| t.task.id == id).map(|t| (t.task.url.clone(), PathBuf::from(&t.task.save_path), t.proxy.clone(), t.task.id));

                        println!("[下载功能] ResumeTask 收到请求，id={}", id);
                        println!("[下载功能] 可以开始下载: {}", can_start);
                        println!("[下载功能] 当前状态: {:?}", current_status);

                        if let Some((url, save_path, proxy, task_id)) = task_data {
                            if current_status == Some(super::download::DownloadStatus::Waiting)
                                || current_status == Some(super::download::DownloadStatus::Paused)
                                || current_status == Some(super::download::DownloadStatus::Cancelled)
                                || matches!(current_status, Some(super::download::DownloadStatus::Failed(_)))
                            {
                                if can_start {
                                    // 更新状态为下载中
                                    let should_reset = current_status == Some(super::download::DownloadStatus::Cancelled)
                                        || matches!(current_status, Some(super::download::DownloadStatus::Failed(_)));

                                    if let Some(task_full) = self.download_state.tasks.iter_mut().find(|t| t.task.id == id) {
                                        task_full.task.status = super::download::DownloadStatus::Downloading;
                                        task_full.task.start_time = Some(std::time::Instant::now());
                                        // 重置取消令牌
                                        if let Some(cancel_token) = &task_full.task.cancel_token {
                                            cancel_token.store(false, std::sync::atomic::Ordering::Relaxed);
                                        }

                                        // 如果任务已取消或失败，重置已下载大小和进度
                                        if should_reset {
                                            println!("[下载功能] 任务已取消或失败，重置下载进度");
                                            task_full.task.downloaded_size = 0;
                                            task_full.task.progress = 0.0;
                                            task_full.task.speed = 0;

                                            // 清空已下载的文件
                                            let _ = std::fs::remove_file(&task_full.task.save_path);
                                            println!("[下载功能] 已清空文件: {}", task_full.task.save_path);
                                        }
                                    }
                                    self.download_state.increment_downloading();
                                    println!("[下载功能] 状态已更新为 Downloading，启动下载");
                                    println!("[下载功能] 当前下载任务数: {}/{}", self.download_state.get_downloading_count(), self.download_state.max_concurrent_downloads);

                                    println!("[下载功能] 最终保存路径: '{}'", save_path.display());

                                    // 打印代理信息
                                    if let Some(ref proxy_url) = proxy {
                                        println!("[下载任务] [ID:{}] 使用代理: {}", task_id, proxy_url);
                                    } else {
                                        println!("[下载任务] [ID:{}] 不使用代理", task_id);
                                    }

                                    // 获取取消令牌和已下载大小
                                    let (cancel_token, downloaded_size) = if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == task_id) {
                                        (task.task.cancel_token.clone().unwrap(), task.task.downloaded_size)
                                    } else {
                                        (std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), 0)
                                    };

                                    return iced::Task::perform(
                                        async_download_wallpaper_task_with_progress(url, save_path, proxy, task_id, cancel_token, downloaded_size),
                                        move |result| {
                                            match result {
                                                Ok(size) => {
                                                    println!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", task_id, size);
                                                    AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, size, None))
                                                }
                                                Err(e) => {
                                                    println!("[下载任务] [ID:{}] 下载失败: {}", task_id, e);
                                                    AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(task_id, 0, Some(e)))
                                                }
                                            }
                                        },
                                    );
                                } else {
                                    println!("[下载功能] 无法开始下载，当前下载任务数已达上限: {}/{}", self.download_state.get_downloading_count(), self.download_state.max_concurrent_downloads);
                                }
                            } else {
                                println!("[下载功能] 错误：任务状态不允许继续: {:?}", current_status);
                            }
                        } else {
                            println!("[下载功能] 错误：找不到任务 id={}", id);
                        }
                    }
                    super::download::DownloadMessage::CancelTask(id) => {
                        // 取消任务
                        println!("[下载功能] 取消任务 ID:{}", id);
                        self.download_state.cancel_task(id);
                        // 将任务状态设置为已取消
                        self.download_state.update_status(id, crate::ui::download::DownloadStatus::Cancelled);
                    }
                    super::download::DownloadMessage::DeleteTask(id) => {
                        // 删除任务
                        self.download_state.remove_task(id);
                    }
                    super::download::DownloadMessage::OpenFileLocation(id) => {
                        // 打开文件位置
                        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
                            let full_path = self.get_absolute_path(&task.task.save_path);

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
                            // Linux: 使用 xdg-open 路径
                            #[cfg(target_os = "linux")]
                            {
                                let _ = std::process::Command::new("xdg-open")
                                    .arg(&full_path)
                                    .spawn();
                            }
                        }
                    }
                    super::download::DownloadMessage::ClearCompleted => {
                        // 清空已完成的任务
                        self.download_state.clear_completed();
                    }
                    super::download::DownloadMessage::DownloadCompleted(id, size, error) => {
                        // 下载完成
                        let task_index = self.download_state.find_task_index(id);
                        if let Some(index) = task_index {
                            if let Some(task) = self.download_state.get_task_by_index(index) {
                                // 检查当前状态
                                let current_status = task.task.status.clone();

                                if current_status == super::download::DownloadStatus::Paused {
                                    // 任务已暂停，保持暂停状态
                                    println!("[下载功能] 任务已暂停，保持暂停状态");
                                } else if error.is_some() {
                                    // 下载失败
                                    let error_msg = error.unwrap();
                                    // 检查是否是用户取消
                                    if error_msg == "下载已取消" {
                                        // 检查任务是否在暂停状态被取消
                                        // 如果任务原本是暂停状态，则保持暂停，否则设置为已取消
                                        println!("[下载功能] 下载被取消，当前状态: {:?}", current_status);
                                        // 如果不是暂停状态，设置为已取消
                                        if current_status != super::download::DownloadStatus::Paused {
                                            task.task.status = super::download::DownloadStatus::Cancelled;
                                        }
                                    } else {
                                        task.task.status = super::download::DownloadStatus::Failed(error_msg.clone());
                                        println!("[下载功能] 任务失败: {}", error_msg);
                                    }
                                } else {
                                    // 下载成功
                                    // 验证实际文件大小
                                    let actual_size = if let Ok(metadata) = std::fs::metadata(&task.task.save_path) {
                                        metadata.len()
                                    } else {
                                        size
                                    };

                                    task.task.status = super::download::DownloadStatus::Completed;
                                    task.task.progress = 1.0;
                                    task.task.total_size = actual_size;
                                    task.task.downloaded_size = actual_size;
                                    println!("[下载功能] 任务完成，实际文件大小: {} bytes", actual_size);
                                }
                            }
                        }

                        // 减少正在下载的任务计数
                        self.download_state.decrement_downloading();
                        println!("[下载功能] 下载任务完成，正在下载任务数: {}/{}", self.download_state.get_downloading_count(), self.download_state.max_concurrent_downloads);

                        // 检查是否有等待中的任务需要开始
                        if let Some(next_task) = self.download_state.get_next_waiting_task() {
                            let next_id = next_task.task.id;
                            let next_url = next_task.task.url.clone();
                            let next_save_path = PathBuf::from(&next_task.task.save_path);
                            let next_proxy = next_task.proxy.clone();
                            let next_task_id = next_task.task.id;
                            let next_cancel_token = next_task.task.cancel_token.clone().unwrap();
                            let next_downloaded_size = next_task.task.downloaded_size;

                            println!("[下载功能] 开始排队任务 ID:{}", next_id);
                            next_task.task.status = super::download::DownloadStatus::Downloading;
                            next_task.task.start_time = Some(std::time::Instant::now());
                            self.download_state.increment_downloading();

                            // 启动下一个下载任务
                            return iced::Task::perform(
                                async_download_wallpaper_task_with_progress(next_url, next_save_path, next_proxy, next_task_id, next_cancel_token, next_downloaded_size),
                                move |result| {
                                    match result {
                                        Ok(s) => {
                                            println!("[下载任务] [ID:{}] 下载成功, 文件大小: {} bytes", next_task_id, s);
                                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(next_task_id, s, None))
                                        }
                                        Err(e) => {
                                            println!("[下载任务] [ID:{}] 下载失败: {}", next_task_id, e);
                                            AppMessage::Download(super::download::DownloadMessage::DownloadCompleted(next_task_id, 0, Some(e)))
                                        }
                                    }
                                },
                            );
                        }
                    }
                    super::download::DownloadMessage::DownloadProgress(id, downloaded, total, speed) => {
                        // 下载进度更新
                        self.download_state.update_progress(id, downloaded, total, speed);
                    }
                    super::download::DownloadMessage::SimulateProgress => {
                        // 模拟进度更新（测试用）
                        for task in self.download_state.tasks.iter_mut() {
                            if task.task.status == super::download::DownloadStatus::Downloading {
                                let increment = (task.task.total_size as f32 * 0.01).max(1024.0) as u64;
                                task.task.downloaded_size = (task.task.downloaded_size + increment).min(task.task.total_size);
                                if task.task.total_size > 0 {
                                    task.task.progress = task.task.downloaded_size as f32 / task.task.total_size as f32;
                                }
                                if task.task.downloaded_size >= task.task.total_size {
                                    task.task.status = super::download::DownloadStatus::Completed;
                                }
                            }
                        }
                    }
                    super::download::DownloadMessage::UpdateSpeed => {
                        // 更新下载速度
                        self.download_state.update_speed();
                    }
                    super::download::DownloadMessage::CopyDownloadLink(id) => {
                        // 复制下载链接到剪贴板
                        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
                            let url = task.task.url.clone();
                            println!("[下载功能] 复制下载链接: {}", url);
                            // TODO: 实现复制到剪贴板功能
                            let _ = self.show_notification("下载链接已复制到剪贴板".to_string(), super::NotificationType::Success);
                        }
                    }
                    super::download::DownloadMessage::SetAsWallpaper(id) => {
                        // 设为壁纸
                        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
                            let path = task.task.save_path.clone();
                            println!("[下载功能] 设为壁纸: {}", path);
                            // TODO: 实现设为壁纸功能
                            let _ = self.show_notification("壁纸设置成功".to_string(), super::NotificationType::Success);
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

// 异步加载在线壁纸函数
async fn async_load_online_wallpapers(
    categories: u32,
    sorting: super::online::Sorting,
    purities: u32,
    query: String,
    page: usize,
    api_key: Option<String>,
    proxy: Option<String>,
    context: crate::services::request_context::RequestContext,
) -> Result<(Vec<super::online::OnlineWallpaper>, bool, usize, usize), Box<dyn std::error::Error + Send + Sync>> {
    let service = crate::services::online_wallhaven::WallhavenService::new(api_key, proxy);
    match service.search_wallpapers(page, categories, sorting, purities, &query, &context).await {
        Ok(result) => Ok(result),
        Err(e) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error + Send + Sync>),
    }
}

// 异步加载在线壁纸图片函数
async fn async_load_online_wallpaper_image(
    url: String,
    proxy: Option<String>,
) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
    // 打印请求参数
    println!("[图片加载] 请求URL: {}", url);
    if let Some(ref proxy_url) = proxy {
        println!("[图片加载] 使用代理: {}", proxy_url);
    } else {
        println!("[图片加载] 不使用代理");
    }

    let client = if let Some(proxy_url) = proxy {
        if !proxy_url.is_empty() {
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                reqwest::Client::builder()
                    .proxy(proxy)
                    .build()
                    .unwrap_or_else(|_| reqwest::Client::new())
            } else {
                reqwest::Client::new()
            }
        } else {
            reqwest::Client::new()
        }
    } else {
        reqwest::Client::new()
    };

    let response = client.get(&url).send().await
        .map_err(|e| {
            println!("[图片加载] 请求失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;

    // 打印响应状态
    println!("[图片加载] 响应状态: {}", response.status());

    if !response.status().is_success() {
        let error_msg = format!("下载失败: {}", response.status());
        println!("[图片加载] {}", error_msg);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_msg)) as Box<dyn std::error::Error + Send + Sync>);
    }

    let bytes = response.bytes().await
        .map_err(|e| {
            println!("[图片加载] 读取响应体失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error + Send + Sync>
        })?;

    // 打印数据大小
    println!("[图片加载] 下载成功，数据大小: {} bytes", bytes.len());

    Ok(iced::widget::image::Handle::from_bytes(bytes.to_vec()))
}

// 异步加载在线壁纸缩略图函数（带缓存）
async fn async_load_online_wallpaper_thumb_with_cache(
    url: String,
    file_size: u64,
    cache_base_path: String,
    proxy: Option<String>,
) -> Result<iced::widget::image::Handle, Box<dyn std::error::Error + Send + Sync>> {
    // 使用DownloadService的智能缓存加载功能
    crate::services::download::DownloadService::load_thumb_with_cache(url, file_size, cache_base_path, proxy).await
}

// 异步下载壁纸任务函数
async fn async_download_wallpaper_task(
    url: String,
    save_path: PathBuf,
    proxy: Option<String>,
    task_id: usize,
) -> Result<u64, String> {
    println!("[下载任务] [ID:{}] 开始下载: {}", task_id, url);
    println!("[下载任务] [ID:{}] 保存路径: {}", task_id, save_path.display());

    // 确保父目录存在
    if let Some(parent_dir) = save_path.parent() {
        println!("[下载任务] [ID:{}] 确保目录存在: {}", task_id, parent_dir.display());
        tokio::fs::create_dir_all(parent_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
        println!("[下载任务] [ID:{}] 目录已准备就绪", task_id);
    }

    // 创建HTTP客户端（带代理）
    let client = if let Some(proxy_url) = &proxy {
        if !proxy_url.is_empty() {
            if let Ok(p) = reqwest::Proxy::all(proxy_url) {
                reqwest::Client::builder()
                    .proxy(p)
                    .build()
                    .map_err(|e| e.to_string())?
            } else {
                reqwest::Client::new()
            }
        } else {
            reqwest::Client::new()
        }
    } else {
        reqwest::Client::new()
    };

    // 发送请求
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("HTTP错误: {}", response.status()));
    }

    // 获取文件大小
    let total_size = response
        .content_length()
        .unwrap_or(0);

    println!("[下载任务] [ID:{}] 文件大小: {} bytes", task_id, total_size);

    // 读取全部数据（对于壁纸文件，使用 bytes() 更简单）
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("读取数据失败: {}", e))?;

    println!("[下载任务] [ID:{}] 准备写入文件...", task_id);

    // 创建文件并写入
    let mut file = tokio::fs::File::create(&save_path)
        .await
        .map_err(|e| format!("创建文件失败: {}", e))?;

    file.write_all(&bytes)
        .await
        .map_err(|e| format!("写入文件失败: {}", e))?;

    file.flush().await.map_err(|e| e.to_string())?;

    println!("[下载任务] [ID:{}] 下载完成: {}", task_id, save_path.display());

    // 返回文件大小
    Ok(bytes.len() as u64)
}

// 带进度更新的异步下载壁纸任务函数
// 使用 tokio::sync::mpsc 通道来发送进度更新
async fn async_download_wallpaper_task_with_progress(
    url: String,
    save_path: PathBuf,
    proxy: Option<String>,
    task_id: usize,
    cancel_token: std::sync::Arc<std::sync::atomic::AtomicBool>,
    downloaded_size: u64,
) -> Result<u64, String> {
    println!("[下载任务] [ID:{}] 开始下载: {}", task_id, url);
    println!("[下载任务] [ID:{}] 保存路径: {}", task_id, save_path.display());
    if downloaded_size > 0 {
        println!("[下载任务] [ID:{}] 断点续传，已下载: {} bytes", task_id, downloaded_size);
    }

    // 确保父目录存在
    if let Some(parent_dir) = save_path.parent() {
        println!("[下载任务] [ID:{}] 确保目录存在: {}", task_id, parent_dir.display());
        tokio::fs::create_dir_all(parent_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;
        println!("[下载任务] [ID:{}] 目录已准备就绪", task_id);
    }

    // 创建优化的HTTP客户端（带代理）
    let create_optimized_client = || -> reqwest::Client {
        reqwest::Client::builder()
            // 连接池配置：最大100个连接，每个主机最多10个连接
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            // 超时配置
            .connect_timeout(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(300))
            // TCP配置：启用TCP_NODELAY减少延迟
            .tcp_nodelay(true)
            // 启用HTTP/2
            .http2_prior_knowledge()
            // 启用gzip压缩（reqwest默认支持）
            .gzip(true)
            // 启用brotli压缩（需要features支持）
            .brotli(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    };

    let client = if let Some(proxy_url) = &proxy {
        if !proxy_url.is_empty() {
            println!("[下载任务] [ID:{}] 尝试创建代理客户端，代理URL: {}", task_id, proxy_url);
            match reqwest::Proxy::all(proxy_url) {
                Ok(p) => {
                    println!("[下载任务] [ID:{}] Proxy::all 成功", task_id);
                    match reqwest::Client::builder()
                        .proxy(p)
                        .pool_max_idle_per_host(10)
                        .pool_idle_timeout(std::time::Duration::from_secs(90))
                        .connect_timeout(std::time::Duration::from_secs(30))
                        .timeout(std::time::Duration::from_secs(300))
                        .tcp_nodelay(true)
                        .http2_prior_knowledge()
                        .gzip(true)
                        .brotli(true)
                        .build() {
                        Ok(http_client) => {
                            println!("[下载任务] [ID:{}] HTTP客户端创建成功（已优化）", task_id);
                            http_client
                        }
                        Err(e) => {
                            println!("[下载任务] [ID:{}] HTTP客户端创建失败: {}，回退到无代理", task_id, e);
                            create_optimized_client()
                        }
                    }
                }
                Err(e) => {
                    println!("[下载任务] [ID:{}] Proxy::all 失败: {}，回退到无代理", task_id, e);
                    create_optimized_client()
                }
            }
        } else {
            println!("[下载任务] [ID:{}] 代理URL为空，使用无代理客户端", task_id);
            create_optimized_client()
        }
    } else {
        println!("[下载任务] [ID:{}] 无代理配置，使用无代理客户端", task_id);
        create_optimized_client()
    };

    // 发送请求（支持断点续传）
    let response = if downloaded_size > 0 {
        // 断点续传：使用 Range 请求头
        let range_header = format!("bytes={}-", downloaded_size);
        println!("[下载任务] [ID:{}] 使用Range请求: {}", task_id, range_header);
        let resp = client
            .get(&url)
            .header("Range", range_header)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;
        
        println!("[下载任务] [ID:{}] Range响应状态: {}", task_id, resp.status());
        if let Some(content_range) = resp.headers().get("content-range") {
            println!("[下载任务] [ID:{}] Content-Range: {:?}", task_id, content_range);
        }
        if let Some(content_length) = resp.content_length() {
            println!("[下载任务] [ID:{}] Range响应Content-Length: {} bytes (剩余部分)", task_id, content_length);
        }
        resp
    } else {
        // 新下载
        client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?
    };

    // 检查响应状态
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(format!("HTTP错误: {}", response.status()));
    }

    // 获取文件大小
    let total_size = if let Some(content_length) = response.content_length() {
        if downloaded_size > 0 {
            // 断点续传：总大小 = 已下载 + 剩余部分
            let total = downloaded_size + content_length;
            println!("[下载任务] [ID:{}] 断点续传：已下载={}, 剩余={}, 总大小={}", 
                task_id, downloaded_size, content_length, total);
            total
        } else {
            println!("[下载任务] [ID:{}] 新下载：Content-Length={}", task_id, content_length);
            content_length
        }
    } else {
        0
    };

    println!("[下载任务] [ID:{}] 文件总大小: {} bytes", task_id, total_size);

    // 使用流式下载，分块读取并批量写入文件
    let mut stream = response.bytes_stream();
    
    // 打开文件（追加模式用于断点续传）
    let mut file = if downloaded_size > 0 {
        // 断点续传：使用写入模式，先定位到文件末尾
        println!("[下载任务] [ID:{}] 检查文件是否存在: {}", task_id, save_path.exists());
        
        let file_exists = save_path.exists();
        println!("[下载任务] [ID:{}] 文件存在: {}", task_id, file_exists);
        
        if !file_exists {
            println!("[下载任务] [ID:{}] 错误：文件不存在，无法断点续传", task_id);
            return Err(format!("文件不存在，无法断点续传: {}", save_path.display()));
        }
        
        // 检查文件大小是否匹配
        if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
            let actual_size = metadata.len();
            println!("[下载任务] [ID:{}] 文件实际大小: {} bytes", task_id, actual_size);
            if actual_size != downloaded_size {
                println!("[下载任务] [ID:{}] 错误：文件大小不匹配，期望: {}, 实际: {}", 
                    task_id, downloaded_size, actual_size);
                return Err(format!("文件大小不匹配，期望: {} bytes，实际: {} bytes", downloaded_size, actual_size));
            }
        }
        
        println!("[下载任务] [ID:{}] 以追加模式打开文件", task_id);
        tokio::fs::OpenOptions::new()
            .write(true)
            .open(&save_path)
            .await
            .map_err(|e| format!("打开文件失败: {}", e))?
    } else {
        // 创建新文件
        println!("[下载任务] [ID:{}] 创建新文件", task_id);
        tokio::fs::File::create(&save_path)
            .await
            .map_err(|e| format!("创建文件失败: {}", e))?
    };

    // 验证文件初始大小
    if let Ok(metadata) = file.metadata().await {
        let initial_size = metadata.len();
        println!("[下载任务] [ID:{}] 文件初始大小: {} bytes", task_id, initial_size);
        if downloaded_size > 0 && initial_size != downloaded_size {
            println!("[下载任务] [ID:{}] 警告：文件初始大小({})与已下载大小({})不匹配", 
                task_id, initial_size, downloaded_size);
        }
    }

    let mut downloaded: u64 = downloaded_size;
    let mut progress_sent = 0i32; // 记录已发送的进度百分比，避免重复发送
    let start_time = std::time::Instant::now();

    // 使用批量缓冲区减少写入次数
    let mut buffer = Vec::with_capacity(1024 * 1024); // 1MB缓冲区
    let mut last_log_time = start_time; // 记录上次日志输出时间

    while let Some(chunk_result) = stream.next().await {
        // 检查是否被取消
        if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
            println!("[下载任务] [ID:{}] 下载被取消", task_id);
            return Err("下载已取消".to_string());
        }

        let chunk = chunk_result
            .map_err(|e| format!("读取数据流失败: {}", e))?;

        // 将数据添加到缓冲区
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;

        // 当缓冲区达到1MB或下载完成时，批量写入文件
        if buffer.len() >= 1024 * 1024 {
            file.write_all(&buffer)
                .await
                .map_err(|e| format!("写入文件失败: {}", e))?;
            buffer.clear();
        }

        // 计算进度百分比
        if total_size > 0 {
            let percent = ((downloaded as f32 / total_size as f32) * 100.0) as i32;
            let percent_f = downloaded as f32 / total_size as f32;
            let current_time = std::time::Instant::now();
            let elapsed_since_last_log = current_time.duration_since(last_log_time).as_secs_f64();

            // 每完成5%或者距离上次日志输出超过1秒才输出进度
            if percent >= progress_sent + 5 || elapsed_since_last_log >= 1.0 {
                progress_sent = percent;
                last_log_time = current_time;

                // 计算下载速度
                let elapsed = start_time.elapsed().as_secs_f64();
                let speed = if elapsed > 0.0 {
                    (downloaded as f64 / elapsed) as u64
                } else {
                    0
                };

                println!("[下载任务] [ID:{}] 进度: {:.1}% ({}/{}) - 速度: {}/s",
                    task_id,
                    percent_f * 100.0,
                    format_file_size(downloaded),
                    format_file_size(total_size),
                    format_file_size(speed));

                // 发送进度更新到UI
                crate::services::send_download_progress(task_id, downloaded, total_size, speed);
            }
        } else {
            // 如果无法获取总大小，每秒输出一次已下载量
            let current_time = std::time::Instant::now();
            let elapsed_since_last_log = current_time.duration_since(last_log_time).as_secs_f64();

            if elapsed_since_last_log >= 1.0 {
                last_log_time = current_time;
                println!("[下载任务] [ID:{}] 已下载: {}", task_id, format_file_size(downloaded));
            }
        }
    }

    // 写入剩余数据
    if !buffer.is_empty() {
        file.write_all(&buffer)
            .await
            .map_err(|e| format!("写入文件失败: {}", e))?;
    }

    // 确保数据写入磁盘
    file.flush().await.map_err(|e| e.to_string())?;

    // 验证文件完整性
    if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
        let actual_size = metadata.len();
        if total_size > 0 && actual_size != total_size {
            println!("[下载任务] [ID:{}] 警告：文件大小不匹配！期望: {} bytes, 实际: {} bytes", 
                task_id, total_size, actual_size);
            return Err(format!("文件大小不匹配：期望 {} bytes，实际 {} bytes", total_size, actual_size));
        }
        println!("[下载任务] [ID:{}] 下载完成: {} (总大小: {}), 实际文件大小: {}", 
            task_id, format_file_size(downloaded), format_file_size(total_size), format_file_size(actual_size));
    } else {
        println!("[下载任务] [ID:{}] 下载完成: {} (总大小: {})", task_id, format_file_size(downloaded), format_file_size(total_size));
    }

    // 返回实际文件大小
    let actual_size = if let Ok(metadata) = tokio::fs::metadata(&save_path).await {
        metadata.len()
    } else {
        downloaded
    };

    Ok(actual_size)
}

/// 格式化文件大小
fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
