use super::App;
use super::message::AppMessage;
use std::time::Duration;
use crate::utils::startup;

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
                _ => None,
            }),
            time::every(Duration::from_millis(100)).map(|_| AppMessage::DebounceTimer),
        ])
    }

    pub fn update(&mut self, msg: AppMessage) {
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
                // 检查是否需要执行延迟的保存操作
                let elapsed = self.debounce_timer.elapsed();
                if elapsed >= Duration::from_millis(300) {
                    let mut needs_save = false;

                    // 保存窗口大小
                    if let Some((width, height)) = self.pending_window_size.take() {
                        self.config.update_window_size(width, height);
                        needs_save = true;
                    }

                    // 保存窗口位置
                    if let Some((x, y)) = self.pending_window_position.take() {
                        self.config.update_window_position(x, y);
                        needs_save = true;
                    }

                    if needs_save {
                        println!("窗口尺寸或位置已保存");
                    }
                }
            }
            AppMessage::AutoStartupToggled(enabled) => {
                self.config.set_auto_startup(enabled);
                if let Err(e) = startup::set_auto_startup(enabled) {
                    eprintln!("设置开机启动失败: {}", e);
                }
            }
        }
    }
}
