// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹页面消息

use crate::services::database::FavoriteGroupDB;
use crate::ui::favorites::state::FavoriteEntry;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

/// 收藏夹分组筛选项（全部 / 未分组 / 指定分组）
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GroupFilter {
    #[default]
    All,
    Ungrouped,
    Group(i64),
}

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
    /// 从数据库加载收藏项与分组
    Load,
    /// 收藏项与分组加载完成
    Loaded(Vec<FavoriteEntry>, Vec<FavoriteGroupDB>),
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
    /// 预览原图加载完成
    ModalImageLoaded(Handle),
    /// 预览原图缓存下载完成，从缓存加载（在线项未入库时两段式加载的中间消息）
    PreviewOriginalDownloaded {
        url: String,
        file_size: u64,
        cache_path: String,
        id: String,
    },
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
    /// 移动收藏项到分组（None = 移出分组）
    MoveToGroup { index: usize, group_id: Option<i64> },
    /// 移动完成
    MoveFinished { index: usize, group_id: Option<i64>, result: Result<(), String> },
    /// 切换分组筛选
    GroupFilterChanged(GroupFilter),
    /// 切换类型筛选
    TypeFilterChanged(TypeFilter),
    /// 切换时间筛选
    TimeFilterChanged(TimeFilter),
    /// 切换排序方向（收藏时间正序/倒序）
    SortOrderToggled,
    /// 分组筛选下拉展开/收起切换
    GroupFilterExpanded,
    /// 分组筛选下拉收起（点击面板外部）
    GroupFilterDismiss,
    /// 类型筛选下拉展开/收起切换
    TypeFilterExpanded,
    /// 类型筛选下拉收起（点击面板外部）
    TypeFilterDismiss,
    /// 时间筛选下拉展开/收起切换
    TimeFilterExpanded,
    /// 时间筛选下拉收起（点击面板外部）
    TimeFilterDismiss,
    /// 请求新建分组（显示输入对话框）
    CreateGroupRequested,
    /// 确认新建分组
    CreateGroupConfirmed,
    /// 新建分组完成
    CreateGroupFinished(Result<i64, String>),
    /// 取消新建分组
    CreateGroupCanceled,
    /// 新建分组输入内容变更
    CreateGroupNameChanged(String),
    /// 请求删除当前筛选分组（显示确认框）
    DeleteGroupRequested,
    /// 确认删除分组
    DeleteGroupConfirmed,
    /// 删除分组完成
    DeleteGroupFinished(Result<(), String>),
    /// 取消删除分组
    DeleteGroupCanceled,
    /// 刷新
    Refresh,
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
            FavoritesMessage::Loaded(entries, groups) => {
                self.favorites_loaded(entries, groups)
            }
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
            FavoritesMessage::PreviewOriginalDownloaded {
                url,
                file_size,
                cache_path,
                id,
            } => self.favorite_preview_original_downloaded(url, file_size, cache_path, id),
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
            FavoritesMessage::MoveToGroup { index, group_id } => {
                self.move_favorite_to_group(index, group_id)
            }
            FavoritesMessage::MoveFinished {
                index,
                group_id,
                result,
            } => self.favorite_moved(index, group_id, result),
            FavoritesMessage::GroupFilterChanged(filter) => {
                self.favorites_state.group_filter = filter;
                self.favorites_state.group_filter_expanded = false;
                self.reload_filtered_entries()
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
            FavoritesMessage::GroupFilterExpanded => {
                self.favorites_state.group_filter_expanded =
                    !self.favorites_state.group_filter_expanded;
                Task::none()
            }
            FavoritesMessage::GroupFilterDismiss => {
                self.favorites_state.group_filter_expanded = false;
                Task::none()
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
            FavoritesMessage::CreateGroupRequested => {
                self.favorites_state.create_group_name.clear();
                self.favorites_state.create_group_visible = true;
                Task::none()
            }
            FavoritesMessage::CreateGroupConfirmed => self.create_favorite_group(),
            FavoritesMessage::CreateGroupFinished(result) => self.favorite_group_created(result),
            FavoritesMessage::CreateGroupCanceled => {
                self.favorites_state.create_group_visible = false;
                Task::none()
            }
            FavoritesMessage::CreateGroupNameChanged(name) => {
                self.favorites_state.create_group_name = name;
                Task::none()
            }
            FavoritesMessage::DeleteGroupRequested => {
                self.favorites_state.delete_group_confirm_visible = true;
                Task::none()
            }
            FavoritesMessage::DeleteGroupConfirmed => self.delete_favorite_group(),
            FavoritesMessage::DeleteGroupFinished(result) => self.favorite_group_deleted(result),
            FavoritesMessage::DeleteGroupCanceled => {
                self.favorites_state.delete_group_confirm_visible = false;
                Task::none()
            }
            FavoritesMessage::Refresh => {
                self.favorites_state.invalidate();
                self.load_favorites()
            }
        }
    }
}
