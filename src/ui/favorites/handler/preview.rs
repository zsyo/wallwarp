// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏项预览模态（打开/翻页/关闭）

use crate::ui::favorites::FavoritesMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use tracing::warn;

impl App {
    /// 打开预览模态并加载原图
    ///
    /// 本地项直接从磁盘读取；在线项优先读已入库文件，否则下载原图到内存
    pub(in crate::ui::favorites) fn preview_favorite_entry(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };

        self.favorites_state.modal_visible = true;
        self.favorites_state.modal_index = index;
        self.favorites_state.modal_handle = None;

        if entry.fav.kind == crate::services::database::KIND_LOCAL {
            let path = crate::utils::helpers::get_absolute_path(&entry.fav.path);
            return Task::perform(async move { Handle::from_path(&path) }, |handle| {
                FavoritesMessage::ModalImageLoaded(handle).into()
            });
        }

        // 在线项：已入库读本地文件，否则下载原图
        let url = entry.fav.url.clone();
        let id = entry.fav.wallhaven_id.clone();
        let file_type = entry.fav.file_type.clone();
        let file_size = entry.fav.file_size.max(0) as u64;

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
                // 未入库：下载原图到缓存目录，完成后从缓存加载预览
                let proxy = self.config.resolved_proxy();
                let cache_path = self.config.data.cache_path.clone();
                let cache_path_for_load = cache_path.clone();
                Task::perform(
                    async move {
                        crate::services::download::DownloadService::download_image_to_cache(
                            &url, &cache_path, proxy,
                        )
                        .await
                        .map_err(|e| e.to_string())?;
                        Ok::<String, String>(url)
                    },
                    move |result: Result<String, String>| match result {
                        Ok(url) => {
                            FavoritesMessage::PreviewOriginalDownloaded {
                                url,
                                file_size,
                                cache_path: cache_path_for_load,
                                id,
                            }
                            .into()
                        }
                        Err(e) => {
                            warn!("[收藏夹] [ID:{}] 预览原图下载失败: {}", id, e);
                            AppMessage::None
                        }
                    },
                )
            }
        }
    }

    /// 预览原图缓存下载完成：从缓存加载为 Handle（两段式加载的第二段）
    pub(in crate::ui::favorites) fn favorite_preview_original_downloaded(
        &mut self,
        url: String,
        file_size: u64,
        cache_path: String,
        id: String,
    ) -> Task<AppMessage> {
        Task::perform(
            crate::services::download::DownloadService::load_thumb_with_cache(
                url,
                file_size,
                cache_path,
                None,
            ),
            move |handle_result| match handle_result {
                Ok(handle) => FavoritesMessage::ModalImageLoaded(handle).into(),
                Err(e) => {
                    warn!("[收藏夹] [ID:{}] 预览原图加载失败: {}", id, e);
                    AppMessage::None
                }
            },
        )
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
        Task::none()
    }
}
