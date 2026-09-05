// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::main::menu_defs::{MenuAction, menu_action_from_id};
use crate::ui::{ActivePage, App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn tray_menu_event(&mut self, id: String) -> Task<AppMessage> {
        // 托盘/悬浮球各自的专属末项
        match id.as_str() {
            "tray_quit" => {
                // 真正退出程序（先关闭悬浮球窗口避免残影）
                return self.quit_program();
            }
            "ball_close" => {
                return self.floating_ball_close();
            }
            _ => {}
        }

        // 公共动作项（托盘与悬浮球同源）
        match menu_action_from_id(&id) {
            Some(MenuAction::ShowWindow) => {
                // 显示窗口并检测状态，如果最小化或不在前台则置顶
                self.show_window()
            }
            Some(MenuAction::SwitchPrevious) => {
                // 切换上一张壁纸
                Task::done(MainMessage::TraySwitchPreviousWallpaper.into())
            }
            Some(MenuAction::SwitchNext) => {
                // 切换下一张壁纸
                Task::done(MainMessage::TraySwitchNextWallpaper.into())
            }
            Some(MenuAction::SaveCurrent) => {
                // 保存当前壁纸到库
                Task::done(MainMessage::TraySaveCurrentWallpaper.into())
            }
            Some(MenuAction::Settings) => {
                // 打开设置窗口
                self.active_page = ActivePage::Settings;
                self.show_window()
            }
            None => Task::none(),
        }
    }
}
