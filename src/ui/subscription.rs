// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, AppMessage};
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::download::DownloadMessage;
use crate::ui::main::MainMessage;
use iced::{Event, Subscription, event, window};
use std::time::Duration;

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

impl App {
    /// 订阅事件
    pub fn subscription(&self) -> Subscription<AppMessage> {
        // 定时更新壁纸任务
        let auto_change_background = if self.auto_change_state.auto_change_enabled {
            let minutes = self.config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30);
            // Iced 的这个定时器非常智能：
            // 如果 minutes 变了，生成的 Subscription ID 就会变，旧的定时器会被自动替换
            iced::time::every(Duration::from_secs(minutes as u64 * 60))
                .map(|_| AutoChangeMessage::AutoChangeTick.into())
        } else {
            Subscription::none()
        };

        // 定时检测系统颜色模式任务
        let auto_detect_color_mode = if self.auto_change_state.auto_detect_color_mode && self.main_state.is_visible {
            iced::time::every(Duration::from_secs(1)).map(|_| MainMessage::AutoDetectColorModeTick.into())
        } else {
            Subscription::none()
        };

        Subscription::batch(vec![
            // 窗口事件监听
            event::listen_with(|event, _status, _loop_status| match event {
                Event::Window(window::Event::Resized(size)) => {
                    Some(MainMessage::WindowResized(size.width as u32, size.height as u32).into())
                }
                Event::Window(window::Event::CloseRequested) => Some(MainMessage::WindowCloseRequested.into()),
                Event::Window(window::Event::Focused) => Some(MainMessage::WindowFocused.into()),
                _ => None,
            }),
            // 托盘事件监听
            Subscription::run(|| {
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
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }),
            // 添加定时切换壁纸定时器
            auto_change_background,
            // 添加自动检测颜色模式定时器
            auto_detect_color_mode,
            // 添加下载进度监听 - 使用run_with
            Subscription::run_with(DownloadProgressSubscription, |_state| {
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
}
