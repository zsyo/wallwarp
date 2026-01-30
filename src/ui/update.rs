// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, AppMessage};
use crate::ui::main::MainMessage;
use crate::ui::online::OnlineMessage;
use iced::Task;

impl App {
    /// 主更新方法 - 处理所有应用消息
    pub fn update(&mut self, msg: AppMessage) -> Task<AppMessage> {
        // 检查是否需要加载初始任务（只在第一次运行时）
        if !self.initial_loaded {
            self.initial_loaded = true;
            // 如果默认页面是在线壁纸，则加载初始数据
            if self.active_page == super::ActivePage::OnlineWallpapers {
                return Task::batch(vec![
                    Task::done(OnlineMessage::LoadWallpapers.into()),
                    Task::done(MainMessage::ScrollToTop("online_wallpapers_scroll".to_string()).into()),
                ]);
            }
        }

        match msg {
            AppMessage::None => Task::none(),
            AppMessage::Main(message) => self.handle_main_message(message),
            AppMessage::Local(message) => self.handle_local_message(message),
            AppMessage::Online(message) => self.handle_online_message(message),
            AppMessage::Download(message) => self.handle_download_message(message),
            AppMessage::AutoChange(message) => self.handle_auto_change_message(message),
            AppMessage::Settings(message) => self.handle_settings_message(message),
        }
    }
}
