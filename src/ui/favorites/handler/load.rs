// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏夹数据加载（收藏项 + 分组）

use crate::services::database::{DatabaseManager, FavoritesRepository};
use crate::ui::favorites::message::GroupFilter;
use crate::ui::favorites::state::FavoriteEntry;
use crate::ui::favorites::FavoritesMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use std::path::Path;
use tracing::{info, warn};

/// 数据库加载（阻塞线程中执行）
async fn load_favorites_from_db() -> Result<
    (
        Vec<crate::services::database::FavoriteDB>,
        Vec<crate::services::database::FavoriteGroupDB>,
    ),
    String,
> {
    tokio::task::spawn_blocking(move || -> Result<_, String> {
        let Some(db) = DatabaseManager::try_get() else {
            return Err("数据库未初始化".to_string());
        };
        let repo = FavoritesRepository::new(db.connection().clone());
        let favorites = repo.load_all_favorites()?;
        let groups = repo.load_groups()?;
        Ok((favorites, groups))
    })
    .await
    .map_err(|e| e.to_string())?
}

impl App {
    /// 加载收藏项与分组，并启动缩略图加载
    pub(in crate::ui::favorites) fn load_favorites(&mut self) -> Task<AppMessage> {
        self.favorites_state.loaded = true;

        Task::perform(load_favorites_from_db(), |result| match result {
            Ok((favorites, groups)) => {
                FavoritesMessage::Loaded(
                    favorites
                        .into_iter()
                        .map(|fav| FavoriteEntry {
                            fav,
                            in_library: false,
                        })
                        .collect(),
                    groups,
                )
                .into()
            }
            Err(e) => {
                warn!("[收藏夹] [DB] 加载失败: {}", e);
                FavoritesMessage::Loaded(Vec::new(), Vec::new()).into()
            }
        })
    }

    /// 收藏项加载完成：标记本地项存在性，启动在线项缩略图加载
    pub(in crate::ui::favorites) fn favorites_loaded(
        &mut self,
        entries: Vec<FavoriteEntry>,
        groups: Vec<crate::services::database::FavoriteGroupDB>,
    ) -> Task<AppMessage> {
        info!("[收藏夹] [DB] 收藏 {} 项, {} 个分组", entries.len(), groups.len());

        let absolute_data_dir =
            crate::utils::helpers::get_absolute_path(&self.config.data.data_path);
        let mut entries = entries;
        for entry in &mut entries {
            // 本地项按绝对路径判断存在性；在线项判断原图是否已下载入库
            if entry.fav.kind == crate::services::database::KIND_LOCAL {
                entry.fav.path = Self::normalize_local_favorite_path(
                    &entry.fav.path,
                    &absolute_data_dir,
                );
                entry.in_library = Path::new(&entry.fav.path).exists();
            } else {
                let file_name = crate::services::wallhaven::generate_file_name(
                    &entry.fav.wallhaven_id,
                    entry
                        .fav
                        .file_type
                        .split('/')
                        .next_back()
                        .unwrap_or("jpg"),
                );
                entry.in_library =
                    Path::new(&absolute_data_dir).join(&file_name).exists();
            }
        }

        self.favorites_state.groups = groups;
        self.favorites_state.all_entries = entries;
        self.favorites_state.apply_filter();

        // 已收藏项 id 集合刷新（在线页/本地页心形按钮状态）
        self.refresh_favorite_markers();

        self.load_favorite_thumbs()
    }

