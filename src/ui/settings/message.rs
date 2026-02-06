// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::{Sorting, TimeRange};
use crate::ui::{App, AppMessage};
use crate::utils::config::{CloseAction, WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};
use iced::Task;

/// 主界面页面消息
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// 语言选择
    LanguageSelected(String),
    /// 开机自启状态更改
    AutoStartupToggled(bool),
    /// 日志状态更改
    LoggingToggled(bool),
    /// 窗口关闭按钮行为选择
    CloseActionSelected(CloseAction),
    /// 打开链接
    OpenUrl(String),
    /// 数据路径选择
    DataPathSelected(String),
    /// 缓存路径选择
    CachePathSelected(String),
    /// 打开路径
    OpenPath(String),
    /// 打开日志目录
    OpenLogsPath,
    /// 显示路径清空确认对话框，参数为路径类型 ("data" 或 "cache")
    ShowPathClearConfirmation(String),
    /// 确认清空路径，参数为路径类型
    ConfirmPathClear(String),
    /// 取消清空路径
    CancelPathClear,
    /// 恢复默认路径，参数为路径类型
    RestoreDefaultPath(String),
    /// 壁纸API密钥更改
    WallhavenApiKeyChanged(String),
    /// 保存壁纸API密钥
    SaveWallhavenApiKey,
    /// 代理协议更改
    ProxyProtocolChanged(String),
    /// 代理地址更改
    ProxyAddressChanged(String),
    /// 代理端口更改
    ProxyPortChanged(u32),
    /// 代理开关切换
    ProxyToggled(bool),
    /// 保存代理设置
    SaveProxy,
    /// 壁纸模式选择
    WallpaperModeSelected(WallpaperMode),
    /// 定时切换模式选择
    AutoChangeModeSelected(WallpaperAutoChangeMode),
    /// 定时切换周期选择
    AutoChangeIntervalSelected(WallpaperAutoChangeInterval),
    /// 自定义切换周期分钟数变化
    CustomIntervalMinutesChanged(u32),
    /// 定时切换关键词变化
    AutoChangeQueryChanged(String),
    /// 保存定时切换关键词
    SaveAutoChangeQuery,
    /// 定时切换排序方式变化
    AutoChangeSortingChanged(Sorting),
    /// 保存定时切换排序方式
    SaveAutoChangeSorting,
    /// 定时切换时间范围变化
    AutoChangeTimeRangeChanged(TimeRange),
    /// 保存定时切换时间范围
    SaveAutoChangeTimeRange,
    /// 展开语言选择器
    LanguagePickerExpanded,
    /// 关闭语言选择器
    LanguagePickerDismiss,
    /// 展开代理协议选择器
    ProxyProtocolPickerExpanded,
    /// 关闭代理协议选择器
    ProxyProtocolPickerDismiss,
    /// 展开主题选择器
    ThemePickerExpanded,
    /// 关闭主题选择器
    ThemePickerDismiss,
    /// 展开排序方式选择器
    SortingPickerExpanded,
    /// 关闭排序方式选择器
    SortingPickerDismiss,
    /// 展开时间范围选择器
    TimeRangePickerExpanded,
    /// 关闭时间范围选择器
    TimeRangePickerDismiss,
}

impl From<SettingsMessage> for AppMessage {
    fn from(msg: SettingsMessage) -> AppMessage {
        AppMessage::Settings(msg)
    }
}

impl App {
    /// 处理本地壁纸相关消息
    pub fn handle_settings_message(&mut self, msg: SettingsMessage) -> Task<AppMessage> {
        match msg {
            SettingsMessage::LanguageSelected(lang) => self.settings_language_selected(lang),
            SettingsMessage::AutoStartupToggled(enabled) => self.settings_auto_startup_toggled(enabled),
            SettingsMessage::LoggingToggled(enabled) => self.settings_logging_toggled(enabled),
            SettingsMessage::CloseActionSelected(action) => self.settings_close_action_selected(action),
            SettingsMessage::OpenUrl(url) => self.settings_open_url(url),
            SettingsMessage::DataPathSelected(path) => self.settings_data_path_selected(path),
            SettingsMessage::CachePathSelected(path) => self.settings_cache_path_selected(path),
            SettingsMessage::OpenPath(path_type) => self.settings_open_path(path_type),
            SettingsMessage::OpenLogsPath => self.settings_open_logs_path(),
            SettingsMessage::ShowPathClearConfirmation(path_type) => self.settings_show_path_clear_confirm(path_type),
            SettingsMessage::ConfirmPathClear(path_type) => self.settings_confirm_path_clear(path_type),
            SettingsMessage::CancelPathClear => self.settings_cancel_path_clear(),
            SettingsMessage::RestoreDefaultPath(path_type) => self.settings_restore_default_path(path_type),
            SettingsMessage::WallhavenApiKeyChanged(api_key) => self.settings_wallhaven_api_key_changed(api_key),
            SettingsMessage::SaveWallhavenApiKey => self.settings_save_wallhaven_api_key(),
            SettingsMessage::ProxyProtocolChanged(protocol) => self.settings_proxy_protocol_changed(protocol),
            SettingsMessage::ProxyAddressChanged(address) => self.settings_proxy_address_changed(address),
            SettingsMessage::ProxyPortChanged(port) => self.settings_proxy_port_changed(port),
            SettingsMessage::ProxyToggled(enabled) => self.settings_proxy_toggled(enabled),
            SettingsMessage::SaveProxy => self.settings_save_proxy(),
            SettingsMessage::WallpaperModeSelected(mode) => self.settings_wallpaper_mode_selected(mode),
            SettingsMessage::AutoChangeModeSelected(mode) => self.settings_auto_change_mode_selected(mode),
            SettingsMessage::AutoChangeIntervalSelected(interval) => {
                self.settings_auto_change_interval_selected(interval)
            }
            SettingsMessage::CustomIntervalMinutesChanged(minutes) => {
                self.settings_custom_interval_minutes_changed(minutes)
            }
            SettingsMessage::AutoChangeQueryChanged(query) => self.settings_auto_change_query_changed(query),
            SettingsMessage::SaveAutoChangeQuery => self.settings_save_auto_change_query(),
            SettingsMessage::AutoChangeSortingChanged(sorting) => self.settings_auto_change_sorting_changed(sorting),
            SettingsMessage::SaveAutoChangeSorting => self.settings_save_auto_change_sorting(),
            SettingsMessage::AutoChangeTimeRangeChanged(time_range) => self.settings_auto_change_time_range_changed(time_range),
            SettingsMessage::SaveAutoChangeTimeRange => self.settings_save_auto_change_time_range(),
            SettingsMessage::LanguagePickerExpanded => self.settings_language_picker_expanded(),
            SettingsMessage::LanguagePickerDismiss => self.settings_language_picker_dismiss(),
            SettingsMessage::ProxyProtocolPickerExpanded => self.settings_proxy_protocol_picker_expanded(),
            SettingsMessage::ProxyProtocolPickerDismiss => self.settings_proxy_protocol_picker_dismiss(),
            SettingsMessage::ThemePickerExpanded => self.settings_theme_picker_expanded(),
            SettingsMessage::ThemePickerDismiss => self.settings_theme_picker_dismiss(),
            SettingsMessage::SortingPickerExpanded => self.settings_sorting_picker_expanded(),
            SettingsMessage::SortingPickerDismiss => self.settings_sorting_picker_dismiss(),
            SettingsMessage::TimeRangePickerExpanded => self.settings_time_range_picker_expanded(),
            SettingsMessage::TimeRangePickerDismiss => self.settings_time_range_picker_dismiss(),
        }
    }
}
