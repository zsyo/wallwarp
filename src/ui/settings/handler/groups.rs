// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸组管理（设置页：保存当前筛选为组 / 删除组 / 列表加载）

use crate::services::database::{DatabaseManager, GroupFilters, WallpaperGroupsRepository};
use crate::ui::online::ResolutionMode;
use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use serde_json;
use tracing::{error, info};

impl App {
    /// 从数据库加载壁纸组列表（进入壁纸组分类时惰性触发）
    pub(in crate::ui::settings) fn settings_load_groups(&mut self) -> Task<AppMessage> {
        Task::perform(
            async move {
                tokio::task::spawn_blocking(
                    || -> Result<Vec<crate::services::database::WallpaperGroupDB>, String> {
                        let Some(db) = DatabaseManager::try_get() else {
                            return Err("数据库未初始化".to_string());
                        };
                        WallpaperGroupsRepository::new(db.connection().clone()).load_all()
                    },
                )
                .await
                .map_err(|e| e.to_string())?
            },
            |result| SettingsMessage::GroupsLoaded(result).into(),
        )
    }

    /// 壁纸组列表加载完成：更新内存缓存
    pub(in crate::ui::settings) fn settings_groups_loaded(
        &mut self,
        result: Result<Vec<crate::services::database::WallpaperGroupDB>, String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(groups) => {
                self.wallpaper_groups = groups;
                self.wallpaper_groups_loaded = true;
            }
            Err(e) => error!("[壁纸组] [DB] 加载壁纸组列表失败: {}", e),
        }
        Task::none()
    }

    /// 把当前在线筛选快照保存为壁纸组（同名覆盖）
    ///
    /// 快照内容：图源 + 在线筛选字段 + 铺满模式
    pub(in crate::ui::settings) fn settings_save_group_from_current(&mut self) -> Task<AppMessage> {
        let name = self.settings_state.group_name_input.trim().to_string();
        if name.is_empty() {
            let message = self.i18n.t("settings.group-name-empty").to_string();
            return self.show_notification(message, NotificationType::Error);
        }

        // 从在线页状态抓取筛选快照（与配置同步取值一致）
        let filters = GroupFilters {
            category: format!("{:03b}", self.online_state.categories),
            purity: format!("{:03b}", self.online_state.purities),
            sorting: self.online_state.sorting.to_string(),
            color: self.online_state.color.value().to_string(),
            source_color: self.online_state.source_color.value().to_string(),
            pixabay_category: self
                .online_state
                .pixabay_category
                .api_value()
                .unwrap_or_default()
                .to_string(),
            editors_choice: self.online_state.editors_choice,
            top_range: self.online_state.time_range.value().to_string(),
            resolution_mode: match self.online_state.resolution_mode {
                ResolutionMode::All => "all".to_string(),
                ResolutionMode::AtLeast => "atleast".to_string(),
                ResolutionMode::Exactly => "exactly".to_string(),
            },
            atleast_resolution: self
                .online_state
                .atleast_resolution
                .map(|r| r.value().to_string())
                .unwrap_or_default(),
            resolutions: self
                .online_state
                .selected_resolutions
                .iter()
                .map(|r| r.value().to_string())
                .collect::<Vec<_>>()
                .join(","),
            ratios: self.online_state.ratios_to_config_value(),
        };
        let filters_json = serde_json::to_string(&filters).unwrap_or_else(|_| "{}".to_string());

        let source = self.online_state.source.id().to_string();
        let mode = self.config.wallpaper.mode.as_str().to_string();

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    WallpaperGroupsRepository::new(db.connection().clone())
                        .upsert(
                            &name,
                            &source,
                            &filters_json,
                            &mode,
                            chrono::Utc::now().timestamp(),
                        )
                        .map(|_| ())
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |result| SettingsMessage::GroupSaved(result).into(),
        )
    }

    /// 壁纸组保存完成：清空输入、重载列表并提示
    pub(in crate::ui::settings) fn settings_group_saved(
        &mut self,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                info!("[壁纸组] 保存成功");
                self.settings_state.group_name_input.clear();
                let message = self.i18n.t("settings.group-saved").to_string();
                self.show_notification(message, NotificationType::Success)
                    .chain(self.settings_load_groups())
            }
            Err(e) => {
                error!("[壁纸组] [DB] 保存失败: {}", e);
                let message = format!("{}: {}", self.i18n.t("settings.group-save-failed"), e);
                self.show_notification(message, NotificationType::Error)
            }
        }
    }

    /// 删除壁纸组
    pub(in crate::ui::settings) fn settings_delete_group(
        &mut self,
        group_id: i64,
    ) -> Task<AppMessage> {
        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    WallpaperGroupsRepository::new(db.connection().clone()).delete(group_id)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| SettingsMessage::GroupDeleted(group_id, result).into(),
        )
    }

    /// 壁纸组删除完成：重载列表并提示
    pub(in crate::ui::settings) fn settings_group_deleted(
        &mut self,
        group_id: i64,
        result: Result<(), String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(()) => {
                info!("[壁纸组] [ID:{}] 已删除", group_id);
                let message = self.i18n.t("settings.group-deleted").to_string();
                self.show_notification(message, NotificationType::Success)
                    .chain(self.settings_load_groups())
            }
            Err(e) => {
                error!("[壁纸组] [DB] [ID:{}] 删除失败: {}", group_id, e);
                let message = format!("{}: {}", self.i18n.t("settings.group-delete-failed"), e);
                self.show_notification(message, NotificationType::Error)
            }
        }
    }
}
