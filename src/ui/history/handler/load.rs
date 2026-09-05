// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::database::wallpaper_history::HISTORY_MAX_ENTRIES;
use crate::services::database::{DatabaseManager, WallpaperHistoryRepository};
use crate::ui::history::HistoryMessage;
use crate::ui::history::state::HistoryEntry;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use std::path::Path;
use tracing::{debug, info, warn};

/// 数据库历史查询（在阻塞线程中执行）
///
/// 顺带把历史版本的混合路径写法（相对/绝对、\\?\ 前缀）规范化为绝对路径：
/// 同一文件的不同写法会绕过 path 主键去重产生重复记录，此处合并为一条并回写数据库
async fn load_history_from_db() -> Result<Vec<HistoryEntry>, String> {
    tokio::task::spawn_blocking(move || {
        let Some(db) = DatabaseManager::try_get() else {
            return Err("数据库未初始化".to_string());
        };
        let repo = WallpaperHistoryRepository::new(db.connection().clone());
        let rows = repo.load_latest(HISTORY_MAX_ENTRIES)?;

        // 按新→旧遍历，规范化路径并去重（保留最新一条）
        let mut seen = std::collections::HashSet::new();
        let mut entries: Vec<HistoryEntry> = Vec::new();
        for row in rows {
            let canonical = crate::utils::helpers::normalize_path(
                &crate::utils::helpers::get_absolute_path(&row.path),
            );
            if seen.insert(canonical.clone()) {
                // 数据库中仍存旧写法时回写规范路径（删旧写新，保持应用时间不变）
                if canonical != row.path {
                    repo.delete(&row.path)?;
                    repo.upsert(&canonical, row.applied_at)?;
                    debug!("[壁纸历史] [DB] 路径规范化: {} -> {}", row.path, canonical);
                }
                entries.push(HistoryEntry {
                    path: canonical,
                    applied_at: row.applied_at,
                    in_library: false,
                });
            } else {
                repo.delete(&row.path)?;
                debug!("[壁纸历史] [DB] 合并重复记录: {}", row.path);
            }
        }

        Ok(entries)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

impl App {
    /// 加载历史条目并为每个存在的文件生成缩略图任务
    pub(in crate::ui) fn load_history_entries(&mut self) -> Task<AppMessage> {
        self.history_state.loaded = true;

        Task::perform(load_history_from_db(), |result| match result {
            Ok(entries) => HistoryMessage::Loaded(entries).into(),
            Err(e) => {
                warn!("[壁纸历史] [DB] 加载失败: {}", e);
                HistoryMessage::Loaded(Vec::new()).into()
            }
        })
    }

    /// 历史条目加载完成：过滤已不存在的文件，启动缩略图加载
    pub(in crate::ui) fn history_entries_loaded(
        &mut self,
        entries: Vec<HistoryEntry>,
    ) -> Task<AppMessage> {
        // 过滤磁盘上已不存在的文件，并标记文件是否已位于正式壁纸目录
        let absolute_data_dir =
            crate::utils::helpers::get_absolute_path(&self.config.data.data_path);
        let absolute_data_dir = Path::new(&absolute_data_dir);
        let mut existing: Vec<HistoryEntry> = entries
            .into_iter()
            .filter(|e| Path::new(&crate::utils::helpers::get_absolute_path(&e.path)).exists())
            .collect();
        for entry in &mut existing {
            entry.in_library = Path::new(&crate::utils::helpers::get_absolute_path(&entry.path))
                .starts_with(absolute_data_dir);
        }
        info!("[壁纸历史] [DB] 有效条目: {} 条", existing.len());

        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (index, entry) in existing.iter().enumerate() {
            let path = entry.path.clone();
            tasks.push(Task::perform(
                async_task::async_load_single_wallpaper_with_fallback(path, cache_path.clone()),
                move |result| {
                    HistoryMessage::ThumbLoaded {
                        index,
                        wallpaper: result.ok(),
                    }
                    .into()
                },
            ));
        }

        self.history_state.entries = existing;
        self.history_state.thumbs = vec![None; self.history_state.entries.len()];
        self.history_state.wallpapers = vec![None; self.history_state.entries.len()];

        Task::batch(tasks)
    }

    /// 缩略图加载完成（保留元数据供列表与预览信息使用）
    pub(in crate::ui::history) fn history_thumb_loaded(
        &mut self,
        index: usize,
        wallpaper: Option<crate::services::local::Wallpaper>,
    ) -> Task<AppMessage> {
        if index >= self.history_state.entries.len() {
            return Task::none();
        }

        let handle = match &wallpaper {
            Some(w) => w
                .image_handle
                .clone()
                .or_else(|| Some(Handle::from_path(&w.thumbnail_path))),
            None => None,
        };

        self.history_state.wallpapers[index] = wallpaper;
        self.history_state.thumbs[index] = handle;
        Task::none()
    }

    /// 重新应用历史壁纸
    pub(in crate::ui::history) fn apply_history_entry(&mut self, index: usize) -> Task<AppMessage> {
        if let Some(entry) = self.history_state.entries.get(index) {
            let full_path = crate::utils::helpers::get_absolute_path(&entry.path);
            return self.apply_wallpaper(full_path);
        }
        Task::none()
    }
}
