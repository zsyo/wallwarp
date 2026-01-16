use super::App;
use super::AppMessage;
use std::time::Duration;

// 用于下载进度订阅的唯一类型标识
#[derive(std::hash::Hash)]
struct DownloadProgressSubscription;

impl App {
    /// 订阅事件
    pub fn subscription(&self) -> iced::Subscription<AppMessage> {
        use iced::event;
        use iced::time;
        use iced::window;

        iced::Subscription::batch([
            event::listen_with(|event, _status, _loop_status| match event {
                iced::Event::Window(window::Event::Resized(size)) => Some(AppMessage::WindowResized(size.width as u32, size.height as u32)),
                iced::Event::Window(window::Event::CloseRequested) => {
                    // 发送一个关闭请求消息，让App处理
                    Some(AppMessage::WindowCloseRequested)
                }
                _ => None,
            }),
            time::every(Duration::from_millis(50)).map(|_| AppMessage::DebounceTimer),
            // 添加动画定时器 - 每100毫秒更新一次旋转角度
            time::every(Duration::from_millis(100)).map(|_| AppMessage::Local(super::local::LocalMessage::AnimationTick)),
            // 添加动态图帧更新定时器 - 每50毫秒更新一次
            time::every(Duration::from_millis(50)).map(|_| AppMessage::Local(super::local::LocalMessage::AnimatedFrameUpdate)),
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
                    iced::Task::perform(async {}, |_| AppMessage::Online(super::online::OnlineMessage::LoadWallpapers)),
                    iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("online_wallpapers_scroll".to_string())),
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
                return self.handle_settings_message(msg);
            }
        }

        iced::Task::none()
    }
}
