// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{ActivePage, App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn tray_menu_event(&mut self, id: String) -> Task<AppMessage> {
        match id.as_str() {
            "tray_show" => {
                // 显示窗口并检测状态，如果最小化或不在前台则置顶
                return self.show_window();
            }
            "tray_switch_previous" => {
                // 切换上一张壁纸
                return Task::done(MainMessage::TraySwitchPreviousWallpaper.into());
            }
            "tray_switch_next" => {
                // 切换下一张壁纸
                return Task::done(MainMessage::TraySwitchNextWallpaper.into());
            }
            "tray_settings" => {
                // 打开设置窗口
                self.active_page = ActivePage::Settings;
                return self.show_window();
            }
            "tray_quit" => {
                // 真正退出程序
                return iced::exit();
            }
            _ => {}
        }

        Task::none()
    }
}
