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
            "tray_save_current" => {
                // 保存当前壁纸到库
                return Task::done(MainMessage::TraySaveCurrentWallpaper.into());
            }
            "tray_settings" => {
                // 打开设置窗口
                self.active_page = ActivePage::Settings;
                return self.show_window();
            }
            "tray_quit" => {
                // 真正退出程序（先关闭悬浮球窗口避免残影）
                return self.quit_program();
            }
            // ===== 悬浮球菜单（动作与托盘菜单同源，末项为关闭悬浮球） =====
            "ball_show" => {
                return self.show_window();
            }
            "ball_switch_previous" => {
                return Task::done(MainMessage::TraySwitchPreviousWallpaper.into());
            }
            "ball_switch_next" => {
                return Task::done(MainMessage::TraySwitchNextWallpaper.into());
            }
            "ball_save_current" => {
                return Task::done(MainMessage::TraySaveCurrentWallpaper.into());
            }
            "ball_settings" => {
                self.active_page = ActivePage::Settings;
                return self.show_window();
            }
            "ball_close" => {
                return self.floating_ball_close();
            }
            _ => {}
        }

        Task::none()
    }
}
