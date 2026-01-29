// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{ActivePage, App, AppMessage, CloseConfirmationAction, NotificationType};
use iced::Task;

/// 主界面页面消息
#[derive(Debug, Clone)]
pub enum MainMessage {
    WindowResized(u32, u32), // 窗口大小改变事件
    ExecutePendingSave,
    PageSelected(ActivePage),
    WindowCloseRequested,
    WindowFocused,
    MinimizeToTray,
    TrayIconClicked,
    TrayMenuEvent(String),
    ScrollToTop(String),                        // 滚动到顶部，参数为滚动组件ID
    ThemeSelected(crate::utils::config::Theme), // 主题选择
    AutoDetectColorModeTick,                    // 自动检测颜色模式
    ShowCloseConfirmation,
    CloseConfirmationResponse(CloseConfirmationAction, bool), // (动作, 是否记住设置)
    CloseConfirmationCancelled,
    ToggleRememberSetting(bool),
    ShowNotification(String, NotificationType), // 显示通知，参数：消息内容，通知类型
    HideNotification,                           // 隐藏通知（用于定时隐藏）
    TraySwitchPreviousWallpaper,
    TraySwitchNextWallpaper,
    AddToWallpaperHistory(String),  // 添加壁纸到历史记录
    RemoveLastFromWallpaperHistory, // 从历史记录末尾移除壁纸
    ExternalInstanceTriggered(String),
    TitleBarDrag,                          // 拖拽标题栏
    TitleBarMinimize,                      // 最小化窗口
    TitleBarMaximize,                      // 最大化/还原窗口
    TitleBarClose,                         // 关闭窗口
    ResizeWindow(iced::window::Direction), // 调整窗口大小（包含所有方向）
}

impl From<MainMessage> for AppMessage {
    fn from(main_message: MainMessage) -> AppMessage {
        AppMessage::Main(main_message)
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
            MainMessage::TraySwitchPreviousWallpaper => self.tray_switch_previous_wallpaper(),
            MainMessage::TraySwitchNextWallpaper => self.tray_switch_next_wallpaper(),
            MainMessage::AddToWallpaperHistory(path) => self.add_to_wallpaper_history(path),
            MainMessage::RemoveLastFromWallpaperHistory => self.remove_last_from_wallpaper_history(),
            MainMessage::ExternalInstanceTriggered(payload) => self.external_instance_triggered(payload),
            MainMessage::TitleBarDrag => self.title_bar_drag(),
            MainMessage::TitleBarMinimize => self.title_bar_minimize(),
            MainMessage::TitleBarMaximize => self.title_bar_maximize(),
            MainMessage::TitleBarClose => self.window_close_requested(),
            MainMessage::ResizeWindow(direction) => self.drag_resize_window(direction),
        }
    }
}
