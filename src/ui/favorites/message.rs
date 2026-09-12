// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面消息

use crate::ui::favorites::state::FavoriteEntry;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

/// 收藏夹类型筛选项（全部 / 在线 / 本地）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TypeFilter {
    #[default]
    All,
    Online,
    Local,
}

/// 收藏夹时间筛选项（全部 / 今日 / 三天内 / 本周 / 本月）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimeFilter {
    #[default]
    All,
    Today,
    ThreeDays,
    ThisWeek,
    ThisMonth,
}

/// 收藏夹页面消息
#[derive(Debug, Clone)]
pub enum FavoritesMessage {
    /// 从数据库加载收藏项
    Load,
    /// 收藏项加载完成
    Loaded(Vec<FavoriteEntry>),
    /// 在线项缩略图加载完成（None 表示加载失败，显示占位）
    ThumbLoaded { index: usize, handle: Option<Handle> },
    /// 设为壁纸（在线项未入库时自动下载后应用）
    ApplyEntry(usize),
    /// 下载该在线收藏项到壁纸库
    DownloadEntry(usize),
    /// 在文件夹中查看（仅本地项）
    OpenLocation(usize),
    /// 预览该收藏项（打开模态）
    PreviewEntry(usize),
    /// 预览上一张
    PreviousImage,
    /// 预览下一张
    NextImage,
    /// 预览原图加载完成（缓存/下载/本地读取共用）
    ModalImageLoaded(Handle),
    /// 预览原图下载进度更新（已下载字节 / 总字节）
    ModalImageProgress(u64, u64),
    /// 预览原图下载失败
    ModalImageDownloadFailed(String),
    /// 关闭预览
    CloseModal,
    /// 请求移除收藏（显示确认框）
    RemoveEntry(usize),
    /// 确认移除
    RemoveConfirmed,
    /// 移除完成
    RemoveFinished { index: usize, result: Result<(), String> },
    /// 取消移除
    RemoveCanceled,
    /// 切换类型筛选
    TypeFilterChanged(TypeFilter),
    /// 切换时间筛选
    TimeFilterChanged(TimeFilter),
    /// 切换排序方向（收藏时间正序/倒序）
    SortOrderToggled,
    /// 滚动到底部：追加下一批收藏项
    LoadMore,
    /// 类型筛选下拉展开/收起切换
    TypeFilterExpanded,
    /// 类型筛选下拉收起（点击面板外部）
    TypeFilterDismiss,
    /// 时间筛选下拉展开/收起切换
    TimeFilterExpanded,
    /// 时间筛选下拉收起（点击面板外部）
    TimeFilterDismiss,
    /// 刷新
    Refresh,
    /// 检查已加载缩略图的缓存文件是否仍存在，失效项重载（进入页面时触发）
    CheckThumbs,
}

impl From<FavoritesMessage> for AppMessage {
    fn from(msg: FavoritesMessage) -> AppMessage {
        AppMessage::Favorites(msg)
    }
}

impl App {
    /// 处理收藏夹相关消息
    pub fn handle_favorites_message(&mut self, msg: FavoritesMessage) -> Task<AppMessage> {
        match msg {
            FavoritesMessage::Load => self.load_favorites(),
            FavoritesMessage::Loaded(entries) => self.favorites_loaded(entries),
            FavoritesMessage::ThumbLoaded { index, handle } => {
                self.favorite_thumb_loaded(index, handle)
            }
            FavoritesMessage::ApplyEntry(index) => self.apply_favorite_entry(index),
            FavoritesMessage::DownloadEntry(index) => self.download_favorite_entry(index),
            FavoritesMessage::OpenLocation(index) => self.view_favorite_file(index),
            FavoritesMessage::PreviewEntry(index) => self.preview_favorite_entry(index),
            FavoritesMessage::PreviousImage => self.previous_favorite_image(),
            FavoritesMessage::NextImage => self.next_favorite_image(),
            FavoritesMessage::ModalImageLoaded(handle) => {
                self.favorite_modal_image_loaded(handle)
            }
            FavoritesMessage::ModalImageProgress(downloaded, total) => {
                self.favorite_modal_image_progress(downloaded, total)
            }
            FavoritesMessage::ModalImageDownloadFailed(e) => {
                self.favorite_modal_image_download_failed(e)
            }
            FavoritesMessage::CloseModal => {
                self.favorites_state.close_modal();
                Task::none()
            }
            FavoritesMessage::RemoveEntry(index) => {
                self.favorites_state.remove_target = Some(index);
                Task::none()
            }
            FavoritesMessage::RemoveConfirmed => self.remove_favorite_entry(),
            FavoritesMessage::RemoveFinished { index, result } => {
                self.favorite_entry_removed(index, result)
            }
            FavoritesMessage::RemoveCanceled => {
                self.favorites_state.remove_target = None;
                Task::none()
            }
            FavoritesMessage::TypeFilterChanged(filter) => {
                self.favorites_state.type_filter = filter;
                self.favorites_state.type_filter_expanded = false;
                self.reload_filtered_entries()
            }
            FavoritesMessage::TimeFilterChanged(filter) => {
                self.favorites_state.time_filter = filter;
                self.favorites_state.time_filter_expanded = false;
                self.reload_filtered_entries()
            }
            FavoritesMessage::SortOrderToggled => {
                self.favorites_state.sort_descending = !self.favorites_state.sort_descending;
                self.reload_filtered_entries()
            }
            FavoritesMessage::LoadMore => {
                if self.favorites_state.load_more() {
                    self.load_favorite_thumbs()
                } else {
                    Task::none()
                }
            }
            FavoritesMessage::TypeFilterExpanded => {
                self.favorites_state.type_filter_expanded =
                    !self.favorites_state.type_filter_expanded;
                Task::none()
            }
            FavoritesMessage::TypeFilterDismiss => {
                self.favorites_state.type_filter_expanded = false;
                Task::none()
            }
            FavoritesMessage::TimeFilterExpanded => {
                self.favorites_state.time_filter_expanded =
                    !self.favorites_state.time_filter_expanded;
                Task::none()
            }
            FavoritesMessage::TimeFilterDismiss => {
                self.favorites_state.time_filter_expanded = false;
                Task::none()
            }
            FavoritesMessage::Refresh => {
                self.favorites_state.invalidate();
                self.load_favorites()
            }
            FavoritesMessage::CheckThumbs => self.favorites_check_thumbs(),
        }
    }
}
