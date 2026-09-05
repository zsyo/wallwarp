// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸历史页面消息

use crate::services::local::Wallpaper;
use crate::ui::history::state::HistoryEntry;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

/// 壁纸历史页面消息
#[derive(Debug, Clone)]
pub enum HistoryMessage {
    /// 从数据库加载历史并生成缩略图任务
    Load,
    /// 历史条目加载完成（新→旧）
    Loaded(Vec<HistoryEntry>),
    /// 缩略图加载完成（None 表示文件已失效，仅显示占位）
    ThumbLoaded {
        index: usize,
        wallpaper: Option<Wallpaper>,
    },
    /// 重新应用该历史壁纸
    ApplyEntry(usize),
    /// 将该条目文件从缓存目录保存到正式壁纸目录
    SaveToLibrary(usize),
    /// 保存到壁纸目录完成（Ok 为库内新路径）
    SaveFinished {
        index: usize,
        result: Result<String, String>,
    },
    /// 预览该历史壁纸（打开模态并加载原图）
    PreviewEntry(usize),
    /// 预览上一张
    PreviousImage,
    /// 预览下一张
    NextImage,
    /// 预览原图加载完成
    ModalImageLoaded(Handle),
    /// 关闭预览
    CloseModal,
    /// 在文件夹中查看该壁纸
    OpenLocation(usize),
    /// 复制文件路径
    CopyPath(usize),
    /// 请求移除记录（显示确认框）
    RemoveEntry(usize),
    /// 确认移除
    RemoveConfirmed,
    /// 移除完成
    RemoveFinished {
        index: usize,
        result: Result<(), String>,
    },
    /// 取消移除
    RemoveCanceled,
    /// 请求清空历史（显示确认框）
    ClearRequested,
    /// 确认清空
    ClearConfirmed,
    /// 清空完成
    ClearFinished(Result<(), String>),
    /// 取消清空
    ClearCanceled,
    /// 刷新
    Refresh,
}

impl From<HistoryMessage> for AppMessage {
    fn from(msg: HistoryMessage) -> AppMessage {
        AppMessage::History(msg)
    }
}

impl App {
    /// 处理壁纸历史相关消息
    pub fn handle_history_message(&mut self, msg: HistoryMessage) -> Task<AppMessage> {
        match msg {
            HistoryMessage::Load => self.load_history_entries(),
            HistoryMessage::Loaded(entries) => self.history_entries_loaded(entries),
            HistoryMessage::ThumbLoaded { index, wallpaper } => {
                self.history_thumb_loaded(index, wallpaper)
            }
            HistoryMessage::ApplyEntry(index) => self.apply_history_entry(index),
            HistoryMessage::SaveToLibrary(index) => self.save_history_entry_to_library(index),
            HistoryMessage::SaveFinished { index, result } => {
                self.history_entry_saved(index, result)
            }
            HistoryMessage::PreviewEntry(index) => self.preview_history_entry(index),
            HistoryMessage::PreviousImage => self.previous_history_image(),
            HistoryMessage::NextImage => self.next_history_image(),
            HistoryMessage::ModalImageLoaded(handle) => self.history_modal_image_loaded(handle),
            HistoryMessage::CloseModal => {
                self.history_state.close_modal();
                Task::none()
            }
            HistoryMessage::OpenLocation(index) => self.view_history_file(index),
            HistoryMessage::CopyPath(index) => self.copy_history_path(index),
            HistoryMessage::RemoveEntry(index) => {
                self.history_state.remove_target = Some(index);
                Task::none()
            }
            HistoryMessage::RemoveConfirmed => self.remove_history_entry(),
            HistoryMessage::RemoveFinished { index, result } => {
                self.history_entry_removed(index, result)
            }
            HistoryMessage::RemoveCanceled => {
                self.history_state.remove_target = None;
                Task::none()
            }
            HistoryMessage::ClearRequested => {
                self.history_state.clear_confirm_visible = true;
                Task::none()
            }
            HistoryMessage::ClearConfirmed => self.clear_history_confirmed(),
            HistoryMessage::ClearFinished(result) => self.history_cleared(result),
            HistoryMessage::ClearCanceled => {
                self.history_state.clear_confirm_visible = false;
                Task::none()
            }
            HistoryMessage::Refresh => {
                self.history_state.invalidate();
                self.load_history_entries()
            }
        }
    }
}
