// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

impl App {
    pub(in crate::ui::main) fn tray_save_current_wallpaper(&mut self) -> Task<AppMessage> {
        // 提前获取翻译文本
        let no_wallpaper_message = self
            .i18n
            .t("local-list.no-wallpaper-to-save")
            .to_string();
        let not_in_cache_message = self
            .i18n
            .t("local-list.wallpaper-not-in-cache")
            .to_string();
        let success_message = self.i18n.t("local-list.save-success").to_string();
        let failed_message = self.i18n.t("local-list.save-failed").to_string();
        let file_not_found_message = self.i18n.t("local-list.file-not-found").to_string();
        let target_file_exists_message = self
            .i18n
            .t("local-list.target-file-exists")
            .to_string();

        // 检查壁纸历史记录是否为空
        if self.wallpaper_history.is_empty() {
            warn!("[托盘菜单] 壁纸历史记录为空，无法保存当前壁纸");
            return Task::done(
                MainMessage::ShowNotification(no_wallpaper_message, NotificationType::Error)
                    .into(),
            );
        }

        // 获取当前壁纸路径（历史记录中的最后一条）
        let current_wallpaper = self.wallpaper_history.last().unwrap().clone();
        let normalized_wallpaper = crate::utils::helpers::normalize_path(&current_wallpaper);

        // 获取当前壁纸的绝对路径
        let absolute_wallpaper = crate::utils::helpers::get_absolute_path(&normalized_wallpaper);
        let wallpaper_path = Path::new(&absolute_wallpaper);

        // 获取data_path的绝对路径
        let data_path = &self.config.data.data_path;
        let absolute_data_path = crate::utils::helpers::get_absolute_path(data_path);
        let data_dir = Path::new(&absolute_data_path);

        // 检查当前壁纸是否已经在data_path目录中
        if wallpaper_path.starts_with(data_dir) {
            info!(
                "[托盘菜单] 当前壁纸已在data_path目录中: {}",
                normalized_wallpaper
            );
            return Task::done(
                MainMessage::ShowNotification(not_in_cache_message, NotificationType::Info).into(),
            );
        }

        // 获取目标路径（data_path）
        let data_path = self.config.data.data_path.clone();
        let wallpaper_file_name = wallpaper_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("wallpaper")
            .to_string();
        let target_path = Path::new(&data_path).join(&wallpaper_file_name);

        // 执行复制操作
        let source_path = current_wallpaper.clone();
        let target_path_str = target_path.to_string_lossy().to_string();
        let failed_message_clone = failed_message.clone();

        Task::perform(
            async move {
                // 检查源文件是否存在
                if !Path::new(&source_path).exists() {
                    return Err(file_not_found_message);
                }

                // 确保目标目录存在
                if let Some(parent) = Path::new(&target_path_str).parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("{}: {}", failed_message_clone, e))?;
                }

                // 检查目标文件是否已存在
                if Path::new(&target_path_str).exists() {
                    return Err(target_file_exists_message);
                }

                // 复制文件
                fs::copy(&source_path, &target_path_str)
                    .map_err(|e| format!("{}: {}", failed_message_clone, e))?;
                info!(
                    "[托盘菜单] 壁纸已从{}复制到{}",
                    crate::utils::helpers::normalize_path(&source_path), target_path_str
                );

                Ok(())
            },
            move |result| match result {
                Ok(_) => {
                    MainMessage::ShowNotification(success_message, NotificationType::Success).into()
                }
                Err(e) => {
                    MainMessage::ShowNotification(
                        format!("{}: {}", failed_message, e),
                        NotificationType::Error,
                    )
                    .into()
                }
            },
        )
    }
}