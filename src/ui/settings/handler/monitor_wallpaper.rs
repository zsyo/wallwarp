// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 多显示器独立壁纸（显示器列表加载与按显示器设置壁纸）

use crate::platform::MonitorInfo;
use crate::services::async_task;
use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{error, info};

impl App {
    /// 加载显示器列表
    pub(in crate::ui::settings) fn settings_load_monitors(&mut self) -> Task<AppMessage> {
        async_task::enumerate_monitors_task(self.main_window_id)
            .map(|result| SettingsMessage::MonitorsLoaded(result).into())
    }

    /// 显示器列表加载完成
    pub(in crate::ui::settings) fn settings_monitors_loaded(
        &mut self,
        result: Result<Vec<MonitorInfo>, String>,
    ) -> Task<AppMessage> {
        self.settings_state.monitors_loaded = true;
        match result {
            Ok(monitors) => self.settings_state.monitors = monitors,
            Err(e) => error!("[设置] [多显示器] 获取显示器列表失败: {}", e),
        }
        Task::none()
    }

    /// 为指定显示器打开图片选择对话框
    pub(in crate::ui::settings) fn settings_select_monitor_image(
        &mut self,
        monitor_id: String,
    ) -> Task<AppMessage> {
        Task::perform(async_task::select_image_async(), move |path| {
            SettingsMessage::MonitorImageSelected(monitor_id.clone(), path).into()
        })
    }

    /// 图片已选择：为指定显示器设置壁纸（路径为空串表示用户取消）
    pub(in crate::ui::settings) fn settings_monitor_image_selected(
        &mut self,
        monitor_id: String,
        path: String,
    ) -> Task<AppMessage> {
        if path.is_empty() {
            return Task::none();
        }
        let mode = self.config.wallpaper.mode;
        async_task::set_wallpaper_for_monitor_task(
            self.main_window_id,
            monitor_id.clone(),
            path,
            mode,
        )
        .map(move |result| SettingsMessage::MonitorWallpaperSet(monitor_id.clone(), result).into())
    }

    /// 显示器壁纸设置完成：通知设置结果
    pub(in crate::ui::settings) fn settings_monitor_wallpaper_set(
        &mut self,
        monitor_id: String,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        let monitor_name = self
            .settings_state
            .monitors
            .iter()
            .find(|m| m.id == monitor_id)
            .map(|m| m.name.clone())
            .unwrap_or_else(|| monitor_id.clone());

        match result {
            Ok(()) => {
                info!("[设置] [多显示器] [{}] 壁纸设置成功", monitor_id);
                let message = self.i18n.t_with_args(
                    "settings.monitor-wallpaper-set",
                    &[("monitor", monitor_name)],
                );
                self.show_notification(message, NotificationType::Success)
            }
            Err(e) => {
                error!("[设置] [多显示器] [{}] 壁纸设置失败: {}", monitor_id, e);
                let message = self.i18n.t_with_args(
                    "settings.monitor-wallpaper-set-failed",
                    &[("monitor", monitor_name), ("error", e)],
                );
                self.show_notification(message, NotificationType::Error)
            }
        }
    }
}
