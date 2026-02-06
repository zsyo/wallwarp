// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::{Sorting, TimeRange};
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_wallpaper_mode_selected(
        &mut self,
        mode: WallpaperMode,
    ) -> Task<AppMessage> {
        let old_mode = self.config.wallpaper.mode;
        info!("[设置] [壁纸模式] 修改: {:?} -> {:?}", old_mode, mode);
        self.settings_state.wallpaper_mode = mode;
        self.config.set_wallpaper_mode(mode);
        iced::Task::none()
    }

    pub(in crate::ui::settings) fn settings_auto_change_mode_selected(
        &mut self,
        mode: WallpaperAutoChangeMode,
    ) -> Task<AppMessage> {
        let old_mode = self.config.wallpaper.auto_change_mode;
        info!("[设置] [定时切换模式] 修改: {:?} -> {:?}", old_mode, mode);
        self.settings_state.auto_change_mode = mode;
        self.config.wallpaper.auto_change_mode = mode;
        self.config.save_to_file();
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_auto_change_interval_selected(
        &mut self,
        interval: WallpaperAutoChangeInterval,
    ) -> Task<AppMessage> {
        let old_interval = self.config.wallpaper.auto_change_interval.clone();
        info!("[设置] [定时切换周期] 修改: {:?} -> {:?}", old_interval, interval);
        self.settings_state.auto_change_interval = interval.clone();
        self.config.wallpaper.auto_change_interval = interval;

        // 根据选择的间隔启动或停止定时任务
        if matches!(
            self.settings_state.auto_change_interval,
            WallpaperAutoChangeInterval::Off
        ) {
            // 选择关闭，停止定时任务
            self.auto_change_state.auto_change_enabled = false;
            self.auto_change_state.next_execute_time = None;
            info!("[定时切换] [停止] 定时任务已停止");
        } else {
            // 选择其他选项，启动定时任务
            self.auto_change_state.auto_change_enabled = true;

            // 计算并记录下次执行时间
            if let Some(minutes) = self.settings_state.auto_change_interval.get_minutes() {
                let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                info!(
                    "[定时切换] [启动] 间隔: {}分钟, 下次执行时间: {}",
                    minutes,
                    next_time.format("%Y-%m-%d %H:%M:%S")
                );
                self.auto_change_state.next_execute_time = Some(next_time);
            }
        }

        self.config.save_to_file();
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_custom_interval_minutes_changed(
        &mut self,
        minutes: u32,
    ) -> Task<AppMessage> {
        // 限制最小值为1
        let minutes = if minutes < 1 { 1 } else { minutes };
        self.settings_state.custom_interval_minutes = minutes;

        // 如果当前选中的是自定义选项，立即更新配置
        if matches!(
            self.settings_state.auto_change_interval,
            WallpaperAutoChangeInterval::Custom(_)
        ) {
            // 同时更新 UI 状态和配置文件
            self.settings_state.auto_change_interval = WallpaperAutoChangeInterval::Custom(minutes);
            self.config.wallpaper.auto_change_interval = WallpaperAutoChangeInterval::Custom(minutes);
            self.config.save_to_file();

            // 重置定时任务并记录下次执行时间
            if self.auto_change_state.auto_change_enabled {
                let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                info!(
                    "[定时切换] [重置] 自定义间隔: {}分钟, 下次执行时间: {}",
                    minutes,
                    next_time.format("%Y-%m-%d %H:%M:%S")
                );
                self.auto_change_state.next_execute_time = Some(next_time);
            }
        }
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_auto_change_query_changed(&mut self, query: String) -> Task<AppMessage> {
        // 只更新临时状态，不保存到配置文件
        self.settings_state.auto_change_query = query;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_auto_change_query(&mut self) -> Task<AppMessage> {
        // 保存到配置文件
        let old_query = self.config.wallpaper.auto_change_query.clone();
        let new_query = self.settings_state.auto_change_query.clone();
        info!(
            "[设置] [定时切换关键词] 保存: {} -> {}",
            if old_query.is_empty() { "(空)" } else { &old_query },
            if new_query.is_empty() { "(空)" } else { &new_query }
        );
        self.config.wallpaper.auto_change_query = new_query;

        self.config.save_to_file();

        // 显示保存成功通知
        let success_message = self.i18n.t("settings.save-success").to_string();
        self.show_notification(success_message, NotificationType::Success)
    }

    pub(in crate::ui::settings) fn settings_auto_change_sorting_changed(
        &mut self,
        sorting: Sorting,
    ) -> Task<AppMessage> {
        let old_sorting = self.settings_state.auto_change_sorting;
        info!(
            "[设置] [定时切换排序方式] 修改: {:?} -> {:?}",
            old_sorting, sorting
        );
        self.settings_state.auto_change_sorting = sorting;

        // 立即保存到配置文件
        self.config.wallpaper.auto_change_sorting = sorting.to_string();
        self.config.save_to_file();

        // 选择后关闭选择器
        self.settings_state.sorting_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_auto_change_sorting(&mut self) -> Task<AppMessage> {
        // 此方法已不再使用，保留以避免编译错误
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_auto_change_time_range_changed(
        &mut self,
        time_range: TimeRange,
    ) -> Task<AppMessage> {
        let old_time_range = self.settings_state.auto_change_time_range;
        info!(
            "[设置] [定时切换时间范围] 修改: {:?} -> {:?}",
            old_time_range, time_range
        );
        self.settings_state.auto_change_time_range = time_range;

        // 立即保存到配置文件
        self.config.wallpaper.auto_change_top_range = time_range.value().to_string();
        self.config.save_to_file();

        // 选择后关闭选择器
        self.settings_state.time_range_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_auto_change_time_range(&mut self) -> Task<AppMessage> {
        // 此方法已不再使用，保留以避免编译错误
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_sorting_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换排序方式选择器的展开/收起状态
        self.settings_state.sorting_picker_expanded = !self.settings_state.sorting_picker_expanded;
        Task::none()
    }
    pub(in crate::ui::settings) fn settings_sorting_picker_dismiss(&mut self) -> Task<AppMessage> {
        self.settings_state.sorting_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_time_range_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换时间范围选择器的展开/收起状态
        self.settings_state.time_range_picker_expanded = !self.settings_state.time_range_picker_expanded;
        Task::none()
    }
    pub(in crate::ui::settings) fn settings_time_range_picker_dismiss(&mut self) -> Task<AppMessage> {
        self.settings_state.time_range_picker_expanded = false;
        Task::none()
    }
}
