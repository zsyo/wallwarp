// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏项预览模态（打开/翻页/关闭/原图下载进度）

use crate::services::async_task;
use crate::services::download::DOWNLOAD_CANCELLED;
use crate::ui::favorites::FavoritesMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tracing::debug;

impl App {
    /// 打开预览模态并加载原图
    ///
    /// 本地项直接从磁盘读取；在线项优先读已入库文件，否则流式下载原图
    /// （进度经广播通道回传，模态占位符显示环形进度）
    pub(in crate::ui::favorites) fn preview_favorite_entry(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        // 先取出条目关键字段，避免与后续状态修改产生借用冲突
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };
        let kind = entry.fav.kind.clone();
        let local_path = entry.fav.path.clone();
        let url = entry.fav.url.clone();
        let id = entry.fav.wallhaven_id.clone();
        let file_type = entry.fav.file_type.clone();
        let file_size = entry.fav.file_size.max(0) as u64;

        self.favorites_state.modal_visible = true;
        self.favorites_state.modal_index = index;
        self.favorites_state.modal_handle = None;
        // 复位上一张的下载状态（取消在途下载，翻页/重开均经过此处）
        self.favorites_state.cancel_modal_download();

        if kind == crate::services::database::KIND_LOCAL {
            let path = crate::utils::helpers::get_absolute_path(&local_path);
            return Task::perform(async move { Handle::from_path(&path) }, |handle| {
                FavoritesMessage::ModalImageLoaded(handle).into()
            });
        }

        // 在线项：已入库读本地文件，否则流式下载原图
        let location = self.resolve_online_file(&url, &id, &file_type, file_size);
        match location.source {
            Some(hit) => {
                let path = match hit {
                    crate::ui::online::OnlineFileHit::InData => crate::utils::helpers::get_absolute_path(
                        &location.target_path.to_string_lossy(),
                    ),
                    crate::ui::online::OnlineFileHit::InCache(cache_path) => cache_path,
                };
                Task::perform(async move { Handle::from_path(&path) }, |handle| {
                    FavoritesMessage::ModalImageLoaded(handle).into()
                })
            }
            None => {
                // 未入库：流式下载原图到缓存目录（落盘路径与在线页缓存键一致，
                // 检查顺序与在线页一致：壁纸目录 → 缓存 → 下载，此处为第三步）
                let proxy = self.config.resolved_proxy();
                let cache_path = self.config.data.cache_path.clone();

                // 登记取消令牌（关闭模态/翻页时经 cancel_modal_download 中断下载）
                let cancel_token = Arc::new(AtomicBool::new(false));
                self.favorites_state.modal_download_cancel_token = Some(cancel_token.clone());

                Task::perform(
                    async_task::async_load_online_wallpaper_image_with_streaming(
                        url,
                        file_size,
                        cache_path,
                        proxy,
                        cancel_token,
                    ),
                    |result| match result {
                        Ok(handle) => FavoritesMessage::ModalImageLoaded(handle).into(),
                        Err(e) => FavoritesMessage::ModalImageDownloadFailed(e.to_string()).into(),
                    },
                )
            }
        }
    }

    /// 预览上一张
    pub(in crate::ui::favorites) fn previous_favorite_image(&mut self) -> Task<AppMessage> {
        let index = self.favorites_state.modal_index;
        if index > 0 {
            return self.preview_favorite_entry(index - 1);
        }
        Task::none()
    }

    /// 预览下一张
    pub(in crate::ui::favorites) fn next_favorite_image(&mut self) -> Task<AppMessage> {
        let index = self.favorites_state.modal_index;
        if index + 1 < self.favorites_state.entries.len() {
            return self.preview_favorite_entry(index + 1);
        }
        Task::none()
    }

    /// 预览原图加载完成
    pub(in crate::ui::favorites) fn favorite_modal_image_loaded(
        &mut self,
        handle: Handle,
    ) -> Task<AppMessage> {
        if self.favorites_state.modal_visible {
            self.favorites_state.modal_handle = Some(handle);
        }
        // 下载已结束：清掉取消令牌，后续残余进度消息不再生效；
        // 不重置进度字段（同在线页）：大图解码上屏前底层占位符仍可见，
        // 保留最后一次的进度环可避免闪现"图片加载中"文字
        self.favorites_state.modal_download_cancel_token = None;
        Task::none()
    }

    /// 预览原图下载进度更新（服务层每 5% 一条，经广播订阅回传）
    pub(in crate::ui::favorites) fn favorite_modal_image_progress(
        &mut self,
        downloaded: u64,
        total: u64,
    ) -> Task<AppMessage> {
        // 模态已关闭或无在途下载（他页进度/已结束）时忽略
        if !self.favorites_state.modal_visible
            || self.favorites_state.modal_download_cancel_token.is_none()
        {
            return Task::none();
        }

        self.favorites_state.modal_downloaded_bytes = downloaded;
        self.favorites_state.modal_total_bytes = total;
        if total > 0 {
            self.favorites_state.modal_download_progress =
                (downloaded as f32 / total as f32).min(1.0);
        }
        Task::none()
    }

    /// 预览原图下载失败
    pub(in crate::ui::favorites) fn favorite_modal_image_download_failed(
        &mut self,
        error: String,
    ) -> Task<AppMessage> {
        // 失败细节由 streaming 层 error 记录，UI 层降为 debug 避免双记
        debug!("[收藏夹] [模态预览] 原图下载失败: {}", error);
        // 切换图片场景：这是旧任务被取消的残余消息，新任务已在途，
        // 不重置其进度显示、不清掉新任务的取消令牌
        if error == DOWNLOAD_CANCELLED && self.favorites_state.modal_download_cancel_token.is_some()
        {
            return Task::none();
        }
        // 重置下载状态（环回落为纯轨道圈 + 加载中文案）
        self.favorites_state.modal_download_progress = 0.0;
        self.favorites_state.modal_downloaded_bytes = 0;
        self.favorites_state.modal_total_bytes = 0;
        self.favorites_state.modal_download_cancel_token = None;
        Task::none()
    }
}
