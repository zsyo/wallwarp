use super::App;
use super::AppMessage;
use super::online::OnlineMessage;
use tracing::error;

impl App {
    /// 处理在线壁纸相关消息
    pub fn handle_online_message(&mut self, msg: OnlineMessage) -> iced::Task<AppMessage> {
        match msg {
            OnlineMessage::LoadWallpapers => self.handle_load_online_wallpapers(),
            OnlineMessage::LoadWallpapersSuccess(wallpapers, last_page, total_pages, current_page) => {
                self.handle_load_online_wallpapers_success(wallpapers, last_page, total_pages, current_page)
            }
            OnlineMessage::LoadWallpapersFailed(error) => self.handle_load_online_wallpapers_failed(error),
            OnlineMessage::WallpaperSelected(wallpaper) => self.handle_online_wallpaper_selected(wallpaper),
            OnlineMessage::LoadPage => self.handle_load_online_page(),
            OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page) => {
                self.handle_load_online_page_success(wallpapers, last_page, total_pages, current_page)
            }
            OnlineMessage::LoadPageFailed(error) => self.handle_load_online_page_failed(error),
            OnlineMessage::ShowModal(index) => self.handle_show_online_modal(index),
            OnlineMessage::ModalImageLoaded(handle) => self.handle_online_modal_image_loaded(handle),
            OnlineMessage::CloseModal => self.handle_close_online_modal(),
            OnlineMessage::NextImage => self.handle_next_online_image(),
            OnlineMessage::PreviousImage => self.handle_previous_online_image(),
            OnlineMessage::ThumbLoaded(idx, handle) => self.handle_thumb_loaded(idx, handle),
            OnlineMessage::DownloadWallpaper(index) => self.handle_download_online_wallpaper(index),
            OnlineMessage::SetAsWallpaper(index) => self.handle_set_online_wallpaper(index),
            OnlineMessage::CategoryToggled(category) => self.handle_category_toggled(category),
            OnlineMessage::SortingChanged(sorting) => self.handle_sorting_changed(sorting),
            OnlineMessage::PurityToggled(purity) => self.handle_purity_toggled(purity),
            OnlineMessage::SearchTextChanged(text) => self.handle_search_text_changed(text),
            OnlineMessage::Search => self.handle_search(),
            OnlineMessage::Refresh => self.handle_refresh(),
            OnlineMessage::ScrollToBottom => self.handle_online_scroll_to_bottom(),
            OnlineMessage::CheckAndLoadNextPage => self.handle_online_check_and_load_next_page(),
            OnlineMessage::ResolutionChanged(resolution) => self.handle_resolution_changed(resolution),
            OnlineMessage::RatioChanged(ratio) => self.handle_ratio_changed(ratio),
            OnlineMessage::ColorChanged(color) => self.handle_color_changed(color),
            OnlineMessage::ColorPickerExpanded => self.handle_color_picker_expanded(),
            OnlineMessage::ColorPickerDismiss => self.handle_color_picker_dismiss(),
            OnlineMessage::TimeRangeChanged(time_range) => self.handle_time_range_changed(time_range),
            OnlineMessage::ResolutionPickerExpanded => self.handle_resolution_picker_expanded(),
            OnlineMessage::ResolutionPickerDismiss => self.handle_resolution_picker_dismiss(),
            OnlineMessage::ResolutionModeChanged(mode) => self.handle_resolution_mode_changed(mode),
            OnlineMessage::ResolutionToggled(resolution) => self.handle_resolution_toggled(resolution),
            OnlineMessage::ResolutionAtLeastSelected(resolution) => self.handle_resolution_atleast_selected(resolution),
            OnlineMessage::RatioPickerExpanded => self.handle_ratio_picker_expanded(),
            OnlineMessage::RatioPickerDismiss => self.handle_ratio_picker_dismiss(),
            OnlineMessage::RatioLandscapeToggled => self.handle_ratio_landscape_toggled(),
            OnlineMessage::RatioPortraitToggled => self.handle_ratio_portrait_toggled(),
            OnlineMessage::RatioAllToggled => self.handle_ratio_all_toggled(),
            OnlineMessage::RatioToggled(ratio) => self.handle_ratio_toggled(ratio),
        }
    }

    fn handle_load_online_wallpapers(&mut self) -> iced::Task<AppMessage> {
        // 设置加载状态
        self.online_state.loading_page = true;
        // 清空当前数据，准备加载新数据
        self.online_state.wallpapers.clear();
        self.online_state.wallpapers_data.clear();
        self.online_state.page_info.clear();
        self.online_state.has_loaded = false;

        // 创建新的请求上下文并取消之前的请求
        self.online_state.cancel_and_new_context();
        let context = self.online_state.request_context.clone();

        // 异步加载在线壁纸
        let categories = self.online_state.categories;
        let sorting = self.online_state.sorting;
        let purities = self.online_state.purities;
        let color = self.online_state.color;
        let time_range = self.online_state.time_range;
        let query = self.online_state.search_text.clone();
        let page = self.online_state.current_page;
        let api_key = if self.config.wallhaven.api_key.is_empty() {
            None
        } else {
            Some(self.config.wallhaven.api_key.clone())
        };

        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        // 计算分辨率参数
        let atleast = if self.online_state.resolution_mode == super::online::ResolutionMode::AtLeast {
            self.online_state.atleast_resolution.map(|r| r.value().to_string())
        } else {
            None
        };

        let resolutions = if self.online_state.resolution_mode == super::online::ResolutionMode::Exactly {
            if !self.online_state.selected_resolutions.is_empty() {
                let res_list: Vec<String> = self.online_state.selected_resolutions.iter().map(|r| r.value().to_string()).collect();
                Some(res_list.join(","))
            } else {
                None
            }
        } else {
            None
        };

        // 计算比例参数
        let mut ratios_vec = Vec::new();

        // 如果选中"全部横屏"，添加 landscape
        if self.online_state.ratio_landscape_selected {
            ratios_vec.push("landscape".to_string());
        }

        // 如果选中"全部竖屏"，添加 portrait
        if self.online_state.ratio_portrait_selected {
            ratios_vec.push("portrait".to_string());
        }

        // 添加详细模式的 ratios
        for ratio in &self.online_state.selected_ratios {
            ratios_vec.push(ratio.value().to_string());
        }

        // 如果没有任何选中项，则为 None
        let ratios = if ratios_vec.is_empty() {
            None
        } else {
            Some(ratios_vec.join(","))
        };

        iced::Task::perform(
            super::async_tasks::async_load_online_wallpapers(categories, sorting, purities, color, query, time_range, atleast, resolutions, ratios, page, api_key, proxy, context),
            |result| match result {
                Ok((wallpapers, last_page, total_pages, current_page)) => AppMessage::Online(super::online::OnlineMessage::LoadWallpapersSuccess(
                    wallpapers,
                    last_page,
                    total_pages,
                    current_page,
                )),
                Err(e) => AppMessage::Online(super::online::OnlineMessage::LoadWallpapersFailed(e.to_string())),
            },
        )
    }

    fn handle_load_online_wallpapers_success(
        &mut self,
        wallpapers: Vec<super::online::OnlineWallpaper>,
        last_page: bool,
        total_pages: usize,
        current_page: usize,
    ) -> iced::Task<AppMessage> {
        // 更新在线壁纸状态，并开始加载缩略图
        self.online_state.current_page = current_page;
        self.online_state.total_pages = total_pages;

        // 判断是否是最后一页：
        // 如果 current_page == total_pages && current_page == 1 && data 为空，说明无数据
        // 否则 last_page（布尔值）表示已加载到最后一页
        let is_empty_data = wallpapers.is_empty();
        let is_first_and_last_page = current_page == 1 && total_pages == 1;
        self.online_state.last_page = if is_empty_data && is_first_and_last_page {
            // 无数据情况：last_page 为 false（允许后续尝试不同筛选条件时重新加载）
            false
        } else {
            last_page
        };
        self.online_state.has_loaded = true; // 标记已加载过数据

        // 处理空数据但非最后一页的情况：自动加载下一页
        if is_empty_data && !last_page && current_page < total_pages {
            // 空数据但还有后续页面，继续加载下一页
            self.online_state.loading_page = false; // 先设置为 false，避免重复加载
            return self.handle_load_online_page();
        }

        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        let cache_path = self.config.data.cache_path.clone();

        let mut tasks = Vec::new();
        for (idx, wallpaper) in wallpapers.iter().enumerate() {
            let url = wallpaper.thumb_large.clone();
            let file_size = wallpaper.file_size;
            let proxy = proxy.clone();
            let cache_path = cache_path.clone();
            tasks.push(iced::Task::perform(
                super::async_tasks::async_load_online_wallpaper_thumb_with_cache(url, file_size, cache_path, proxy),
                move |result| match result {
                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, handle)),
                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, iced::widget::image::Handle::from_bytes(vec![]))),
                },
            ));
        }

        self.online_state.wallpapers_data = wallpapers.clone();
        self.online_state.wallpapers = wallpapers.into_iter().map(|_w| super::online::WallpaperLoadStatus::Loading).collect();
        self.online_state.total_count = self.online_state.wallpapers.len();

        // 初始化 page_info，记录第一页的结束索引和页码
        self.online_state.page_info.clear();
        // 如果有数据，添加第一页的分页信息（用于在第一页数据后显示分页标识）
        if !self.online_state.wallpapers.is_empty() {
            self.online_state.page_info.push(super::online::PageInfo {
                end_index: self.online_state.wallpapers.len(),
                page_num: current_page,
            });
        }

        self.online_state.loading_page = false;

        iced::Task::batch(tasks)
    }

    fn handle_load_online_wallpapers_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        // 加载失败
        self.online_state.loading_page = false;
        self.online_state.has_loaded = true; // 标记已加载过数据（虽然失败了）
        error!("[在线壁纸] 加载失败: {}", error);
        iced::Task::none()
    }

    fn handle_online_wallpaper_selected(&mut self, _wallpaper: super::online::OnlineWallpaper) -> iced::Task<AppMessage> {
        // 处理壁纸选择
        iced::Task::none()
    }

    fn handle_load_online_page(&mut self) -> iced::Task<AppMessage> {
        // 加载下一页：先递增页码
        self.online_state.current_page += 1;
        self.online_state.loading_page = true;

        // 创建新的请求上下文并取消之前的请求
        self.online_state.cancel_and_new_context();
        let context = self.online_state.request_context.clone();

        let categories = self.online_state.categories;
        let sorting = self.online_state.sorting;
        let purities = self.online_state.purities;
        let color = self.online_state.color;
        let time_range = self.online_state.time_range;
        let query = self.online_state.search_text.clone();
        let page = self.online_state.current_page;
        let api_key = if self.config.wallhaven.api_key.is_empty() {
            None
        } else {
            Some(self.config.wallhaven.api_key.clone())
        };

        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        // 计算分辨率参数
        let atleast = if self.online_state.resolution_mode == super::online::ResolutionMode::AtLeast {
            self.online_state.atleast_resolution.map(|r| r.value().to_string())
        } else {
            None
        };

        let resolutions = if self.online_state.resolution_mode == super::online::ResolutionMode::Exactly {
            if !self.online_state.selected_resolutions.is_empty() {
                let res_list: Vec<String> = self.online_state.selected_resolutions.iter().map(|r| r.value().to_string()).collect();
                Some(res_list.join(","))
            } else {
                None
            }
        } else {
            None
        };

        // 计算比例参数
        let mut ratios_vec = Vec::new();

        // 如果选中"全部横屏"，添加 landscape
        if self.online_state.ratio_landscape_selected {
            ratios_vec.push("landscape".to_string());
        }

        // 如果选中"全部竖屏"，添加 portrait
        if self.online_state.ratio_portrait_selected {
            ratios_vec.push("portrait".to_string());
        }

        // 添加详细模式的 ratios
        for ratio in &self.online_state.selected_ratios {
            ratios_vec.push(ratio.value().to_string());
        }

        // 如果没有任何选中项，则为 None
        let ratios = if ratios_vec.is_empty() {
            None
        } else {
            Some(ratios_vec.join(","))
        };

        iced::Task::perform(
            super::async_tasks::async_load_online_wallpapers(categories, sorting, purities, color, query, time_range, atleast, resolutions, ratios, page, api_key, proxy, context),
            |result| match result {
                Ok((wallpapers, last_page, total_pages, current_page)) => {
                    AppMessage::Online(super::online::OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page))
                }
                Err(e) => AppMessage::Online(super::online::OnlineMessage::LoadPageFailed(e.to_string())),
            },
        )
    }

    fn handle_load_online_page_success(
        &mut self,
        wallpapers: Vec<super::online::OnlineWallpaper>,
        last_page: bool,
        total_pages: usize,
        current_page: usize,
    ) -> iced::Task<AppMessage> {
        // 添加新壁纸到列表，并开始加载缩略图
        self.online_state.current_page = current_page;
        self.online_state.total_pages = total_pages;

        // 判断是否是最后一页：
        // 如果 current_page == total_pages && current_page == 1 && data 为空，说明无数据
        // 否则 last_page（布尔值）表示已加载到最后一页
        let is_empty_data = wallpapers.is_empty();
        let is_first_and_last_page = current_page == 1 && total_pages == 1;
        self.online_state.last_page = if is_empty_data && is_first_and_last_page {
            // 无数据情况：last_page 为 false
            false
        } else {
            last_page
        };
        self.online_state.has_loaded = true; // 标记已加载过数据

        // 处理空数据但非最后一页的情况：自动加载下一页
        if is_empty_data && !last_page && current_page < total_pages {
            // 空数据但还有后续页面，继续加载下一页
            self.online_state.loading_page = false; // 先设置为 false，避免重复加载
            return self.handle_load_online_page();
        }

        let proxy = if self.config.global.proxy.is_empty() {
            None
        } else {
            Some(self.config.global.proxy.clone())
        };

        let cache_path = self.config.data.cache_path.clone();

        let start_idx = self.online_state.wallpapers.len();
        let mut tasks = Vec::new();
        for (offset, wallpaper) in wallpapers.iter().enumerate() {
            let idx = start_idx + offset;
            let url = wallpaper.thumb_large.clone();
            let file_size = wallpaper.file_size;
            let proxy = proxy.clone();
            let cache_path = cache_path.clone();
            tasks.push(iced::Task::perform(
                super::async_tasks::async_load_online_wallpaper_thumb_with_cache(url, file_size, cache_path, proxy),
                move |result| match result {
                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, handle)),
                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ThumbLoaded(idx, iced::widget::image::Handle::from_bytes(vec![]))),
                },
            ));
        }

        // 保存原始数据
        for wallpaper in &wallpapers {
            self.online_state.wallpapers_data.push(wallpaper.clone());
            self.online_state.wallpapers.push(super::online::WallpaperLoadStatus::Loading);
        }

        // 在添加完当前页数据后记录分页信息
        // 这样分页标识就可以在当前页数据的下面显示
        if !wallpapers.is_empty() {
            self.online_state.page_info.push(super::online::PageInfo {
                end_index: self.online_state.wallpapers.len(),
                page_num: current_page,
            });
        }

        self.online_state.loading_page = false;

        iced::Task::batch(tasks)
    }

    fn handle_load_online_page_failed(&mut self, error: String) -> iced::Task<AppMessage> {
        // 加载失败
        self.online_state.loading_page = false;
        self.online_state.has_loaded = true; // 标记已加载过数据（虽然失败了）
        error!("[在线壁纸] 加载页面失败: {}", error);
        iced::Task::none()
    }

    fn handle_show_online_modal(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 显示模态窗口
        self.online_state.current_image_index = index;
        self.online_state.modal_visible = true;
        self.online_state.modal_image_handle = None;

        // 异步加载图片数据
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let proxy = if self.config.global.proxy.is_empty() {
                None
            } else {
                Some(self.config.global.proxy.clone())
            };

            return iced::Task::perform(super::async_tasks::async_load_online_wallpaper_image(url, proxy), |result| match result {
                Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
            });
        }

        iced::Task::none()
    }

    fn handle_online_modal_image_loaded(&mut self, handle: iced::widget::image::Handle) -> iced::Task<AppMessage> {
        // 模态窗口图片加载完成，保存图片数据
        self.online_state.modal_image_handle = Some(handle);
        iced::Task::none()
    }

    fn handle_close_online_modal(&mut self) -> iced::Task<AppMessage> {
        // 关闭模态窗口
        self.online_state.modal_visible = false;
        iced::Task::none()
    }

    fn handle_next_online_image(&mut self) -> iced::Task<AppMessage> {
        // 显示下一张图片
        if self.online_state.current_image_index < self.online_state.wallpapers.len().saturating_sub(1) {
            let next_index = self.online_state.current_image_index + 1;
            self.online_state.current_image_index = next_index;
            self.online_state.modal_image_handle = None;

            if let Some(wallpaper) = self.online_state.wallpapers_data.get(next_index) {
                let url = wallpaper.path.clone();
                let proxy = if self.config.global.proxy.is_empty() {
                    None
                } else {
                    Some(self.config.global.proxy.clone())
                };

                return iced::Task::perform(super::async_tasks::async_load_online_wallpaper_image(url, proxy), |result| match result {
                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
                });
            }
        }

        iced::Task::none()
    }

    fn handle_previous_online_image(&mut self) -> iced::Task<AppMessage> {
        // 显示上一张图片
        if self.online_state.current_image_index > 0 {
            let prev_index = self.online_state.current_image_index - 1;
            self.online_state.current_image_index = prev_index;
            self.online_state.modal_image_handle = None;

            if let Some(wallpaper) = self.online_state.wallpapers_data.get(prev_index) {
                let url = wallpaper.path.clone();
                let proxy = if self.config.global.proxy.is_empty() {
                    None
                } else {
                    Some(self.config.global.proxy.clone())
                };

                return iced::Task::perform(super::async_tasks::async_load_online_wallpaper_image(url, proxy), |result| match result {
                    Ok(handle) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(handle)),
                    Err(_) => AppMessage::Online(super::online::OnlineMessage::ModalImageLoaded(iced::widget::image::Handle::from_bytes(vec![]))),
                });
            }
        }

        iced::Task::none()
    }

    fn handle_thumb_loaded(&mut self, idx: usize, handle: iced::widget::image::Handle) -> iced::Task<AppMessage> {
        // 缩略图加载完成
        if idx < self.online_state.wallpapers.len() {
            if let Some(wallpaper) = self.online_state.wallpapers_data.get(idx) {
                self.online_state.wallpapers[idx] = super::online::WallpaperLoadStatus::ThumbLoaded(wallpaper.clone(), handle);
            }
        }
        iced::Task::none()
    }

    fn handle_download_online_wallpaper(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 下载壁纸
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let id = wallpaper.id.clone();
            let file_type = wallpaper.file_type.clone();
            let file_size = wallpaper.file_size;

            // 生成目标文件路径
            let file_name = super::download::generate_file_name(&id, file_type.split('/').last().unwrap_or("jpg"));
            let data_path = self.config.data.data_path.clone();
            let target_path = std::path::PathBuf::from(&data_path).join(&file_name);

            // 1. 检查目标文件是否已存在于 data_path 中
            if let Ok(metadata) = std::fs::metadata(&target_path) {
                let actual_size = metadata.len();
                if actual_size == file_size {
                    // 文件已存在且大小匹配
                    let success_message = format!("{}: {}", self.i18n.t("download-tasks.file-already-exists").to_string(), file_name);
                    return iced::Task::done(AppMessage::ShowNotification(success_message, super::NotificationType::Info));
                }
            }

            // 2. 检查缓存文件是否存在且大小匹配
            let cache_path = self.config.data.cache_path.clone();
            if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, file_size) {
                if let Ok(metadata) = std::fs::metadata(&cache_file_path) {
                    let cache_size = metadata.len();
                    if cache_size == file_size {
                        // 缓存文件存在且大小匹配，直接复制到 data_path
                        let _ = std::fs::create_dir_all(&data_path);
                        match std::fs::copy(&cache_file_path, &target_path) {
                            Ok(_) => {
                                let success_message = format!("{}: {}", self.i18n.t("download-tasks.copied-from-cache").to_string(), file_name);
                                return iced::Task::done(AppMessage::ShowNotification(success_message, super::NotificationType::Success));
                            }
                            Err(e) => {
                                error!("[在线壁纸] [ID:{}] 从缓存复制失败: {}", id, e);
                                // 复制失败，继续走下载流程
                            }
                        }
                    }
                }
            }

            // 3. 检查下载任务列表中是否已有相同 URL 的任务
            let has_duplicate = self.download_state.tasks.iter().any(|task| {
                task.task.url == url
                    && task.task.status != super::download::DownloadStatus::Completed
                    && task.task.status != super::download::DownloadStatus::Cancelled
                    && !matches!(task.task.status, super::download::DownloadStatus::Failed(_))
            });

            if has_duplicate {
                // 任务已在下载队列中
                let info_message = self.i18n.t("download-tasks.task-already-in-queue").to_string();
                return iced::Task::done(AppMessage::ShowNotification(info_message, super::NotificationType::Info));
            }

            // 4. 开始下载
            return self.start_download(url, &id, &file_type);
        }

        iced::Task::none()
    }

    fn handle_set_online_wallpaper(&mut self, index: usize) -> iced::Task<AppMessage> {
        // 设为壁纸
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let id = wallpaper.id.clone();
            let file_type = wallpaper.file_type.clone();
            let file_size = wallpaper.file_size;

            // 生成目标文件路径
            let file_name = super::download::generate_file_name(&id, file_type.split('/').last().unwrap_or("jpg"));
            let data_path = self.config.data.data_path.clone();
            let target_path = std::path::PathBuf::from(&data_path).join(&file_name);

            // 1. 检查目标文件是否已存在于 data_path 中
            if let Ok(metadata) = std::fs::metadata(&target_path) {
                let actual_size = metadata.len();
                if actual_size == file_size {
                    // 文件已存在且大小匹配，直接设置壁纸
                    let full_path = super::common::get_absolute_path(&target_path.to_string_lossy().to_string());
                    let success_message = self.i18n.t("local-list.set-wallpaper-success").to_string();
                    let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                    return iced::Task::perform(super::async_tasks::async_set_wallpaper(full_path), move |result| match result {
                        Ok(_) => AppMessage::ShowNotification(success_message, super::NotificationType::Success),
                        Err(e) => AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error),
                    });
                }
            }

            // 2. 检查缓存文件是否存在且大小匹配
            let cache_path = self.config.data.cache_path.clone();
            if let Ok(cache_file_path) = crate::services::download::DownloadService::get_online_image_cache_final_path(&cache_path, &url, file_size) {
                if let Ok(metadata) = std::fs::metadata(&cache_file_path) {
                    let cache_size = metadata.len();
                    if cache_size == file_size {
                        // 缓存文件存在且大小匹配，复制到 data_path
                        let _ = std::fs::create_dir_all(&data_path);
                        match std::fs::copy(&cache_file_path, &target_path) {
                            Ok(_) => {
                                // 复制成功，设置壁纸
                                let full_path = super::common::get_absolute_path(&target_path.to_string_lossy().to_string());
                                let success_message = self.i18n.t("local-list.set-wallpaper-success").to_string();
                                let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                                return iced::Task::perform(super::async_tasks::async_set_wallpaper(full_path), move |result| match result {
                                    Ok(_) => AppMessage::ShowNotification(success_message, super::NotificationType::Success),
                                    Err(e) => AppMessage::ShowNotification(format!("{}: {}", failed_message, e), super::NotificationType::Error),
                                });
                            }
                            Err(e) => {
                                error!("[在线壁纸] [ID:{}] 从缓存复制失败: {}", id, e);
                                // 复制失败，继续走下载流程
                            }
                        }
                    }
                }
            }

            // 3. 文件不存在，启动下载任务
            // 设置待设置壁纸的文件名
            self.online_state.pending_set_wallpaper_filename = Some(file_name.clone());

            // 检查下载任务列表中是否已有相同 URL 的任务
            let has_duplicate = self.download_state.tasks.iter().any(|task| {
                task.task.url == url
                    && task.task.status != super::download::DownloadStatus::Completed
                    && task.task.status != super::download::DownloadStatus::Cancelled
                    && !matches!(task.task.status, super::download::DownloadStatus::Failed(_))
            });

            if has_duplicate {
                // 任务已在下载队列中，只更新待设置壁纸的文件名
                let info_message = self.i18n.t("download-tasks.task-already-in-queue").to_string();
                return iced::Task::done(AppMessage::ShowNotification(info_message, super::NotificationType::Info));
            }

            // 开始下载
            return self.start_download(url, &id, &file_type);
        }

        iced::Task::none()
    }

    fn handle_category_toggled(&mut self, category: super::online::Category) -> iced::Task<AppMessage> {
        // 切换分类：使用位掩码而不是枚举索引值
        self.online_state.categories ^= category.bit_value();
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_sorting_changed(&mut self, sorting: super::online::Sorting) -> iced::Task<AppMessage> {
        self.online_state.sorting = sorting;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_purity_toggled(&mut self, purity: super::online::Purity) -> iced::Task<AppMessage> {
        // 切换纯净度：使用位掩码而不是枚举索引值
        self.online_state.purities ^= purity.bit_value();
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_search_text_changed(&mut self, text: String) -> iced::Task<AppMessage> {
        self.online_state.search_text = text;
        iced::Task::none()
    }

    fn handle_search(&mut self) -> iced::Task<AppMessage> {
        // 搜索：重置到第一页并重新加载
        self.online_state.current_page = 1;

        // 滚动到顶部，避免触发自动加载下一页
        let scroll_to_top_task = iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("online_wallpapers".to_string()));

        // 执行搜索和滚动到顶部
        iced::Task::batch([self.handle_load_online_wallpapers(), scroll_to_top_task])
    }

    fn handle_refresh(&mut self) -> iced::Task<AppMessage> {
        // 刷新：清空搜索框内容，重置到第一页并重新加载
        self.online_state.search_text.clear();
        self.online_state.current_page = 1;

        // 滚动到顶部，避免触发自动加载下一页
        let scroll_to_top_task = iced::Task::perform(async {}, |_| AppMessage::ScrollToTop("online_wallpapers".to_string()));

        // 执行刷新和滚动到顶部
        iced::Task::batch([self.handle_load_online_wallpapers(), scroll_to_top_task])
    }

    fn handle_online_scroll_to_bottom(&mut self) -> iced::Task<AppMessage> {
        // 滚动到底部，加载下一页
        if !self.online_state.last_page && !self.online_state.loading_page {
            self.handle_load_online_page()
        } else {
            iced::Task::none()
        }
    }

    fn handle_online_check_and_load_next_page(&mut self) -> iced::Task<AppMessage> {
        // 检查是否需要自动加载下一页
        if !self.online_state.last_page && !self.online_state.loading_page {
            // 如果没有数据，不执行检查（等待空数据自动加载逻辑处理）
            if self.online_state.wallpapers.is_empty() {
                return iced::Task::none();
            }

            // 计算每行可以显示多少张图
            let available_width = (self.current_window_width as f32 - crate::ui::style::IMAGE_SPACING).max(crate::ui::style::IMAGE_WIDTH);
            let unit_width = crate::ui::style::IMAGE_WIDTH + crate::ui::style::IMAGE_SPACING;
            let items_per_row = (available_width / unit_width).floor() as usize;
            let items_per_row = items_per_row.max(1);

            // 计算实际行数
            let num_wallpapers = self.online_state.wallpapers.len();
            let num_rows = (num_wallpapers + items_per_row - 1) / items_per_row;

            // 估算内容高度
            let estimated_content_height = num_rows as f32 * (crate::ui::style::IMAGE_HEIGHT + crate::ui::style::IMAGE_SPACING);

            // 如果估算的内容高度小于窗口高度，需要加载下一页
            // 这样可以确保内容足够多，能够显示滚动条
            if estimated_content_height < self.current_window_height as f32 {
                self.handle_load_online_page()
            } else {
                iced::Task::none()
            }
        } else {
            iced::Task::none()
        }
    }

    fn handle_resolution_changed(&mut self, resolution: super::online::Resolution) -> iced::Task<AppMessage> {
        self.online_state.resolution = resolution;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_ratio_changed(&mut self, ratio: super::online::Ratio) -> iced::Task<AppMessage> {
        self.online_state.ratio = ratio;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_color_changed(&mut self, color: super::online::ColorOption) -> iced::Task<AppMessage> {
        self.online_state.color = color;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        // 选择颜色后自动关闭颜色选择器
        self.online_state.color_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_color_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 切换颜色选择器的展开/收起状态
        self.online_state.color_picker_expanded = !self.online_state.color_picker_expanded;
        iced::Task::none()
    }

    fn handle_color_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭颜色选择器
        self.online_state.color_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_time_range_changed(&mut self, time_range: super::online::TimeRange) -> iced::Task<AppMessage> {
        self.online_state.time_range = time_range;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_resolution_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 切换分辨率选择器的展开/收起状态
        self.online_state.resolution_picker_expanded = !self.online_state.resolution_picker_expanded;
        iced::Task::none()
    }

    fn handle_resolution_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭分辨率选择器
        self.online_state.resolution_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_resolution_mode_changed(&mut self, mode: super::online::ResolutionMode) -> iced::Task<AppMessage> {
        // 切换分辨率筛选模式
        self.online_state.resolution_mode = mode;
        // 切换模式时清空之前的选择
        self.online_state.selected_resolutions.clear();
        self.online_state.atleast_resolution = None;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_resolution_toggled(&mut self, resolution: super::online::Resolution) -> iced::Task<AppMessage> {
        // Exactly模式：切换分辨率选择状态
        if let Some(pos) = self.online_state.selected_resolutions.iter().position(|&r| r == resolution) {
            // 如果已选中，则取消选中
            self.online_state.selected_resolutions.remove(pos);
        } else {
            // 如果未选中，则添加到选中列表
            self.online_state.selected_resolutions.push(resolution);
        }
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_resolution_atleast_selected(&mut self, resolution: super::online::Resolution) -> iced::Task<AppMessage> {
        // AtLeast模式：选择分辨率（不自动关闭）
        self.online_state.atleast_resolution = Some(resolution);
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_ratio_picker_expanded(&mut self) -> iced::Task<AppMessage> {
        // 展开比例选择器
        self.online_state.ratio_picker_expanded = true;
        iced::Task::none()
    }

    fn handle_ratio_picker_dismiss(&mut self) -> iced::Task<AppMessage> {
        // 关闭比例选择器
        self.online_state.ratio_picker_expanded = false;
        iced::Task::none()
    }

    fn handle_ratio_toggled(&mut self, ratio: crate::services::wallhaven::AspectRatio) -> iced::Task<AppMessage> {
        // 切换比例选择状态（多选）
        if let Some(pos) = self.online_state.selected_ratios.iter().position(|&r| r == ratio) {
            // 如果已选中，则取消选中
            self.online_state.selected_ratios.remove(pos);
        } else {
            // 如果未选中，则添加到选中列表
            self.online_state.selected_ratios.push(ratio);
        }
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_ratio_landscape_toggled(&mut self) -> iced::Task<AppMessage> {
        // 切换"全部横屏"选项
        self.online_state.ratio_landscape_selected = !self.online_state.ratio_landscape_selected;

        // 如果选中"全部横屏"，清空宽屏和超宽屏分组下的选中项
        if self.online_state.ratio_landscape_selected {
            self.online_state.selected_ratios.retain(|r| {
                !matches!(r, 
                    crate::services::wallhaven::AspectRatio::R16x9 | 
                    crate::services::wallhaven::AspectRatio::R16x10 |
                    crate::services::wallhaven::AspectRatio::R21x9 |
                    crate::services::wallhaven::AspectRatio::R32x9 |
                    crate::services::wallhaven::AspectRatio::R48x9
                )
            });
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_ratio_portrait_toggled(&mut self) -> iced::Task<AppMessage> {
        // 切换"全部竖屏"选项
        self.online_state.ratio_portrait_selected = !self.online_state.ratio_portrait_selected;

        // 如果选中"全部竖屏"，清空竖屏分组下的选中项
        if self.online_state.ratio_portrait_selected {
            self.online_state.selected_ratios.retain(|r| {
                !matches!(r,
                    crate::services::wallhaven::AspectRatio::R9x16 |
                    crate::services::wallhaven::AspectRatio::R10x16 |
                    crate::services::wallhaven::AspectRatio::R9x18
                )
            });
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }

    fn handle_ratio_all_toggled(&mut self) -> iced::Task<AppMessage> {
        // 切换"全部"选项
        self.online_state.ratio_all_selected = !self.online_state.ratio_all_selected;

        // 如果选中"全部"，清空其他所有选项的选中状态
        if self.online_state.ratio_all_selected {
            self.online_state.ratio_landscape_selected = false;
            self.online_state.ratio_portrait_selected = false;
            self.online_state.selected_ratios.clear();
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        iced::Task::none()
    }
}
