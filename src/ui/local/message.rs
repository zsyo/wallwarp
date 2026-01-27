// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::local::Wallpaper;
use crate::ui::AppMessage;

#[derive(Debug, Clone)]
pub enum LocalMessage {
    LoadWallpapers,
    LoadWallpapersSuccess(Vec<String>),
    LoadWallpapersFailed(String),
    LoadPage,
    LoadPageSuccess(Vec<(usize, Wallpaper)>),
    LoadPageFailed(String),
    WallpaperSelected(Wallpaper),
    ScrollToBottom,
    CheckAndLoadNextPage, // 检查是否需要自动加载下一页
    ShowModal(usize),
    CloseModal,
    NextImage,
    PreviousImage,
    ViewInFolder(usize),
    SetWallpaper(usize),
    ShowDeleteConfirm(usize),
    CloseDeleteConfirm,
    ConfirmDelete(usize),
    ModalImageLoaded(iced::widget::image::Handle),
}

#[derive(Debug, Clone)]
pub enum WallpaperLoadStatus {
    Loading,
    Loaded(Wallpaper),
}

impl From<LocalMessage> for AppMessage {
    fn from(local_message: LocalMessage) -> AppMessage {
        AppMessage::Local(local_message)
    }
}