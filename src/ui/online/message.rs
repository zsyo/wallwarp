// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::{App, AppMessage};
use iced::Task;

/// 在线壁纸页面消息类型
#[derive(Debug, Clone)]
pub enum OnlineMessage {
    /// 加载壁纸
    LoadWallpapers,
    /// 加载壁纸成功
    LoadWallpapersSuccess(Vec<wallhaven::OnlineWallpaper>, bool, usize, usize),
    /// 加载壁纸失败
    LoadWallpapersFailed(String),
    /// 加载指定页
    LoadPage,
    /// 加载指定页成功
    LoadPageSuccess(Vec<wallhaven::OnlineWallpaper>, bool, usize, usize),
    /// 加载指定页失败
    LoadPageFailed(String),
    /// 滚动到底部
    ScrollToBottom,
    /// 检查并加载下一页
    CheckAndLoadNextPage,
    /// 显示模态窗口
    ShowModal(usize),
    /// 关闭模态窗口
    CloseModal,
    /// 下一张图片
    NextImage,
    /// 上一张图片
    PreviousImage,
    /// 下载壁纸
    DownloadWallpaper(usize),
    /// 从缓存下载
    DownloadFromCache(usize),
    /// 从缓存设置为壁纸
    SetAsWallpaperFromCache(usize),
    /// 设置为壁纸
    SetAsWallpaper(usize),
    /// 模态窗口图片加载完成
    ModalImageLoaded(iced::widget::image::Handle),
    /// 模态窗口图片下载完成
    ModalImageDownloaded(iced::widget::image::Handle),
    /// 模态窗口图片下载失败
    ModalImageDownloadFailed(String),
    /// 缩略图加载完成（内部消息，用于从异步任务传递 Handle）
    ThumbLoaded(usize, iced::widget::image::Handle),
    // 筛选条件相关消息
    /// 切换分类选择状态
    CategoryToggled(wallhaven::Category),
    /// 改变排序方式
    SortingChanged(wallhaven::Sorting),
    /// 切换纯净度选择状态
    PurityToggled(wallhaven::Purity),
    /// 改变分辨率
    ResolutionChanged(wallhaven::Resolution),
    /// 改变比例
    RatioChanged(wallhaven::Ratio),
    /// 改变颜色
    ColorChanged(wallhaven::ColorOption),
    /// 展开颜色选择器
    ColorPickerExpanded,
    /// 关闭颜色选择器
    ColorPickerDismiss,
    /// 改变时间范围
    TimeRangeChanged(wallhaven::TimeRange),
    /// 搜索文本改变
    SearchTextChanged(String),
    /// 执行搜索
    Search,
    /// 刷新
    Refresh,
    // 分辨率筛选器相关消息
    /// 展开分辨率选择器
    ResolutionPickerExpanded,
    /// 关闭分辨率选择器
    ResolutionPickerDismiss,
    /// 切换分辨率筛选模式
    ResolutionModeChanged(super::ResolutionMode),
    /// 切换分辨率选择状态（Exactly模式）
    ResolutionToggled(wallhaven::Resolution),
    /// 选择分辨率（AtLeast模式）
    ResolutionAtLeastSelected(wallhaven::Resolution),
    // 比例筛选器相关消息
    /// 展开比例选择器
    RatioPickerExpanded,
    /// 关闭比例选择器
    RatioPickerDismiss,
    /// 切换"全部横屏"选项
    RatioLandscapeToggled,
    /// 切换"全部竖屏"选项
    RatioPortraitToggled,
    /// 切换"全部"选项
    RatioAllToggled,
    /// 切换比例选择状态
    RatioToggled(wallhaven::AspectRatio),
    /// 展开排序方式选择器
    SortingPickerExpanded,
    /// 关闭排序方式选择器
    SortingPickerDismiss,
    /// 展开时间范围选择器
    TimeRangePickerExpanded,
    /// 关闭时间范围选择器
    TimeRangePickerDismiss,
}

impl From<OnlineMessage> for AppMessage {
    fn from(msg: OnlineMessage) -> AppMessage {
        AppMessage::Online(msg)
    }
}

