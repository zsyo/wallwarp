// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use super::common;
use super::local::LocalMessage;
use tracing::error;

impl App {
    /// 处理本地壁纸相关消息
    pub fn handle_local_message(&mut self, msg: LocalMessage) -> iced::Task<AppMessage> {
        match msg {
            LocalMessage::LoadWallpapers => self.handle_load_local_wallpapers(),
            LocalMessage::LoadWallpapersSuccess(paths) => self.handle_load_local_wallpapers_success(paths),
            LocalMessage::LoadWallpapersFailed(error) => self.handle_load_local_wallpapers_failed(error),
            LocalMessage::LoadPage => self.handle_load_local_page(),
            LocalMessage::LoadPageSuccess(wallpapers_with_idx) => {
                self.handle_load_local_page_success(wallpapers_with_idx)
            }
            LocalMessage::LoadPageFailed(error) => self.handle_load_local_page_failed(error),
            LocalMessage::WallpaperSelected(wallpaper) => self.handle_local_wallpaper_selected(wallpaper),
            LocalMessage::ShowModal(index) => self.handle_show_local_modal(index),
            LocalMessage::ModalImageLoaded(handle) => self.handle_local_modal_image_loaded(handle),
            LocalMessage::CloseModal => self.handle_close_local_modal(),
            LocalMessage::NextImage => self.handle_next_local_image(),
            LocalMessage::PreviousImage => self.handle_previous_local_image(),
            LocalMessage::ScrollToBottom => self.handle_scroll_to_bottom(),
            LocalMessage::CheckAndLoadNextPage => self.handle_check_and_load_next_page(),
            LocalMessage::ViewInFolder(index) => self.handle_view_in_folder(index),
            LocalMessage::ShowDeleteConfirm(index) => self.handle_show_delete_confirm(index),
            LocalMessage::CloseDeleteConfirm => self.handle_close_delete_confirm(),
            LocalMessage::ConfirmDelete(index) => self.handle_confirm_delete(index),
            LocalMessage::SetWallpaper(index) => self.handle_set_wallpaper(index),
            // 定时切换壁纸相关消息
            LocalMessage::StartAutoChange => self.handle_start_auto_change(),
            LocalMessage::StopAutoChange => self.handle_stop_auto_change(),
            LocalMessage::AutoChangeTick => self.handle_auto_change_tick(),
            LocalMessage::GetSupportedImagesSuccess(paths) => self.handle_get_supported_images_success(paths),
            LocalMessage::GetSupportedImagesFailed(error) => self.handle_get_supported_images_failed(error),
            LocalMessage::SetRandomWallpaperSuccess(path) => self.handle_set_random_wallpaper_success(path),
            LocalMessage::SetRandomWallpaperFailed(error) => self.handle_set_random_wallpaper_failed(error),
        }
    }

    fn handle_load_local_wallpapers(&mut self) -> iced::Task<AppMessage> {
        let data_path = self.config.data.data_path.clone();
        iced::Task::perform(
            super::async_tasks::async_load_wallpaper_paths(data_path),
            |result| match result {
                Ok(paths) => AppMessage::Local(super::local::LocalMessage::LoadWallpapersSuccess(paths)),
                Err(e) => AppMessage::Local(super::local::LocalMessage::LoadWallpapersFailed(e.to_string())),
            },
        )
    }

    fn handle_load_local_wallpapers_success(&mut self, paths: Vec<String>) -> iced::Task<AppMessage> {
        // 更新本地状态，初始化壁纸加载状态列表
        self.local_state.all_paths = paths;
        self.local_state.total_count = self.local_state.all_paths.len();

        // 初始化壁纸状态为Loading，并加载第一页
        let page_end = std::cmp::min(self.local_state.page_size, self.local_state.total_count);
        self.local_state.wallpapers = vec![super::local::WallpaperLoadStatus::Loading; page_end];

        // 触发第一页加载
        iced::Task::perform(async {}, |_| AppMessage::Local(super::local::LocalMessage::LoadPage))
    }

