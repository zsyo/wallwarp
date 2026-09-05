// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 统一的"设置壁纸"任务构造
//!
//! 各页面(本地/在线/下载/托盘/模态)设置壁纸的异步模板完全一致：
//! 失败弹出错误通知，成功后记入壁纸历史(或由调用方定制)，统一收敛到此处

use crate::services::async_task;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::Path;

impl App {
    /// 异步设置壁纸，成功后将路径记入壁纸历史，失败弹出错误通知
    pub(in crate::ui) fn apply_wallpaper(&mut self, path: String) -> Task<AppMessage> {
        let history_path = path.clone();
        self.set_wallpaper_task(path, move || {
            MainMessage::AddToWallpaperHistory(history_path).into()
        })
    }

    /// 异步复制缓存文件到目标路径后设置壁纸
    ///
    /// 复制在 spawn_blocking 中执行，避免大文件复制阻塞 UI；
    /// 成功后按目标路径记入壁纸历史
    pub(in crate::ui) fn apply_wallpaper_after_copy(
        &mut self,
        source_path: String,
        target_path: &Path,
    ) -> Task<AppMessage> {
        // 壁纸库新增了文件，本地页列表缓存失效
        self.local_state.loaded_data_path = None;
        let full_path = helpers::get_absolute_path(&target_path.to_string_lossy());
        let wallpaper_mode = self.config.wallpaper.mode;
        let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

        Task::perform(
            async_task::async_copy_and_set_wallpaper(
                source_path,
                target_path.to_string_lossy().to_string(),
                wallpaper_mode,
            ),
            move |result| match result {
                Ok(_) => MainMessage::AddToWallpaperHistory(full_path).into(),
                Err(e) => MainMessage::ShowNotification(
                    format!("{}: {}", failed_message, e),
                    NotificationType::Error,
                )
                .into(),
            },
        )
    }

    /// 构造"异步设置壁纸"任务的基础模板
    ///
    /// 失败统一弹出错误通知；成功后发送的消息由 `on_success` 决定
    pub(in crate::ui) fn set_wallpaper_task(
        &mut self,
        path: String,
        on_success: impl FnOnce() -> AppMessage + Send + 'static,
    ) -> Task<AppMessage> {
        let wallpaper_mode = self.config.wallpaper.mode;
        let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

        Task::perform(
            async_task::async_set_wallpaper(path, wallpaper_mode),
            move |result| match result {
                Ok(_) => on_success(),
                Err(e) => MainMessage::ShowNotification(
                    format!("{}: {}", failed_message, e),
                    NotificationType::Error,
                )
                .into(),
            },
        )
    }
}
