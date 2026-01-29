// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::wallhaven;
use crate::ui::online::{OnlineMessage, PageInfo, WallpaperLoadStatus};
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use iced::widget::image::Handle;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn load_online_page_success(
        &mut self,
        wallpapers: Vec<wallhaven::OnlineWallpaper>,
        last_page: bool,
        total_pages: usize,
        current_page: usize,
    ) -> Task<AppMessage> {
        // 添加新壁纸到列表，并开始加载缩略图
        self.online_state.current_page = current_page;
        self.online_state.total_pages = total_pages;

        // 判断是否是最后一页：
        // 如果 current_page == total_pages && current_page == 1 && data 为空，说明无数据
        // 否则 last_page（布尔值）表示已加载到最后一页
        let is_empty_data = wallpapers.is_empty();
        let is_first_and_last_page = current_page == 1 && total_pages == 1;
        self.online_state.last_page = if is_empty_data && is_first_and_last_page {
            // 无数据情况：last_page 为 false
            false
        } else {
            last_page
        };
        self.online_state.has_loaded = true; // 标记已加载过数据

        // 处理空数据但非最后一页的情况：自动加载下一页
        if is_empty_data && !last_page && current_page < total_pages {
            // 空数据但还有后续页面，继续加载下一页
            self.online_state.loading_page = false; // 先设置为 false，避免重复加载
            return self.load_online_page();
        }

        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        let cache_path = self.config.data.cache_path.clone();

        let start_idx = self.online_state.wallpapers.len();
        let mut tasks = Vec::new();
        for (offset, wallpaper) in wallpapers.iter().enumerate() {
            let idx = start_idx + offset;
            let url = wallpaper.thumb_large.clone();
            let file_size = wallpaper.file_size;
            let proxy = proxy.clone();
            let cache_path = cache_path.clone();

            // 创建取消令牌
            let cancel_token = Arc::new(AtomicBool::new(false));
            self.online_state.thumb_load_cancel_tokens.push(cancel_token.clone());

            tasks.push(Task::perform(
                async_task::async_load_online_wallpaper_thumb_with_cache_with_cancel(
                    url,
                    file_size,
                    cache_path,
                    proxy,
                    cancel_token,
                ),
                move |result| match result {
                    Ok(handle) => OnlineMessage::ThumbLoaded(idx, handle).into(),
                    Err(_) => OnlineMessage::ThumbLoaded(idx, Handle::from_bytes(vec![])).into(),
                },
            ));
        }

        // 保存原始数据
        for wallpaper in &wallpapers {
            self.online_state.wallpapers_data.push(wallpaper.clone());
            self.online_state.wallpapers.push(WallpaperLoadStatus::Loading);
        }

        // 在添加完当前页数据后记录分页信息
        // 这样分页标识就可以在当前页数据的下面显示
        if !wallpapers.is_empty() {
            self.online_state.page_info.push(PageInfo {
                end_index: self.online_state.wallpapers.len(),
                page_num: current_page,
            });
        }

        self.online_state.loading_page = false;

        Task::batch(tasks)
    }

    pub(in crate::ui::online) fn load_online_page_failed(&mut self, error: String) -> Task<AppMessage> {
        // 加载失败
        self.online_state.loading_page = false;
        self.online_state.has_loaded = true; // 标记已加载过数据（虽然失败了）
        error!("[在线壁纸] 加载页面失败: {}", error);

        // 检查是否是超时或连接错误，显示气泡通知
        if error.contains("请求超时") || error.contains("连接失败") {
            return self.show_notification(error, NotificationType::Error);
        }

        Task::none()
    }
}
