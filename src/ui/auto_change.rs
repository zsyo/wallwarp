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

/// 定时切换壁纸相关状态
#[derive(Debug)]
pub struct AutoChangeState {
    /// 是否启用定时切换
    pub auto_change_enabled: bool,
    /// 定时切换计时器
    pub auto_change_timer: Option<std::time::Instant>,
    /// 上次执行时间
    pub auto_change_last_time: Option<std::time::Instant>,
    /// 是否自动检测颜色模式
    pub auto_detect_color_mode: bool,
}

impl Default for AutoChangeState {
    fn default() -> Self {
        Self {
            auto_change_enabled: false,
            auto_change_timer: None,
            auto_change_last_time: None,
            auto_detect_color_mode: false,
        }
    }
}