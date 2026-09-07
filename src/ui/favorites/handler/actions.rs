// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏项操作：移动分组 / 分组新建与删除

use crate::services::database::{DatabaseManager, FavoritesRepository};
use crate::ui::favorites::FavoritesMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{info, warn};

impl App {
    /// 移动收藏项到指定分组（None = 移出分组）
    pub(in crate::ui::favorites) fn move_favorite_to_group(
        &mut self,
        index: usize,
        group_id: Option<i64>,
    ) -> Task<AppMessage> {
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };
        let wallhaven_id = entry.fav.wallhaven_id.clone();

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    let repo = FavoritesRepository::new(db.connection().clone());
                    // 移动分组复用 upsert：先查出完整行再改 group_id 写回
                    let fav = repo
                        .load_all_favorites()?
                        .into_iter()
                        .find(|f| f.wallhaven_id == wallhaven_id)
                        .ok_or_else(|| "收藏项不存在".to_string())?;
                    repo.upsert_favorite(&crate::services::database::FavoriteDB {
                        group_id,
                        ..fav
                    })
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| {
                FavoritesMessage::MoveFinished {
                    index,
                    group_id,
                    result,
                }
                .into()
            },
        )
    }

    /// 移动完成：更新内存状态
    pub(in crate::ui::favorites) fn favorite_moved(
        &mut self,
        index: usize,
        group_id: Option<i64>,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                // all_entries 与 entries 是同一数据的两份快照，同步更新
                if let Some(entry) = self
                    .favorites_state
                    .entries
                    .get(index)
                    .map(|e| e.fav.wallhaven_id.clone())
                {
                    if let Some(src) = self
                        .favorites_state
                        .all_entries
                        .iter_mut()
                        .find(|e| e.fav.wallhaven_id == entry)
                    {
                        src.fav.group_id = group_id;
                    }
                    if let Some(src) = self
                        .favorites_state
                        .entries
                        .iter_mut()
                        .find(|e| e.fav.wallhaven_id == entry)
                    {
                        src.fav.group_id = group_id;
                    }
                }

                info!("[收藏夹] [DB] 移动收藏项到分组 {:?} 完成", group_id);
                self.show_notification(
                    self.i18n.t("favorites.moved").to_string(),
                    NotificationType::Success,
                )
            }
            Err(e) => {
                warn!("[收藏夹] [DB] 移动收藏项失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.move-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }

    /// 确认新建分组
    pub(in crate::ui::favorites) fn create_favorite_group(&mut self) -> Task<AppMessage> {
        let name = self.favorites_state.create_group_name.trim().to_string();
        if name.is_empty() {
            return Task::none();
        }
        self.favorites_state.create_group_visible = false;

        info!("[收藏夹] [DB] 新建分组: {}", name);

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<i64, String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    FavoritesRepository::new(db.connection().clone())
                        .insert_group(&name, chrono::Utc::now().timestamp())
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |result| FavoritesMessage::CreateGroupFinished(result).into(),
        )
    }

    /// 新建分组完成：刷新分组列表
    pub(in crate::ui::favorites) fn favorite_group_created(
        &mut self,
        result: Result<i64, String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(_) => {
                self.show_notification(
                    self.i18n.t("favorites.group-created").to_string(),
                    NotificationType::Success,
                )
            }
            Err(e) => {
                warn!("[收藏夹] [DB] 新建分组失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.group-create-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }

    /// 确认删除当前筛选分组（组内收藏项回落未分组）
    pub(in crate::ui::favorites) fn delete_favorite_group(&mut self) -> Task<AppMessage> {
        self.favorites_state.delete_group_confirm_visible = false;
        let Some(group_id) = self.current_filter_group() else {
            return Task::none();
        };

        info!("[收藏夹] [DB] 删除分组 ID:{}", group_id);

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    FavoritesRepository::new(db.connection().clone()).delete_group(group_id)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |result| FavoritesMessage::DeleteGroupFinished(result).into(),
        )
    }

    /// 删除分组完成：更新分组列表与收藏项归属
    pub(in crate::ui::favorites) fn favorite_group_deleted(
        &mut self,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                let Some(group_id) = self.current_filter_group() else {
                    return Task::none();
                };
                // 组内收藏项回落未分组（内存同步）
                for entry in &mut self.favorites_state.all_entries {
                    if entry.fav.group_id == Some(group_id) {
                        entry.fav.group_id = None;
                    }
                }
                self.favorites_state.groups.retain(|g| g.id != group_id);
                self.favorites_state.group_filter =
                    crate::ui::favorites::message::GroupFilter::All;
                self.favorites_state.apply_filter();

                self.show_notification(
                    self.i18n.t("favorites.group-deleted").to_string(),
                    NotificationType::Success,
                )
            }
            Err(e) => {
                warn!("[收藏夹] [DB] 删除分组失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.group-delete-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
