// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::download::DownloadMessage;
use crate::ui::main::MainMessage;
use crate::ui::online::OnlineMessage;

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

impl App {
    /// 订阅事件
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        use iced::event;
        use iced::window;

        // 定时更新壁纸任务
        let auto_change_background = if self.auto_change_state.auto_change_enabled {
            let minutes = self.config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30);
            // Iced 的这个定时器非常智能：
            // 如果 minutes 变了，生成的 Subscription ID 就会变，旧的定时器会被自动替换
            iced::time::every(std::time::Duration::from_secs(minutes as u64 * 60))
                .map(|_| AutoChangeMessage::AutoChangeTick.into())
        } else {
            iced::Subscription::none()
        };

        // 定时检测系统颜色模式任务
        let auto_detect_color_mode = if self.auto_change_state.auto_detect_color_mode && self.is_visible {
            iced::time::every(std::time::Duration::from_secs(1)).map(|_| MainMessage::AutoDetectColorModeTick.into())
        } else {
            iced::Subscription::none()
        };

        iced::Subscription::batch(vec![
            // 窗口事件监听
            event::listen_with(|event, _status, _loop_status| match event {
                iced::Event::Window(window::Event::Resized(size)) => {
                    Some(MainMessage::WindowResized(size.width as u32, size.height as u32).into())
                }
                iced::Event::Window(window::Event::CloseRequested) => Some(MainMessage::WindowCloseRequested.into()),
                iced::Event::Window(window::Event::Focused) => Some(MainMessage::WindowFocused.into()),
                _ => None,
            }),
            // 托盘事件监听
            iced::Subscription::run(|| {
                use tray_icon::{TrayIconEvent, menu::MenuEvent};

                async_stream::stream! {
                    loop {
                        // 1. 消耗并发送所有菜单事件
                        while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
                            yield MainMessage::TrayMenuEvent(menu_event.id.0).into();
                        }

                        // 2. 消耗并发送所有托盘图标事件
                        while let Ok(tray_event) = TrayIconEvent::receiver().try_recv() {
                            if let TrayIconEvent::DoubleClick { .. } = tray_event {
                                yield MainMessage::TrayIconClicked.into();
                            }
                        }

                        // 3. 这里的休眠保证了即使窗口最小化，后台流依然以 10hz 频率清理队列
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }),
            // 添加定时切换壁纸定时器
            auto_change_background,
            // 添加自动检测颜色模式定时器
            auto_detect_color_mode,
            // 添加下载进度监听 - 使用run_with
            iced::Subscription::run_with(DownloadProgressSubscription, |_state| {
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
                                    yield DownloadMessage::DownloadProgress(update.task_id,update.downloaded,update.total,update.speed).into();
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
            }),
        ])
    }

    /// 主更新方法 - 处理所有应用消息
    pub fn update(&mut self, msg: AppMessage) -> iced::Task<AppMessage> {
        // 检查是否需要加载初始任务（只在第一次运行时）
        if !self.initial_loaded {
            self.initial_loaded = true;
            // 如果默认页面是在线壁纸，则加载初始数据
            if self.active_page == super::ActivePage::OnlineWallpapers {
                return iced::Task::batch(vec![
                    iced::Task::done(OnlineMessage::LoadWallpapers.into()),
                    iced::Task::done(MainMessage::ScrollToTop("online_wallpapers_scroll".to_string()).into()),
                ]);
            }
        }

        match msg {
            AppMessage::None => {
                // 空消息，不做任何操作
            }
            AppMessage::Main(main_message) => {
                return self.handle_main_message(main_message);
            }
            AppMessage::Local(local_message) => {
                return self.handle_local_message(local_message);
            }
            AppMessage::Online(online_message) => {
                return self.handle_online_message(online_message);
            }
            AppMessage::Download(download_message) => {
                return self.handle_download_message(download_message);
            }
            AppMessage::AutoChange(auto_change_message) => {
                return self.handle_auto_change_message(auto_change_message);
            }
            _ => {
                // 其他消息交给 main_handlers 处理
                return self.handle_settings_message(msg);
            }
        }

        iced::Task::none()
    }
}
