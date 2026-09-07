// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, AppMessage};
use crate::ui::download::DownloadMessage;
use crate::ui::main::MainMessage;
use iced::{Event, Subscription, event, window};

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

        // 系统颜色模式变化监听（事件驱动：Windows 注册表通知 / Linux D-Bus 信号，
        // macOS 由 dark-light 内部线程轮询），替代逐秒轮询
        let auto_detect_color_mode =
            if self.auto_change_state.auto_detect_color_mode && self.main_state.is_visible {
                Subscription::run(|| {
                    async_stream::stream! {
                        // Watcher 仅上报变化事件：先同步一次当前模式
                        yield MainMessage::AutoDetectColorModeTick.into();

                        let Ok(watcher) = crate::platform::subscribe_color_mode() else {
                            tracing::warn!("[主题监听] 订阅系统颜色模式变化失败，自动检测停用");
                            return;
                        };

                        // recv 为阻塞调用（注册表通知/D-Bus 等待），经专用线程转发，
                        // 避免阻塞订阅执行器线程
                        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
                        std::thread::spawn(move || {
                            while watcher.recv().is_some() {
                                if tx.send(()).is_err() {
                                    break;
                                }
                            }
                        });

                        while rx.recv().await.is_some() {
                            yield MainMessage::AutoDetectColorModeTick.into();
                        }
                    }
                })
            } else {
                Subscription::none()
            };

        // 热键录制激活时：追加全量键盘事件订阅（基础订阅过滤了无修饰键的字符键，
        // 录制需要捕获任意主键，如单独按 A）
        let hotkey_recording = if self.settings_state.hotkey_recording.is_some() {
            event::listen_with(|event, _status, window_id| match event {
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key, modifiers, ..
                }) => Some(MainMessage::KeyEvent(window_id, key, modifiers).into()),
                _ => None,
            })
        } else {
            Subscription::none()
        };

        Subscription::batch(vec![
            // 窗口与键盘事件监听（携带窗口Id，由处理器按主窗口/悬浮球过滤）
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
                // 键盘事件过滤：仅转发功能键（Esc/方向键/F1-F12）与组合键，
                // 纯字符按键（打字场景）不转发，避免文本输入产生额外消息
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key, modifiers, ..
                }) => {
                    let forwarded = match &key {
                        iced::keyboard::Key::Named(name) => {
                            matches!(
                                name,
                                iced::keyboard::key::Named::Escape
                                    | iced::keyboard::key::Named::ArrowLeft
                                    | iced::keyboard::key::Named::ArrowRight
                                    | iced::keyboard::key::Named::F1
                                    | iced::keyboard::key::Named::F2
                                    | iced::keyboard::key::Named::F3
                                    | iced::keyboard::key::Named::F4
                                    | iced::keyboard::key::Named::F5
                                    | iced::keyboard::key::Named::F6
                                    | iced::keyboard::key::Named::F7
                                    | iced::keyboard::key::Named::F8
                                    | iced::keyboard::key::Named::F9
                                    | iced::keyboard::key::Named::F10
                                    | iced::keyboard::key::Named::F11
                                    | iced::keyboard::key::Named::F12
                            )
                        }
                        _ => false,
                    } || !modifiers.is_empty();
                    forwarded.then_some(MainMessage::KeyEvent(window_id, key, modifiers).into())
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
                    let tray_tx = tx.clone();
                    std::thread::spawn(move || {
                        let receiver = TrayIconEvent::receiver();
                        while let Ok(event) = receiver.recv() {
                            if let TrayIconEvent::DoubleClick { .. } = event
                                && tray_tx.send(MainMessage::TrayIconClicked.into()).is_err()
                            {
                                break;
                            }
                        }
                    });

                    // 全局热键事件（按下与释放都会上报，仅转发按下）
                    std::thread::spawn(move || {
                        use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

                        let receiver = GlobalHotKeyEvent::receiver();
                        while let Ok(event) = receiver.recv() {
                            if event.state == HotKeyState::Pressed {
                                let hotkey_event = crate::utils::hotkey_manager::HotkeyEvent {
                                    id: event.id,
                                    state: event.state,
                                };
                                if tx.send(MainMessage::HotkeyTriggered(hotkey_event).into())
                                    .is_err()
                                {
                                    break;
                                }
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
            // 热键录制中的全量键盘监听
            hotkey_recording,
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
