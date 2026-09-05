// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, AppMessage};
use crate::ui::download::DownloadMessage;
use crate::ui::main::MainMessage;
use iced::{Event, Subscription, event, window};
use std::time::Duration;

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

// 用于模态窗口图片下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct ModalImageProgressSubscription;

impl App {
    /// 订阅事件
    pub fn subscription(&self) -> Subscription<AppMessage> {
        // 定时更新壁纸任务
        let auto_change_background = if self.auto_change_state.auto_change_enabled {
            match self.auto_change_state.next_execute_time {
                Some(dt) => {
                    let ts = dt.timestamp();
                    Subscription::run_with(ts, |id| Self::create_timer_stream(*id))
                }
                None => Subscription::none(),
            }
        } else {
            Subscription::none()
        };

        // 定时检测系统颜色模式任务
        let auto_detect_color_mode =
            if self.auto_change_state.auto_detect_color_mode && self.main_state.is_visible {
                iced::time::every(Duration::from_secs(1))
                    .map(|_| MainMessage::AutoDetectColorModeTick.into())
            } else {
                Subscription::none()
            };

        Subscription::batch(vec![
            // 窗口事件监听（携带窗口Id，由处理器按主窗口/悬浮球过滤）
            event::listen_with(|event, _status, window_id| match event {
                Event::Window(window::Event::Resized(size)) => Some(
                    MainMessage::WindowResized(window_id, size.width as u32, size.height as u32)
                        .into(),
                ),
                Event::Window(window::Event::CloseRequested) => {
                    Some(MainMessage::WindowCloseRequested(window_id).into())
                }
                Event::Window(window::Event::Focused) => {
                    Some(MainMessage::WindowFocused(window_id).into())
                }
                Event::Window(window::Event::Moved(pos)) => {
                    Some(MainMessage::WindowMoved(window_id, pos).into())
                }
                _ => None,
            }),
            // 托盘事件监听（事件驱动：专用线程阻塞接收后转发，空闲时零唤醒）
            Subscription::run(|| {
                use tray_icon::{TrayIconEvent, menu::MenuEvent};

                async_stream::stream! {
                    // muda/tray-icon 的接收端是全局同步 channel，无法直接 .await；
                    // 用两个专用线程阻塞 recv 并转发到 async channel
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AppMessage>();

                    // 菜单事件（托盘菜单 + 悬浮球菜单共用 muda 全局通道）
                    let menu_tx = tx.clone();
                    std::thread::spawn(move || {
                        let receiver = MenuEvent::receiver();
                        while let Ok(event) = receiver.recv() {
                            if menu_tx
                                .send(MainMessage::TrayMenuEvent(event.id.0).into())
                                .is_err()
                            {
                                break;
                            }
                        }
                    });

                    // 托盘图标事件（双击显示主窗口）
                    std::thread::spawn(move || {
                        let receiver = TrayIconEvent::receiver();
                        while let Ok(event) = receiver.recv() {
                            if let TrayIconEvent::DoubleClick { .. } = event
                                && tx.send(MainMessage::TrayIconClicked.into()).is_err()
                            {
                                break;
                            }
                        }
                    });

                    while let Some(message) = rx.recv().await {
                        yield message;
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
                let rx = crate::services::DOWNLOAD_PROGRESS_TX
                    .get()
                    .map(|tx| tx.subscribe());

                async_stream::stream! {
                    if let Some(mut rx) = rx {
                        // Channel关闭时 recv 返回 Err，循环自然结束
                        while let Ok(update) = rx.recv().await {
                            yield DownloadMessage::DownloadProgress(update.task_id,update.downloaded,update.total,update.speed).into();
                        }
                    } else {
                        // 如果channel未初始化，返回空stream
                        std::future::pending::<()>().await;
                    }
                }
            }),
            // 添加模态窗口图片下载进度监听
            Subscription::run_with(ModalImageProgressSubscription, |_state| {
                // 初始化模态图片进度channel
                crate::services::init_modal_image_progress_channel();

                let rx = crate::services::MODAL_IMAGE_PROGRESS_TX
                    .get()
                    .map(|tx| tx.subscribe());

                async_stream::stream! {
                    if let Some(mut rx) = rx {
                        while let Ok((downloaded, total)) = rx.recv().await {
                            yield crate::ui::online::OnlineMessage::ModalImageProgress(
                                downloaded, total,
                            )
                            .into();
                        }
                    } else {
                        std::future::pending::<()>().await;
                    }
                }
            }),
        ])
    }
}
