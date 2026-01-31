// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};

/// 定时切换壁纸相关消息
#[derive(Debug, Clone)]
pub enum AutoChangeMessage {
    /// 定时切换定时器事件
    AutoChangeTick,
    /// 获取支持的图片文件列表成功
    GetSupportedImagesSuccess(Vec<String>),
    /// 获取支持的图片文件列表失败
    GetSupportedImagesFailed(String),
    /// 随机设置壁纸成功
    SetRandomWallpaperSuccess(String),
    /// 随机设置壁纸失败
    SetRandomWallpaperFailed(String),
}

impl From<AutoChangeMessage> for AppMessage {
    fn from(msg: AutoChangeMessage) -> AppMessage {
        AppMessage::AutoChange(msg)
    }
}

impl App {
    /// 处理定时切换壁纸相关消息
    pub fn handle_auto_change_message(&mut self, msg: AutoChangeMessage) -> iced::Task<AppMessage> {
        match msg {
            AutoChangeMessage::AutoChangeTick => self.auto_change_tick(),
            AutoChangeMessage::GetSupportedImagesSuccess(paths) => self.get_supported_images_success(paths),
            AutoChangeMessage::GetSupportedImagesFailed(error) => self.get_supported_images_failed(error),
            AutoChangeMessage::SetRandomWallpaperSuccess(path) => self.set_random_wallpaper_success(path),
            AutoChangeMessage::SetRandomWallpaperFailed(error) => self.set_random_wallpaper_failed(error),
        }
    }
}
