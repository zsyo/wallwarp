// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::main) fn add_to_wallpaper_history(&mut self, path: String) -> Task<AppMessage> {
        // 检查历史记录中是否已存在该路径，如果存在则先移除
        if let Some(pos) = self.wallpaper_history.iter().position(|p| p == &path) {
            self.wallpaper_history.remove(pos);
        }

        // 记录路径用于日志输出
        let path_for_log = path.clone();

        // 添加到历史记录末尾
        self.wallpaper_history.push(path);

        // 限制历史记录最多50条
        if self.wallpaper_history.len() > 50 {
            self.wallpaper_history.remove(0);
        }

        info!(
            "[壁纸历史] 添加记录: {}, 当前记录数: {}",
            path_for_log,
            self.wallpaper_history.len()
        );

        // 如果开启了定时切换壁纸,那么重新计算下次切换时间
        if self.auto_change_state.auto_change_enabled {
            if let Some(minutes) = self.config.wallpaper.auto_change_interval.get_minutes() {
                if minutes > 0 {
                    self.auto_change_state.next_execute_time =
                        Some(chrono::Local::now() + chrono::Duration::minutes(minutes as i64));
                }
            }
        }

        // 更新托盘菜单项的启用状态
        self.tray_manager
            .update_switch_previous_item(self.wallpaper_history.len());

        Task::none()
    }

    pub(in crate::ui::main) fn remove_last_from_wallpaper_history(&mut self) -> Task<AppMessage> {
        // 从历史记录末尾移除壁纸
        if let Some(removed) = self.wallpaper_history.pop() {
            info!(
                "[壁纸历史] 移除记录: {}, 当前记录数: {}",
                removed,
                self.wallpaper_history.len()
            );
        }

        // 如果开启了定时切换壁纸,那么重新计算下次切换时间
        if self.auto_change_state.auto_change_enabled {
            if let Some(minutes) = self.config.wallpaper.auto_change_interval.get_minutes() {
                if minutes > 0 {
                    self.auto_change_state.next_execute_time =
                        Some(chrono::Local::now() + chrono::Duration::minutes(minutes as i64));
                }
            }
        }

        // 更新托盘菜单项的启用状态
        self.tray_manager
            .update_switch_previous_item(self.wallpaper_history.len());

        Task::none()
    }
}
