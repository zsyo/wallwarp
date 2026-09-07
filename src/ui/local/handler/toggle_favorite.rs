// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 本地壁纸收藏/取消收藏

use crate::services::database::{DatabaseManager, FavoriteDB, FavoritesRepository, KIND_LOCAL};
use crate::ui::local::message::WallpaperLoadStatus;
use crate::ui::local::LocalMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use tracing::{info, warn};

/// 本地文件收藏主键前缀（favorites 表 wallhaven_id 字段）
pub(crate) const LOCAL_FAVORITE_PREFIX: &str = "file:";

impl App {
    /// 切换本地壁纸收藏状态（卡片心形按钮）
    pub(in crate::ui::local) fn toggle_local_favorite(&mut self, index: usize) -> Task<AppMessage> {
        let Some(status) = self.local_state.wallpapers.get(index) else {
            return Task::none();
        };
        let WallpaperLoadStatus::Loaded(wallpaper) = status else {
            return Task::none();
        };

        // 主键 = file:{规范化绝对路径}
        let normalized = helpers::normalize_path(&wallpaper.path);
        let key = format!("{}{}", LOCAL_FAVORITE_PREFIX, normalized);
        let key_for_msg = key.clone();
        let is_favorite = self.local_state.favorite_paths.contains(&normalized);

        if is_favorite {
            let key_for_db = key.clone();
            return Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || -> Result<(), String> {
                        let Some(db) = DatabaseManager::try_get() else {
                            return Err("数据库未初始化".to_string());
                        };
                        FavoritesRepository::new(db.connection().clone()).remove_favorite(&key_for_db)
                    })
                    .await
                    .map_err(|e| e.to_string())?
                },
            move |result| LocalMessage::FavoriteToggled { key: key_for_msg, result }.into(),
        );
    }

        // 收藏：保存元数据快照（path 存绝对路径，收藏夹页读取不依赖 data_path 锚点）
        let title = std::path::Path::new(&normalized)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| normalized.clone());

        let fav = FavoriteDB {
            wallhaven_id: key.clone(),
            kind: KIND_LOCAL.to_string(),
            title,
            url: String::new(),
            path: normalized.clone(),
            thumb_url: String::new(),
            file_type: String::new(),
            file_size: wallpaper.file_size as i64,
            width: wallpaper.width as i64,
            height: wallpaper.height as i64,
            purity: String::new(),
            resolution: helpers::format_resolution(wallpaper.width, wallpaper.height),
            ratio: String::new(),
            category: String::new(),
            group_id: None,
            created_at: chrono::Utc::now().timestamp(),
        };

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    FavoritesRepository::new(db.connection().clone()).upsert_favorite(&fav)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| LocalMessage::FavoriteToggled { key, result }.into(),
        )
    }

    /// 收藏状态切换完成：更新内存标记并提示
    pub(in crate::ui::local) fn local_favorite_toggled(
        &mut self,
        key: String,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                let normalized = key
                    .strip_prefix(LOCAL_FAVORITE_PREFIX)
                    .unwrap_or(&key)
                    .to_string();
                let now_favorite = match DatabaseManager::try_get() {
                    Some(db) => FavoritesRepository::new(db.connection().clone())
                        .is_favorite(&key)
                        .unwrap_or(false),
                    None => false,
                };

                if now_favorite {
                    self.local_state.favorite_paths.insert(normalized.clone());
                    info!("[本地壁纸] [{}] 已收藏", normalized);
                    // 收藏数据已变化，收藏夹页下次进入时重载
                    self.favorites_state.loaded = false;
                    self.show_notification(
                        self.i18n.t("favorites.added").to_string(),
                        NotificationType::Success,
                    )
                } else {
                    self.local_state.favorite_paths.remove(&normalized);
                    info!("[本地壁纸] [{}] 已取消收藏", normalized);
                    // 收藏数据已变化，收藏夹页下次进入时重载
                    self.favorites_state.loaded = false;
                    self.show_notification(
                        self.i18n.t("favorites.removed-toast").to_string(),
                        NotificationType::Info,
                    )
                }
            }
            Err(e) => {
                warn!("[本地壁纸] [{}] 收藏状态切换失败: {}", key, e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.favorite-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
