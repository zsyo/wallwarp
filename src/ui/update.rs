// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use tracing::warn;

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

impl App {
    /// 订阅事件
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        use iced::event;
        use iced::window;

        iced::Subscription::batch([
            // 窗口事件监听
            event::listen_with(|event, _status, _loop_status| match event {
                iced::Event::Window(window::Event::Resized(size)) => {
                    Some(AppMessage::WindowResized(size.width as u32, size.height as u32))
                }
                iced::Event::Window(window::Event::CloseRequested) => Some(AppMessage::WindowCloseRequested),
                _ => None,
            }),
            // 托盘事件监听
            iced::Subscription::run(|| {
                use tray_icon::{TrayIconEvent, menu::MenuEvent};

                async_stream::stream! {
                    loop {
                        // 1. 消耗并发送所有菜单事件
                        while let Ok(menu_event) = MenuEvent::receiver().try_recv() {
                            yield AppMessage::TrayMenuEvent(menu_event.id.0);
                        }

                        // 2. 消耗并发送所有托盘图标事件
                        while let Ok(tray_event) = TrayIconEvent::receiver().try_recv() {
                            if let TrayIconEvent::DoubleClick { .. } = tray_event {
                                yield AppMessage::TrayIconClicked;
                            }
                        }

                        // 3. 这里的休眠保证了即使窗口最小化，后台流依然以 10hz 频率清理队列
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }),
            // 添加定时切换壁纸定时器 - 每秒检查一次是否需要切换壁纸
            iced::time::every(std::time::Duration::from_secs(1))
                .map(|_| AppMessage::Local(super::local::LocalMessage::AutoChangeTick)),
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
            AppMessage::Local(local_message) => {
                return self.handle_local_message(local_message);
            }
            AppMessage::Online(online_message) => {
                return self.handle_online_message(online_message);
            }
            AppMessage::Download(download_message) => {
                return self.handle_download_message(download_message);
            }
            _ => {
                // 其他消息交给 settings_handlers 处理
                warn!("other message running...");
                return self.handle_settings_message(msg);
            }
        }

        iced::Task::none()
    }
}