impl App {
    /// 处理在线壁纸相关消息
    pub fn handle_online_message(&mut self, msg: OnlineMessage) -> Task<AppMessage> {
        match msg {
            OnlineMessage::LoadWallpapers => self.load_online_wallpapers(),
            OnlineMessage::LoadWallpapersSuccess(wallpapers, last_page, total_pages, current_page) => {
                self.load_online_wallpapers_success(wallpapers, last_page, total_pages, current_page)
            }
            OnlineMessage::LoadWallpapersFailed(error) => self.load_online_wallpapers_failed(error),
            OnlineMessage::LoadPage => self.load_online_page(),
            OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page) => {
                self.load_online_page_success(wallpapers, last_page, total_pages, current_page)
            }
            OnlineMessage::LoadPageFailed(error) => self.load_online_page_failed(error),
            OnlineMessage::ShowModal(index) => self.show_online_modal(index),
            OnlineMessage::ModalImageLoaded(handle) => self.online_modal_image_loaded(handle),
            OnlineMessage::ModalImageDownloaded(handle) => self.modal_image_downloaded(handle),
            OnlineMessage::ModalImageDownloadFailed(error) => self.modal_image_download_failed(error),
            OnlineMessage::CloseModal => self.close_online_modal(),
            OnlineMessage::NextImage => self.next_online_image(),
            OnlineMessage::PreviousImage => self.previous_online_image(),
            OnlineMessage::ThumbLoaded(idx, handle) => self.online_thumb_loaded(idx, handle),
            OnlineMessage::DownloadWallpaper(index) => self.download_online_wallpaper(index),
            OnlineMessage::DownloadFromCache(index) => self.download_from_cache(index),
            OnlineMessage::SetAsWallpaperFromCache(index) => self.set_wallpaper_from_cache(index),
            OnlineMessage::SetAsWallpaper(index) => self.set_online_wallpaper(index),
            OnlineMessage::CategoryToggled(category) => self.online_filter_category_toggled(category),
            OnlineMessage::SortingChanged(sorting) => self.online_filter_sorting_changed(sorting),
            OnlineMessage::PurityToggled(purity) => self.online_filter_purity_toggled(purity),
            OnlineMessage::SearchTextChanged(text) => self.online_filter_search_text_changed(text),
            OnlineMessage::Search => self.online_search(),
            OnlineMessage::Refresh => self.online_refresh(),
            OnlineMessage::ScrollToBottom => self.online_scroll_to_bottom(),
            OnlineMessage::CheckAndLoadNextPage => self.online_check_and_load_next_page(),
            OnlineMessage::ResolutionChanged(resolution) => self.online_filter_resolution_changed(resolution),
            OnlineMessage::RatioChanged(ratio) => self.online_filter_ratio_changed(ratio),
            OnlineMessage::ColorChanged(color) => self.online_filter_color_changed(color),
            OnlineMessage::ColorPickerExpanded => self.online_filter_color_picker_expanded(),
            OnlineMessage::ColorPickerDismiss => self.online_filter_color_picker_dismiss(),
            OnlineMessage::TimeRangeChanged(time_range) => self.online_filter_time_range_changed(time_range),
            OnlineMessage::ResolutionPickerExpanded => self.online_filter_resolution_picker_expanded(),
            OnlineMessage::ResolutionPickerDismiss => self.online_filter_resolution_picker_dismiss(),
            OnlineMessage::ResolutionModeChanged(mode) => self.online_filter_resolution_mode_changed(mode),
            OnlineMessage::ResolutionToggled(resolution) => self.online_filter_resolution_toggled(resolution),
            OnlineMessage::ResolutionAtLeastSelected(resolution) => {
                self.online_filter_resolution_atleast_selected(resolution)
            }
            OnlineMessage::RatioPickerExpanded => self.online_filter_ratio_picker_expanded(),
            OnlineMessage::RatioPickerDismiss => self.online_filter_ratio_picker_dismiss(),
            OnlineMessage::RatioLandscapeToggled => self.online_filter_ratio_landscape_toggled(),
            OnlineMessage::RatioPortraitToggled => self.online_filter_ratio_portrait_toggled(),
            OnlineMessage::RatioAllToggled => self.online_filter_ratio_all_toggled(),
            OnlineMessage::RatioToggled(ratio) => self.online_filter_ratio_toggled(ratio),
            OnlineMessage::SortingPickerExpanded => self.online_filter_sorting_picker_expanded(),
            OnlineMessage::SortingPickerDismiss => self.online_filter_sorting_picker_dismiss(),
            OnlineMessage::TimeRangePickerExpanded => self.online_filter_time_range_picker_expanded(),
            OnlineMessage::TimeRangePickerDismiss => self.online_filter_time_range_picker_dismiss(),
        }
    }
}
