// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 收藏项移除

use crate::services::database::{DatabaseManager, FavoritesRepository};
use crate::ui::favorites::FavoritesMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{info, warn};

impl App {
    /// 确认移除一条收藏
    pub(in crate::ui::favorites) fn remove_favorite_entry(&mut self) -> Task<AppMessage> {
        let Some(index) = self.favorites_state.remove_target.take() else {
            return Task::none();
        };
        let Some(entry) = self.favorites_state.entries.get(index) else {
            return Task::none();
        };
        let wallhaven_id = entry.fav.wallhaven_id.clone();

        info!("[收藏夹] [DB] 移除收藏: {}", wallhaven_id);

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    FavoritesRepository::new(db.connection().clone()).remove_favorite(&wallhaven_id)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| FavoritesMessage::RemoveFinished { index, result }.into(),
        )
    }

    /// 移除完成：更新列表状态与各页收藏标记
    pub(in crate::ui::favorites) fn favorite_entry_removed(
        &mut self,
        index: usize,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                if index < self.favorites_state.entries.len() {
                    let wallhaven_id =
                        self.favorites_state.entries[index].fav.wallhaven_id.clone();
                    self.favorites_state.entries.remove(index);
                    self.favorites_state.thumbs.remove(index);
                    self.favorites_state
                        .all_entries
                        .retain(|e| e.fav.wallhaven_id != wallhaven_id);
                }

                // 预览模态索引修正
                if self.favorites_state.modal_visible {
                    if self.favorites_state.modal_index == index {
                        self.favorites_state.close_modal();
                    } else if self.favorites_state.modal_index > index {
                        self.favorites_state.modal_index -= 1;
                    }
                }

                // 各页面心形按钮状态刷新
                self.refresh_favorite_markers();

                self.show_notification(
                    self.i18n.t("favorites.removed").to_string(),
                    NotificationType::Success,
                )
            }
            Err(e) => {
                warn!("[收藏夹] [DB] 移除收藏失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("favorites.remove-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
