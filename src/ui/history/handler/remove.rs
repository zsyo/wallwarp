// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 历史记录移除/清空（均只操作记录，不删除源文件）

use crate::services::database::{DatabaseManager, WallpaperHistoryRepository};
use crate::ui::history::HistoryMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{info, warn};

impl App {
    /// 确认移除一条历史记录
    pub(in crate::ui::history) fn remove_history_entry(&mut self) -> Task<AppMessage> {
        let Some(index) = self.history_state.remove_target.take() else {
            return Task::none();
        };
        let Some(entry) = self.history_state.entries.get(index) else {
            return Task::none();
        };
        let path = entry.path.clone();

        info!("[壁纸历史] [DB] 移除记录: {}", path);

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    WallpaperHistoryRepository::new(db.connection().clone()).delete(&path)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| HistoryMessage::RemoveFinished { index, result }.into(),
        )
    }

    /// 移除完成：更新列表状态
    pub(in crate::ui::history) fn history_entry_removed(
        &mut self,
        index: usize,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                if index < self.history_state.entries.len() {
                    self.history_state.entries.remove(index);
                    self.history_state.thumbs.remove(index);
                    self.history_state.wallpapers.remove(index);
                }

                // 预览模态索引修正
                if self.history_state.modal_visible {
                    if self.history_state.modal_index == index {
                        self.history_state.close_modal();
                    } else if self.history_state.modal_index > index {
                        self.history_state.modal_index -= 1;
                    }
                }

                self.show_notification(self.i18n.t("history.removed"), NotificationType::Success)
            }
            Err(e) => {
                warn!("[壁纸历史] [DB] 移除失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("history.remove-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }

    /// 确认清空全部历史
    pub(in crate::ui::history) fn clear_history_confirmed(&mut self) -> Task<AppMessage> {
        self.history_state.clear_confirm_visible = false;

        info!("[壁纸历史] [DB] 清空全部历史");

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    WallpaperHistoryRepository::new(db.connection().clone()).clear_all()
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |result| HistoryMessage::ClearFinished(result).into(),
        )
    }

    /// 清空完成：清空列表状态
    pub(in crate::ui::history) fn history_cleared(&mut self, result: Result<(), String>) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                self.history_state.entries.clear();
                self.history_state.thumbs.clear();
                self.history_state.wallpapers.clear();
                self.history_state.close_modal();
                self.history_state.loaded = true;

                self.show_notification(self.i18n.t("history.cleared"), NotificationType::Success)
            }
            Err(e) => {
                warn!("[壁纸历史] [DB] 清空失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("history.clear-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
