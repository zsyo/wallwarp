// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use super::AutoChangeMessage;

impl App {
    /// 处理定时切换壁纸相关消息
    pub fn handle_auto_change_message(&mut self, msg: AutoChangeMessage) -> iced::Task<AppMessage> {
        match msg {
            AutoChangeMessage::StartAutoChange => self.handle_start_auto_change(),
            AutoChangeMessage::StopAutoChange => self.handle_stop_auto_change(),
            AutoChangeMessage::AutoChangeTick => self.handle_auto_change_tick(),
            AutoChangeMessage::GetSupportedImagesSuccess(paths) => {
                self.handle_get_supported_images_success(paths)
            }
            AutoChangeMessage::GetSupportedImagesFailed(error) => self.handle_get_supported_images_failed(error),
            AutoChangeMessage::SetRandomWallpaperSuccess(path) => self.handle_set_random_wallpaper_success(path),
            AutoChangeMessage::SetRandomWallpaperFailed(error) => {
                self.handle_set_random_wallpaper_failed(error)
            }
        }
    }

    /// 启动定时切换壁纸
    fn handle_start_auto_change(&mut self) -> iced::Task<AppMessage> {
        // 检查定时切换间隔是否为关闭状态
        if matches!(
            self.config.wallpaper.auto_change_interval,
            crate::utils::config::WallpaperAutoChangeInterval::Off
        ) {
            return iced::Task::none();
        }

        // 启动定时切换
        self.auto_change_state.auto_change_enabled = true;
        self.auto_change_state.auto_change_timer = Some(std::time::Instant::now());
        self.auto_change_state.auto_change_last_time = Some(std::time::Instant::now());

        // 根据切换模式启动不同的逻辑
        match self.config.wallpaper.auto_change_mode {
            crate::utils::config::WallpaperAutoChangeMode::Local => {
                // 本地模式：获取支持的图片文件列表
                let data_path = self.config.data.data_path.clone();
                iced::Task::perform(
                    crate::ui::async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesSuccess(paths))
                        }
                        Err(e) => AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesFailed(
                            e.to_string(),
                        )),
                    },
                )
            }
            crate::utils::config::WallpaperAutoChangeMode::Online => {
                // 在线模式：启动在线壁纸切换
                iced::Task::none()
            }
        }
    }

    /// 停止定时切换壁纸
    fn handle_stop_auto_change(&mut self) -> iced::Task<AppMessage> {
        self.auto_change_state.auto_change_enabled = false;
        self.auto_change_state.auto_change_timer = None;
        self.auto_change_state.auto_change_last_time = None;
        iced::Task::none()
    }

    /// 处理定时切换壁纸的定时器事件
    fn handle_auto_change_tick(&mut self) -> iced::Task<AppMessage> {
        if !self.auto_change_state.auto_change_enabled {
            return iced::Task::none();
        }

        // 2. 更新最后一次执行时间（用于 UI 显示或其他逻辑参考）
        self.auto_change_state.auto_change_last_time = Some(std::time::Instant::now());

        // 3. 记录日志 (现在只有在真正执行时才会打印)
        let next_interval = self.config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30);
        let next_time_label = chrono::Local::now() + chrono::Duration::minutes(next_interval as i64);
        tracing::info!(
            "[定时切换] 执行壁纸切换。模式: {:?}, 下次预计时间: {}",
            self.config.wallpaper.auto_change_mode,
            next_time_label.format("%H:%M:%S")
        );

        // 4. 根据模式直接执行切换任务
        match self.config.wallpaper.auto_change_mode {
            crate::utils::config::WallpaperAutoChangeMode::Local => {
                let data_path = self.config.data.data_path.clone();
                iced::Task::perform(
                    crate::ui::async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            if paths.is_empty() {
                                AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesFailed(
                                    "没有找到支持的壁纸文件".to_string(),
                                ))
                            } else {
                                AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesSuccess(paths))
                            }
                        }
                        Err(e) => AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesFailed(
                            e.to_string(),
                        )),
                    },
                )
            }
            crate::utils::config::WallpaperAutoChangeMode::Online => {
                let config = self.config.clone();
                let auto_change_running = self.auto_change_running.clone();
                iced::Task::perform(
                    crate::ui::async_tasks::async_set_random_online_wallpaper(config, auto_change_running),
                    |result| match result {
                        Ok(path) => AppMessage::AutoChange(AutoChangeMessage::SetRandomWallpaperSuccess(path)),
                        Err(e) => AppMessage::AutoChange(AutoChangeMessage::SetRandomWallpaperFailed(
                            e.to_string(),
                        )),
                    },
                )
            }
        }
    }

    /// 处理获取支持的图片文件列表成功
    fn handle_get_supported_images_success(&mut self, paths: Vec<String>) -> iced::Task<AppMessage> {
        if !paths.is_empty() {
            // 记录找到的壁纸数量
            tracing::info!("[定时切换] [获取] 找到 {} 张壁纸", paths.len());

            // 获取成功，立即设置一张随机壁纸
            let wallpaper_mode = self.config.wallpaper.mode;

            iced::Task::perform(
                crate::ui::async_tasks::async_set_random_wallpaper(paths, wallpaper_mode),
                |result| match result {
                    Ok(path) => AppMessage::AutoChange(AutoChangeMessage::SetRandomWallpaperSuccess(path)),
                    Err(e) => AppMessage::AutoChange(AutoChangeMessage::SetRandomWallpaperFailed(e.to_string())),
                },
            )
        } else {
            // 没有找到支持的壁纸
            tracing::warn!("[定时切换] [获取] 没有找到支持的壁纸文件");
            let error_message = self.i18n.t("local-list.no-valid-wallpapers").to_string();
            iced::Task::done(AppMessage::ShowNotification(
                error_message,
                crate::ui::NotificationType::Error,
            ))
        }
    }

    /// 处理获取支持的图片文件列表失败
    fn handle_get_supported_images_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        tracing::error!("[定时切换] [失败] 获取壁纸列表失败: {}", error);
        self.auto_change_state.auto_change_enabled = false;
        let error_message = format!("获取壁纸列表失败: {}", error);
        iced::Task::done(AppMessage::ShowNotification(
            error_message,
            crate::ui::NotificationType::Error,
        ))
    }

    /// 处理随机设置壁纸成功
    fn handle_set_random_wallpaper_success(&mut self, path: String) -> iced::Task<AppMessage> {
        tracing::info!("[定时切换] [成功] 已设置壁纸: {}", path);

        // 将壁纸路径添加到历史记录
        // let success_message = format!("已设置壁纸: {}", path.clone());
        iced::Task::batch(vec![
            iced::Task::done(AppMessage::AddToWallpaperHistory(path)),
            // iced::Task::done(AppMessage::ShowNotification(
            //     success_message,
            //     super::NotificationType::Success,
            // )),
        ])
    }

    /// 处理随机设置壁纸失败
    fn handle_set_random_wallpaper_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        tracing::error!("[定时切换] [失败] 设置壁纸失败: {}", error);
        let error_message = format!("设置壁纸失败: {}", error);
        iced::Task::done(AppMessage::ShowNotification(
            error_message,
            crate::ui::NotificationType::Error,
        ))
    }
}