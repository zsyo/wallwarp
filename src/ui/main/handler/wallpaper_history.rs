// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use std::path::Path;
use tracing::info;

impl App {
    /// 检查当前壁纸是否可以保存到库（即当前壁纸不存在于data_path目录中）
    pub(in crate::ui) fn can_save_current_wallpaper(&self) -> bool {
        if self.wallpaper_history.is_empty() {
            return false;
        }

        let current_wallpaper = self.wallpaper_history.last().unwrap();

        // 获取当前壁纸的绝对路径（标准化去除\\?\前缀）
        let normalized_wallpaper = crate::utils::helpers::normalize_path(current_wallpaper);
        let absolute_wallpaper = crate::utils::helpers::get_absolute_path(&normalized_wallpaper);
        let wallpaper_path = Path::new(&absolute_wallpaper);

        // 获取data_path的绝对路径
        let data_path = &self.config.data.data_path;
        let absolute_data_path = crate::utils::helpers::get_absolute_path(data_path);
        let data_dir = Path::new(&absolute_data_path);

        // 判断当前壁纸是否不在data_path目录中
        !wallpaper_path.starts_with(data_dir)
    }

    pub(in crate::ui::main) fn add_to_wallpaper_history(
        &mut self,
        path: String,
    ) -> Task<AppMessage> {
        // 统一以"绝对路径 + 去除 \\?\ 前缀"的规范形态入库：
        // 同一文件若以不同写法（相对/绝对）记录会绕过主键去重，产生重复历史
        let path =
            crate::utils::helpers::normalize_path(&crate::utils::helpers::get_absolute_path(&path));

        // 检查历史记录中是否已存在该路径，如果存在则先移除
        if let Some(pos) = self.wallpaper_history.iter().position(|p| p == &path) {
            self.wallpaper_history.remove(pos);
        }

        // 记录路径用于日志输出
        let path_for_log = path.clone();

        // 添加到历史记录末尾
        self.wallpaper_history.push(path);

        // 限制历史记录最多50条
        if self.wallpaper_history.len() > 50 {
            self.wallpaper_history.remove(0);
        }

        info!(
            "[壁纸历史] 添加记录: {}, 当前记录数: {}",
            path_for_log,
            self.wallpaper_history.len()
        );

        // 如果开启了定时切换壁纸，重置下次切换时间
        self.reset_auto_change_next_execute_time();

        // 更新托盘与悬浮球菜单项的启用状态
        self.update_menu_items();

        // 历史页缓存失效
        self.history_state.invalidate();

        // 正处于历史页时自动重新加载列表（新记录置顶），避免列表清空后不恢复
        let reload_history_page = if self.active_page == crate::ui::ActivePage::WallpaperHistory {
            Task::done(crate::ui::history::HistoryMessage::Load.into())
        } else {
            Task::none()
        };

        // 持久化历史记录（失败仅告警，不影响主流程）
        let db_path = path_for_log.clone();
        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || -> Result<(), String> {
                    use crate::services::database::wallpaper_history::HISTORY_MAX_ENTRIES;
                    use crate::services::database::{DatabaseManager, WallpaperHistoryRepository};

                    let Some(db) = DatabaseManager::try_get() else {
                        return Err("数据库未初始化".to_string());
                    };
                    let repo = WallpaperHistoryRepository::new(db.connection().clone());
                    repo.upsert(&db_path, chrono::Utc::now().timestamp())?;
                    repo.prune(HISTORY_MAX_ENTRIES)
                })
                .await
                .map_err(|e| e.to_string())?
            },
            |result| {
                if let Err(e) = result {
                    tracing::warn!("[壁纸历史] [DB] 写入失败: {}", e);
                }
                AppMessage::None.into()
            },
        )
        .chain(reload_history_page)
    }
}
