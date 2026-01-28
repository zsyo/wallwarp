// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸消息模块
//!
//! 定义本地壁纸页面的消息类型

use crate::services::local::Wallpaper;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

/// 本地壁纸页面消息
#[derive(Debug, Clone)]
pub enum LocalMessage {
    /// 加载壁纸列表
    LoadWallpapers,
    /// 壁纸列表加载成功
    LoadWallpapersSuccess(Vec<String>),
    /// 加载页面
    LoadPage,
    /// 页面加载成功
    LoadPageSuccess(Vec<(usize, Wallpaper)>),
    /// 滚动到底部
    ScrollToBottom,
    /// 检查是否需要自动加载下一页
    CheckAndLoadNextPage,
    /// 显示模态窗口
    ShowModal(usize),
    /// 关闭模态窗口
    CloseModal,
    /// 下一张图片
    NextImage,
    /// 上一张图片
    PreviousImage,
    /// 在文件夹中查看
    ViewInFolder(usize),
    /// 设置壁纸
    SetWallpaper(usize),
    /// 显示删除确认对话框
    ShowDeleteConfirm(usize),
    /// 关闭删除确认对话框
    CloseDeleteConfirm,
    /// 确认删除
    ConfirmDelete(usize),
    /// 模态窗口图片加载完成
    ModalImageLoaded(Handle),
}

/// 壁纸加载状态
#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    /// 加载中
    Loading,
    /// 已加载
    Loaded(Wallpaper),
}

impl From<LocalMessage> for AppMessage {
    fn from(local_message: LocalMessage) -> AppMessage {
        AppMessage::Local(local_message)
    }
}

impl App {
    /// 处理本地壁纸相关消息
    pub fn handle_local_message(&mut self, msg: LocalMessage) -> Task<AppMessage> {
        match msg {
            LocalMessage::LoadWallpapers => self.load_local_wallpapers(),
            LocalMessage::LoadWallpapersSuccess(paths) => self.load_local_wallpapers_success(paths),
            LocalMessage::LoadPage => self.load_local_page(),
            LocalMessage::LoadPageSuccess(wallpapers_with_idx) => self.load_local_page_success(wallpapers_with_idx),
            LocalMessage::ShowModal(index) => self.show_local_modal(index),
            LocalMessage::ModalImageLoaded(handle) => self.local_modal_image_loaded(handle),
            LocalMessage::CloseModal => self.close_local_modal(),
            LocalMessage::NextImage => self.next_local_image(),
            LocalMessage::PreviousImage => self.previous_local_image(),
            LocalMessage::ScrollToBottom => self.local_scroll_to_bottom(),
            LocalMessage::CheckAndLoadNextPage => self.load_local_next_page(),
            LocalMessage::ViewInFolder(index) => self.view_local_file(index),
            LocalMessage::ShowDeleteConfirm(index) => self.show_local_delete_confirm(index),
            LocalMessage::CloseDeleteConfirm => self.close_local_delete_confirm(),
            LocalMessage::ConfirmDelete(index) => self.confirm_local_delete(index),
            LocalMessage::SetWallpaper(index) => self.local_set_as_wallpaper(index),
        }
    }
}
