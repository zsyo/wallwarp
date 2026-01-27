// Copyright (C) 2026 zsyo - GNU AGPL v3.0

/// 定时切换壁纸相关消息
#[derive(Debug, Clone)]
pub enum AutoChangeMessage {
    /// 启动定时切换
    StartAutoChange,
    /// 停止定时切换
    StopAutoChange,
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