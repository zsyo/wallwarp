// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{ActivePage, App, AppMessage, CloseConfirmationAction, NotificationType};
use iced::Task;

/// 主界面页面消息
#[derive(Debug, Clone)]
pub enum MainMessage {
    /// 窗口大小改变事件
    WindowResized(u32, u32),
    /// 执行延迟保存事件
    ExecutePendingSave,
    /// 页面选择事件
    PageSelected(ActivePage),
    /// 窗口关闭请求事件
    WindowCloseRequested,
    /// 窗口聚焦事件
    WindowFocused,
    /// 窗口最小化到托盘事件
    MinimizeToTray,
    /// 托盘图标点击事件
    TrayIconClicked,
    /// 托盘菜单事件
    TrayMenuEvent(String),
    /// 滚动到顶部，参数为滚动组件ID
    ScrollToTop(String),
    /// 主题选择
    ThemeSelected(crate::utils::config::Theme),
    /// 自动检测颜色模式
    AutoDetectColorModeTick,
    /// 显示关闭确认弹窗
    ShowCloseConfirmation,
    /// 关闭确认弹窗事件(动作, 是否记住设置)
    CloseConfirmationResponse(CloseConfirmationAction, bool),
    /// 关闭确认弹窗取消事件
    CloseConfirmationCancelled,
    /// 关闭确认弹窗记住设置开关事件
    ToggleRememberSetting(bool),
    /// 显示通知，参数：消息内容，通知类型
    ShowNotification(String, NotificationType),
    /// 隐藏通知（用于定时隐藏）
    HideNotification,
    /// 隐藏通知（带版本号，用于防止旧版本的隐藏任务关闭新显示的通知）
    HideNotificationWithVersion(u64),
    /// 托盘切换上一张壁纸事件
    TraySwitchPreviousWallpaper,
    /// 托盘切换下一张壁纸事件
    TraySwitchNextWallpaper,
    /// 托盘保存当前壁纸到库事件
    TraySaveCurrentWallpaper,
    /// 添加壁纸到历史记录
    AddToWallpaperHistory(String),
    /// 从历史记录末尾移除壁纸
    RemoveLastFromWallpaperHistory,
    /// 外部实例触发事件
    ExternalInstanceTriggered(String),
    /// 拖拽自定义标题栏事件
    TitleBarDrag,
    /// 自定义标题栏最小化窗口按钮事件
    TitleBarMinimize,
    /// 自定义标题栏最大化/还原窗口按钮事件
    TitleBarMaximize,
    /// 自定义标题栏关闭窗口按钮事件
    TitleBarClose,
    /// 从最大化还原窗口后恢复边框调整大小状态
    RestoreBorderResize(bool),
    /// 调整窗口大小（包含所有方向）
    ResizeWindow(iced::window::Direction),
}

impl From<MainMessage> for AppMessage {
    fn from(msg: MainMessage) -> AppMessage {
        AppMessage::Main(msg)
    }
}

impl App {
    /// 处理本地壁纸相关消息
    pub fn handle_main_message(&mut self, msg: MainMessage) -> Task<AppMessage> {
        match msg {
            MainMessage::PageSelected(page) => self.page_selected(page),
            MainMessage::WindowResized(width, height) => self.window_resized(width, height),
            MainMessage::ExecutePendingSave => self.execute_pending_save(),
            MainMessage::WindowCloseRequested => self.window_close_requested(),
            MainMessage::WindowFocused => self.window_focused(),
            MainMessage::MinimizeToTray => self.minimize_to_tray(),
            MainMessage::TrayIconClicked => self.show_window(),
            MainMessage::TrayMenuEvent(id) => self.tray_menu_event(id),
            MainMessage::ScrollToTop(scrollable_id) => self.scroll_to_top(scrollable_id),
            MainMessage::ThemeSelected(theme) => self.theme_selected(theme),
            MainMessage::AutoDetectColorModeTick => self.detect_color_mode(),
            MainMessage::ShowCloseConfirmation => self.show_close_confirm(),
            MainMessage::CloseConfirmationResponse(action, remember_setting) => {
                self.close_confirm_response(action, remember_setting)
            }
            MainMessage::CloseConfirmationCancelled => self.close_confirm_cancelled(),
            MainMessage::ToggleRememberSetting(checked) => self.toggle_remember_setting(checked),
            MainMessage::ShowNotification(message, notification_type) => {
                self.show_notification(message, notification_type)
            }
            MainMessage::HideNotification => self.hide_notification(),
            MainMessage::HideNotificationWithVersion(version) => self.hide_notification_with_version(version),
            MainMessage::TraySwitchPreviousWallpaper => self.tray_switch_previous_wallpaper(),
            MainMessage::TraySwitchNextWallpaper => self.tray_switch_next_wallpaper(),
            MainMessage::TraySaveCurrentWallpaper => self.tray_save_current_wallpaper(),
            MainMessage::AddToWallpaperHistory(path) => self.add_to_wallpaper_history(path),
            MainMessage::RemoveLastFromWallpaperHistory => self.remove_last_from_wallpaper_history(),
            MainMessage::ExternalInstanceTriggered(payload) => self.external_instance_triggered(payload),
            MainMessage::TitleBarDrag => self.title_bar_drag(),
            MainMessage::TitleBarMinimize => self.title_bar_minimize(),
            MainMessage::TitleBarMaximize => self.title_bar_maximize(),
            MainMessage::TitleBarClose => self.window_close_requested(),
            MainMessage::RestoreBorderResize(window_state) => self.restore_border_resize(window_state),
            MainMessage::ResizeWindow(direction) => self.drag_resize_window(direction),
        }
    }
}