    /// 为当前筛选后的收藏项启动缩略图加载
    pub(in crate::ui::favorites) fn load_favorite_thumbs(&mut self) -> Task<AppMessage> {
        let proxy = self.config.resolved_proxy();
        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (index, entry) in self.favorites_state.entries.iter().enumerate() {
            // 本地项源文件已失效：跳过缩略图任务，直接标记失败（卡片显示失效占位）
            if entry.fav.kind == crate::services::database::KIND_LOCAL && !entry.in_library {
                self.favorites_state.thumbs[index] = crate::ui::favorites::state::ThumbState::Failed;
                continue;
            }

            let task = match entry.fav.kind.as_str() {
                crate::services::database::KIND_LOCAL => {
                    // 本地项：复用历史页的本地缩略图生成
                    let path = crate::utils::helpers::get_absolute_path(&entry.fav.path);
                    let cache_path = cache_path.clone();
                    Task::perform(
                        crate::services::async_task::async_load_single_wallpaper_with_fallback(
                            path,
                            cache_path,
                        ),
                        move |result| {
                            let handle = result.ok().and_then(|w| {
                                w.image_handle
                                    .clone()
                                    .or_else(|| Some(iced::widget::image::Handle::from_path(
                                        &w.thumbnail_path,
                                    )))
                            });
                            FavoritesMessage::ThumbLoaded { index, handle }.into()
                        },
                    )
                }
                _ => {
                    // 在线项：复用在线页的缩略图缓存加载
                    let url = entry.fav.thumb_url.clone();
                    let file_size = entry.fav.file_size.max(0) as u64;
                    let proxy = proxy.clone();
                    let cache_path = cache_path.clone();
                    Task::perform(
                        crate::services::async_task::async_load_online_wallpaper_thumb_with_cache(
                            url,
                            file_size,
                            cache_path,
                            proxy,
                        ),
                        move |result| {
                            FavoritesMessage::ThumbLoaded {
                                index,
                                handle: result.ok(),
                            }
                            .into()
                        },
                    )
                }
            };
            tasks.push(task);
        }

        Task::batch(tasks)
    }

    /// 缩略图加载完成
    pub(in crate::ui::favorites) fn favorite_thumb_loaded(
        &mut self,
        index: usize,
        handle: Option<Handle>,
    ) -> Task<AppMessage> {
        if index < self.favorites_state.entries.len() {
            self.favorites_state.thumbs[index] = match handle {
                Some(handle) => crate::ui::favorites::state::ThumbState::Loaded(handle),
                None => crate::ui::favorites::state::ThumbState::Failed,
            };
        }
        Task::none()
    }

    /// 分组筛选变化后重建视图与缩略图
    pub(in crate::ui::favorites) fn reload_filtered_entries(&mut self) -> Task<AppMessage> {
        self.favorites_state.apply_filter();
        self.load_favorite_thumbs()
    }

    /// 刷新各页面的收藏标记集合（卡片心形按钮实心/空心状态）
    pub fn refresh_favorite_markers(&mut self) {
        let ids: std::collections::HashSet<String> = self
            .favorites_state
            .all_entries
            .iter()
            .map(|e| e.fav.wallhaven_id.clone())
            .collect();

        self.online_state.favorite_ids = ids
            .iter()
            .filter(|id| !id.starts_with("file:"))
            .cloned()
            .collect();

        self.local_state.favorite_paths = ids
            .iter()
            .filter_map(|id| id.strip_prefix("file:").map(|p| p.to_string()))
            .collect();
    }

    /// 当前分组筛选对应的分组 id（删除分组确认框用；All/Ungrouped 时为 None）
    pub(in crate::ui::favorites) fn current_filter_group(&self) -> Option<i64> {
        match self.favorites_state.group_filter {
            GroupFilter::Group(id) => Some(id),
            _ => None,
        }
    }

    /// 规范化本地收藏项的路径（兼容历史数据）
    ///
    /// 新数据存绝对路径；历史数据可能存相对 data_path 的路径——
    /// 相对路径优先按 data_path 拼接（文件存在时），否则回退当前目录拼接
    pub(in crate::ui::favorites) fn normalize_local_favorite_path(
        path: &str,
        absolute_data_dir: &str,
    ) -> String {
        if std::path::Path::new(path).is_absolute() {
            return path.to_string();
        }
        let joined_with_data_dir = std::path::Path::new(absolute_data_dir).join(path);
        if joined_with_data_dir.exists() {
            return joined_with_data_dir.to_string_lossy().to_string();
        }
        crate::utils::helpers::get_absolute_path(path)
    }
}
