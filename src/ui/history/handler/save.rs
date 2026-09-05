// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 历史条目保存到正式壁纸目录（针对缓存目录中的壁纸，如定时切换/在线切换产生的文件）

use crate::services::database::{DatabaseManager, WallpaperHistoryRepository};
use crate::ui::history::HistoryMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

impl App {
    /// 将该条目的图片文件复制到正式壁纸目录，并把历史记录改指向库内文件
    pub(in crate::ui::history) fn save_history_entry_to_library(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        let Some(entry) = self.history_state.entries.get(index) else {
            return Task::none();
        };
        if entry.in_library {
            return Task::none();
        }

        let source = crate::utils::helpers::get_absolute_path(&entry.path);
        let file_name = Path::new(&source)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("wallpaper")
            .to_string();
        let data_dir = crate::utils::helpers::get_absolute_path(&self.config.data.data_path);
        let target = crate::utils::helpers::normalize_path(
            &Path::new(&data_dir).join(file_name).to_string_lossy(),
        );
        let applied_at = entry.applied_at;

        // 提前获取翻译文本，避免线程安全问题
        let file_missing_message = self.i18n.t("history.file-missing").to_string();
        let failed_message = self.i18n.t("history.save-failed").to_string();

        info!("[壁纸历史] [保存] {} -> {}", source, target);

        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<String, String> {
                    if !Path::new(&source).exists() {
                        return Err(format!("{}: {}", file_missing_message, source));
                    }

                    // 目标已存在视为已在库中，跳过复制（不覆盖既有文件）
                    if Path::new(&target).exists() {
                        info!("[壁纸历史] [保存] 目标已存在，跳过复制: {}", target);
                    } else {
                        if let Some(parent) = Path::new(&target).parent() {
                            fs::create_dir_all(parent)
                                .map_err(|e| format!("{}: {}", failed_message, e))?;
                        }
                        fs::copy(&source, &target)
                            .map_err(|e| format!("{}: {}", failed_message, e))?;
                        info!("[壁纸历史] [保存] 复制完成: {}", target);
                    }

                    // 历史记录改指向库内文件（保持原应用时间）
                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    let repo = WallpaperHistoryRepository::new(db.connection().clone());
                    repo.delete(&source)?;
                    repo.upsert(&target, applied_at)?;
                    Ok(target)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            move |result| HistoryMessage::SaveFinished { index, result }.into(),
        )
    }

    /// 保存完成：更新条目指向与内存历史，并刷新本地页与菜单状态
    pub(in crate::ui::history) fn history_entry_saved(
        &mut self,
        index: usize,
        result: Result<String, String>,
    ) -> Task<AppMessage> {
        match result {
            Ok(new_path) => {
                if index < self.history_state.entries.len() {
                    let old_path = self.history_state.entries[index].path.clone();
                    self.history_state.entries[index].path = new_path.clone();
                    self.history_state.entries[index].in_library = true;
                    if let Some(w) = self.history_state.wallpapers[index].as_mut() {
                        w.path = new_path.clone();
                    }

                    // 内存历史同步改指向（若为当前壁纸，托盘/悬浮球"保存当前壁纸"的可用状态随之变化）
                    if let Some(pos) = self.wallpaper_history.iter().position(|p| *p == old_path) {
                        self.wallpaper_history[pos] = new_path;
                    }
                    self.update_menu_items();
                }

                // 壁纸库新增了文件，本地页列表缓存失效
                self.local_state.loaded_data_path = None;

                self.show_notification(self.i18n.t("history.saved"), NotificationType::Success)
            }
            Err(e) => {
                warn!("[壁纸历史] [保存] 失败: {}", e);
                self.show_notification(
                    format!("{}: {}", self.i18n.t("history.save-failed"), e),
                    NotificationType::Error,
                )
            }
        }
    }
}
