// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线壁纸收藏/取消收藏

use crate::services::database::{FavoriteDB, FavoritesRepository, KIND_ONLINE};
use crate::services::database::DatabaseManager;
use crate::ui::online::OnlineMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{info, warn};

impl App {
    /// 切换在线壁纸收藏状态（卡片/弹窗心形按钮）
    pub(in crate::ui::online) fn toggle_online_favorite(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        let Some(wallpaper) = self.online_state.wallpapers_data.get(index) else {
            return Task::none();
        };
        let id = wallpaper.id.clone();
        let is_favorite = self.online_state.favorite_ids.contains(&id);

        if is_favorite {
            // 取消收藏
            let id_for_db = id.clone();
            return Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || -> Result<(), String> {
                        let Some(db) = DatabaseManager::try_get() else {
                            return Err("数据库未初始化".to_string());
                        };
                        FavoritesRepository::new(db.connection().clone())
                            .remove_favorite(&id_for_db)
                    })
                    .await
                    .map_err(|e| e.to_string())?
                },
                move |result| OnlineMessage::FavoriteToggled { id, result }.into(),
            );
        }

        // 收藏：保存完整元数据快照
        let fav = FavoriteDB {
            wallhaven_id: wallpaper.id.clone(),
            kind: KIND_ONLINE.to_string(),
            title: crate::services::wallhaven::generate_file_name(
                &wallpaper.id,
                wallpaper.file_type.split('/').next_back().unwrap_or("jpg"),
            ),
            url: wallpaper.path.clone(),
            path: wallpaper.url.clone(),
            thumb_url: wallpaper.thumb_large.clone(),
            file_type: wallpaper.file_type.clone(),
            file_size: wallpaper.file_size as i64,
            width: wallpaper.width as i64,
            height: wallpaper.height as i64,
            purity: wallpaper.purity.clone(),
            resolution: wallpaper.resolution.clone(),
            ratio: wallpaper.ratio.clone(),
            category: wallpaper.category.clone(),
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
            move |result| OnlineMessage::FavoriteToggled { id, result }.into(),
        )
    }

    /// 收藏状态切换完成：更新内存标记并提示
    pub(in crate::ui::online) fn online_favorite_toggled(
        &mut self,
        id: String,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                // 重新查询实际状态（切换方向由 DB 决定，避免内存竞态）
                let now_favorite = match DatabaseManager::try_get() {
                    Some(db) => FavoritesRepository::new(db.connection().clone())
                        .is_favorite(&id)
                        .unwrap_or(false),
                    None => false,
                };

                if now_favorite {
                    self.online_state.favorite_ids.insert(id.clone());
                    info!("[在线壁纸] [ID:{}] 已收藏", id);
                    // 收藏数据已变化，收藏夹页下次进入时重载
                    self.favorites_state.loaded = false;
                    self.show_notification(
                        self.i18n.t("favorites.added").to_string(),
                        NotificationType::Success,
                    )
                } else {
                    self.online_state.favorite_ids.remove(&id);
                    info!("[在线壁纸] [ID:{}] 已取消收藏", id);
                    // 收藏数据已变化，收藏夹页下次进入时重载
                    self.favorites_state.loaded = false;
                    self.show_notification(
                        self.i18n.t("favorites.removed-toast").to_string(),
                        NotificationType::Info,
                    )
                }
            }
            Err(e) => {
                warn!("[在线壁纸] [ID:{}] 收藏状态切换失败: {}", id, e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.favorite-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