    fn handle_load_local_wallpapers_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        // 由于现在使用WallpaperLoadStatus处理单个壁纸的错误，整体错误处理已不再需要
        // 可以考虑显示一个通知或者在UI的其他地方显示错误
        error!("[本地壁纸] 加载列表失败: {}", error);
        iced::Task::none()
    }

    fn handle_load_local_page(&mut self) -> iced::Task<AppMessage> {
        if self.local_state.current_page * self.local_state.page_size >= self.local_state.total_count {
            // 没有更多壁纸可加载
            self.local_state.loading_page = false;
            return iced::Task::none();
        }

        // 设置加载状态
        self.local_state.loading_page = true;

        // 获取当前页需要加载的壁纸路径
        let start_idx = self.local_state.current_page * self.local_state.page_size;
        let end_idx = std::cmp::min(start_idx + self.local_state.page_size, self.local_state.total_count);

        // 为每个壁纸启动单独的异步任务
        let mut tasks = Vec::new();
        for (i, path) in self.local_state.all_paths[start_idx..end_idx].iter().enumerate() {
            let path = path.clone();
            let cache_path = self.config.data.cache_path.clone();
            let absolute_idx = start_idx + i;

            tasks.push(iced::Task::perform(
                super::async_tasks::async_load_single_wallpaper_with_fallback(path.clone(), cache_path),
                move |result| match result {
                    Ok(wallpaper) => AppMessage::Local(super::local::LocalMessage::LoadPageSuccess(vec![(
                        absolute_idx,
                        wallpaper,
                    )])),
                    Err(_) => AppMessage::Local(super::local::LocalMessage::LoadPageSuccess(vec![(
                        absolute_idx,
                        crate::services::local::Wallpaper::new(path, "加载失败".to_string(), 0, 0, 0),
                    )])), // 创建失败状态
                },
            ));
        }

        // 更新当前页面的壁纸状态为加载中
        let page_start = self.local_state.current_page * self.local_state.page_size;
        let page_end = std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

        if self.local_state.current_page == 0 {
            // 第一页：初始化wallpapers数组
            self.local_state.wallpapers = vec![super::local::WallpaperLoadStatus::Loading; page_end];
        } else {
            // 后续页面：扩展wallpapers数组
            for _ in 0..(page_end - self.local_state.wallpapers.len()) {
                self.local_state
                    .wallpapers
                    .push(super::local::WallpaperLoadStatus::Loading);
            }
        }

        self.local_state.current_page += 1;
        iced::Task::batch(tasks)
    }

    fn handle_load_local_page_success(
        &mut self,
        wallpapers_with_idx: Vec<(usize, crate::services::local::Wallpaper)>,
    ) -> iced::Task<AppMessage> {
        // 为每个加载完成的壁纸更新状态
        for (idx, wallpaper) in wallpapers_with_idx {
            if idx < self.local_state.wallpapers.len() {
                self.local_state.wallpapers[idx] = super::local::WallpaperLoadStatus::Loaded(wallpaper);
            }
        }

        // 检查是否所有壁纸都已加载完成，如果是则更新loading_page状态
        let page_start = (self.local_state.current_page - 1) * self.local_state.page_size; // 上一页的起始位置
        let page_end = std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

        let all_loaded = (page_start..page_end).all(|i| {
            i < self.local_state.wallpapers.len()
                && matches!(
                    self.local_state.wallpapers[i],
                    super::local::WallpaperLoadStatus::Loaded(_)
                )
        });

        if all_loaded {
            self.local_state.loading_page = false;

            // 添加检查是否需要自动加载下一页的任务
            let check_task = iced::Task::perform(async {}, |_| {
                AppMessage::Local(super::local::LocalMessage::CheckAndLoadNextPage)
            });
            return check_task;
        }

        iced::Task::none()
    }

    fn handle_load_local_page_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        // 更新加载状态
        self.local_state.loading_page = false;
        // 由于现在使用WallpaperLoadStatus处理单个壁纸的错误，整体错误处理已不再需要
        error!("[本地壁纸] 加载页面失败: {}", error);
        iced::Task::none()
    }

    fn handle_local_wallpaper_selected(
        &mut self,
        _wallpaper: crate::services::local::Wallpaper,
    ) -> iced::Task<AppMessage> {
        // 处理壁纸选择
        iced::Task::none()
    }

    fn handle_show_local_modal(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 检查要显示的图片是否为失败状态
        if let Some(wallpaper_status) = self.local_state.wallpapers.get(index) {
            if let super::local::WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                if wallpaper.name == "加载失败" {
                    // 如果是失败的图片，不显示模态窗口
                    return iced::Task::none();
                }
            }
        }

        // 显示模态窗口，设置当前图片索引
        self.local_state.current_image_index = index;
        self.local_state.modal_visible = true;

        // 清除之前的图片数据
        self.local_state.modal_image_handle = None;

        // 异步加载图片数据
        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
            return iced::Task::perform(
                async move {
                    // 异步加载图片数据
                    let image_handle = iced::widget::image::Handle::from_path(&path);
                    // 等待一小段时间确保图片数据已加载
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    image_handle
                },
                |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
            );
        }

        iced::Task::none()
    }

    fn handle_local_modal_image_loaded(&mut self, handle: iced::widget::image::Handle) -> iced::Task<AppMessage> {
        // 模态窗口图片加载完成，保存图片数据
        self.local_state.modal_image_handle = Some(handle);
        iced::Task::none()
    }

    fn handle_close_local_modal(&mut self) -> iced::Task<AppMessage> {
        // 关闭模态窗口
        self.local_state.modal_visible = false;
        iced::Task::none()
    }

    fn handle_next_local_image(&mut self) -> iced::Task<AppMessage> {
        // 显示下一张图片，跳过加载失败的图片
        if let Some(next_index) = self
            .local_state
            .find_next_valid_image_index(self.local_state.current_image_index, 1)
        {
            self.local_state.current_image_index = next_index;

            // 清除之前的图片数据
            self.local_state.modal_image_handle = None;

            // 异步加载图片数据
            if let Some(path) = self.local_state.all_paths.get(next_index).cloned() {
                return iced::Task::perform(
                    async move {
                        // 异步加载图片数据
                        let image_handle = iced::widget::image::Handle::from_path(&path);
                        // 等待一小段时间确保图片数据已加载
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        image_handle
                    },
                    |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
                );
            }
        }

        iced::Task::none()
    }

    fn handle_previous_local_image(&mut self) -> iced::Task<AppMessage> {
        // 显示上一张图片，跳过加载失败的图片
        if let Some(prev_index) = self
            .local_state
            .find_next_valid_image_index(self.local_state.current_image_index, -1)
        {
            self.local_state.current_image_index = prev_index;

            // 清除之前的图片数据
            self.local_state.modal_image_handle = None;

            // 异步加载图片数据
            if let Some(path) = self.local_state.all_paths.get(prev_index).cloned() {
                return iced::Task::perform(
                    async move {
                        // 异步加载图片数据
                        let image_handle = iced::widget::image::Handle::from_path(&path);
                        // 等待一小段时间确保图片数据已加载
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        image_handle
                    },
                    |handle| AppMessage::Local(super::local::LocalMessage::ModalImageLoaded(handle)),
                );
            }
        }

        iced::Task::none()
    }

    fn handle_scroll_to_bottom(&mut self) -> iced::Task<AppMessage> {
        // 滚动到底部，如果还有更多壁纸则加载下一页
        if self.local_state.current_page * self.local_state.page_size < self.local_state.total_count
            && !self.local_state.loading_page
        {
            return iced::Task::perform(async {}, |_| AppMessage::Local(super::local::LocalMessage::LoadPage));
        }

        iced::Task::none()
    }

    fn handle_check_and_load_next_page(&mut self) -> iced::Task<AppMessage> {
        // 检查是否需要自动加载下一页
        // 条件：还有更多壁纸，且当前没有正在加载
        if self.local_state.current_page * self.local_state.page_size < self.local_state.total_count
            && !self.local_state.loading_page
        {
            // 计算每行可以显示多少张图
            let available_width =
                (self.current_window_width as f32 - crate::ui::style::IMAGE_SPACING).max(crate::ui::style::IMAGE_WIDTH);
            let unit_width = crate::ui::style::IMAGE_WIDTH + crate::ui::style::IMAGE_SPACING;
            let items_per_row = (available_width / unit_width).floor() as usize;
            let items_per_row = items_per_row.max(1);

            // 计算实际行数
            let num_wallpapers = self.local_state.wallpapers.len();
            let num_rows = (num_wallpapers + items_per_row - 1) / items_per_row; // 向上取整

            // 估算内容高度：行数 * (每张图高度 + 间距)
            let estimated_content_height =
                num_rows as f32 * (crate::ui::style::IMAGE_HEIGHT + crate::ui::style::IMAGE_SPACING);

            // 如果估算的内容高度小于窗口高度，说明没有滚动条，需要加载下一页
            if estimated_content_height < self.current_window_height as f32 {
                return iced::Task::perform(async {}, |_| AppMessage::Local(super::local::LocalMessage::LoadPage));
            }
        }

        iced::Task::none()
    }

    fn handle_view_in_folder(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 查看文件夹并选中文件
        if let Some(path) = self.local_state.all_paths.get(index) {
            let full_path = common::get_absolute_path(path);
            crate::utils::helpers::open_file_in_explorer(&full_path);
        }

        iced::Task::none()
    }

    fn handle_show_delete_confirm(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 显示删除确认对话框
        self.local_state.delete_confirm_visible = true;
        self.local_state.delete_target_index = Some(index);
        iced::Task::none()
    }

    fn handle_close_delete_confirm(&mut self) -> iced::Task<AppMessage> {
        // 关闭删除确认对话框
        self.local_state.delete_confirm_visible = false;
        self.local_state.delete_target_index = None;
        iced::Task::none()
    }

    fn handle_confirm_delete(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 确认删除壁纸
        self.local_state.delete_confirm_visible = false;
        self.local_state.delete_target_index = None;

        // 删除壁纸
        if let Some(path) = self.local_state.all_paths.get(index) {
            let full_path = common::get_absolute_path(path);

            // 尝试删除文件
            match std::fs::remove_file(&full_path) {
                Ok(_) => {
                    // 删除成功，从列表中移除
                    self.local_state.all_paths.remove(index);
                    self.local_state.wallpapers.remove(index);
                    self.local_state.total_count -= 1;

                    // 如果删除的是当前显示的图片，关闭模态窗口
                    if self.local_state.modal_visible && self.local_state.current_image_index == index {
                        self.local_state.modal_visible = false;
                    } else if self.local_state.modal_visible && self.local_state.current_image_index > index {
                        // 如果删除的图片在当前显示图片之前，调整索引
                        self.local_state.current_image_index -= 1;
                    }

                    // 显示成功通知
                    return self.show_notification(
                        self.i18n.t("local-list.delete-success"),
                        super::NotificationType::Success,
                    );
                }
                Err(e) => {
                    // 删除失败，显示错误通知
                    return self.show_notification(
                        format!("{}: {}", self.i18n.t("local-list.delete-failed"), e),
                        super::NotificationType::Error,
                    );
                }
            }
        }

        iced::Task::none()
    }

    fn handle_set_wallpaper(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 设置壁纸
        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
            let full_path = common::get_absolute_path(&path);
            let wallpaper_mode = self.config.wallpaper.mode;

            // 提前获取翻译文本，避免线程安全问题
            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

            // 异步设置壁纸
            return iced::Task::perform(
                super::async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                move |result| match result {
                    Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                    Err(e) => AppMessage::ShowNotification(
                        format!("{}: {}", failed_message, e),
                        super::NotificationType::Error,
                    ),
                },
            );
        }

        iced::Task::none()
    }

    // 定时切换壁纸相关处理方法

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
        self.local_state.auto_change_enabled = true;
        self.local_state.auto_change_timer = Some(std::time::Instant::now());
        self.local_state.auto_change_last_time = Some(std::time::Instant::now());

        // 根据切换模式启动不同的逻辑
        match self.config.wallpaper.auto_change_mode {
            crate::utils::config::WallpaperAutoChangeMode::Local => {
                // 本地模式：获取支持的图片文件列表
                let data_path = self.config.data.data_path.clone();
                iced::Task::perform(
                    super::async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => AppMessage::Local(super::local::LocalMessage::GetSupportedImagesSuccess(paths)),
                        Err(e) => {
                            AppMessage::Local(super::local::LocalMessage::GetSupportedImagesFailed(e.to_string()))
                        }
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
        self.local_state.auto_change_enabled = false;
        self.local_state.auto_change_timer = None;
        self.local_state.auto_change_last_time = None;
        iced::Task::none()
    }

    /// 处理定时切换壁纸的定时器事件
    fn handle_auto_change_tick(&mut self) -> iced::Task<AppMessage> {
        if !self.local_state.auto_change_enabled {
            return iced::Task::none();
        }

        // 2. 更新最后一次执行时间（用于 UI 显示或其他逻辑参考）
        self.local_state.auto_change_last_time = Some(std::time::Instant::now());

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
                    super::async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            if paths.is_empty() {
                                AppMessage::Local(LocalMessage::GetSupportedImagesFailed(
                                    "没有找到支持的壁纸文件".to_string(),
                                ))
                            } else {
                                AppMessage::Local(LocalMessage::GetSupportedImagesSuccess(paths))
                            }
                        }
                        Err(e) => AppMessage::Local(LocalMessage::GetSupportedImagesFailed(e.to_string())),
                    },
                )
            }
            crate::utils::config::WallpaperAutoChangeMode::Online => {
                let config = self.config.clone();
                let auto_change_running = self.auto_change_running.clone();
                iced::Task::perform(
                    super::async_tasks::async_set_random_online_wallpaper(config, auto_change_running),
                    |result| match result {
                        Ok(path) => AppMessage::Local(LocalMessage::SetRandomWallpaperSuccess(path)),
                        Err(e) => AppMessage::Local(LocalMessage::SetRandomWallpaperFailed(e.to_string())),
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
                super::async_tasks::async_set_random_wallpaper(paths, wallpaper_mode),
                |result| match result {
                    Ok(path) => AppMessage::Local(super::local::LocalMessage::SetRandomWallpaperSuccess(path)),
                    Err(e) => AppMessage::Local(super::local::LocalMessage::SetRandomWallpaperFailed(e.to_string())),
                },
            )
        } else {
            // 没有找到支持的壁纸
            tracing::warn!("[定时切换] [获取] 没有找到支持的壁纸文件");
            let error_message = self.i18n.t("local-list.no-valid-wallpapers").to_string();
            iced::Task::done(AppMessage::ShowNotification(
                error_message,
                super::NotificationType::Error,
            ))
        }
    }

    /// 处理获取支持的图片文件列表失败
    fn handle_get_supported_images_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        tracing::error!("[定时切换] [失败] 获取壁纸列表失败: {}", error);
        self.local_state.auto_change_enabled = false;
        let error_message = format!("获取壁纸列表失败: {}", error);
        iced::Task::done(AppMessage::ShowNotification(
            error_message,
            super::NotificationType::Error,
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
            super::NotificationType::Error,
        ))
    }
}
