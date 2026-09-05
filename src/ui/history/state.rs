// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::local::Wallpaper;
use iced::widget::image::Handle;

/// 一条壁纸历史记录
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// 壁纸文件路径
    pub path: String,
    /// 最近应用的 Unix 时间戳（秒）
    pub applied_at: i64,
    /// 文件是否已位于正式壁纸目录（false 时行内显示"保存到壁纸库"）
    pub in_library: bool,
}

/// 壁纸历史页面状态
#[derive(Debug, Default)]
pub struct HistoryState {
    /// 是否已在本会话加载过（壁纸变化后会被置回 false 以便重载）
    pub loaded: bool,
    /// 历史条目（新→旧排序，已过滤磁盘上不存在的文件）
    pub entries: Vec<HistoryEntry>,
    /// 缩略图句柄（与 entries 索引对应；None = 加载中）
    pub thumbs: Vec<Option<Handle>>,
    /// 缩略图加载后保留的壁纸元数据（分辨率/文件大小，供列表与预览信息）
    pub wallpapers: Vec<Option<Wallpaper>>,
    /// 预览模态可见性
    pub modal_visible: bool,
    /// 预览的条目索引
    pub modal_index: usize,
    /// 预览原图句柄（未解码完成时为 None）
    pub modal_handle: Option<Handle>,
    /// 待确认移除的条目索引（Some = 显示确认框）
    pub remove_target: Option<usize>,
    /// 清空确认框可见性
    pub clear_confirm_visible: bool,
}

impl HistoryState {
    /// 复位（数据源变化后重载）
    pub fn invalidate(&mut self) {
        self.loaded = false;
        self.entries.clear();
        self.thumbs.clear();
        self.wallpapers.clear();
        self.close_modal();
        self.remove_target = None;
        self.clear_confirm_visible = false;
    }

    /// 关闭预览模态并释放原图句柄
    pub fn close_modal(&mut self) {
        self.modal_visible = false;
        self.modal_index = 0;
        self.modal_handle = None;
    }
}
