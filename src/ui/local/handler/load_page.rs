// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::services::local::Wallpaper;
use crate::ui::local::{LocalMessage, WallpaperLoadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 加载本地壁纸页面
    pub(in crate::ui::local) fn load_local_page(&mut self) -> Task<AppMessage> {
        if self.local_state.current_page * self.local_state.page_size >= self.local_state.total_count {
            // 没有更多壁纸可加载
            self.local_state.loading_page = false;
            return Task::none();
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

            tasks.push(Task::perform(
                async_task::async_load_single_wallpaper_with_fallback(path.clone(), cache_path),
                move |result| match result {
                    Ok(wallpaper) => LocalMessage::LoadPageSuccess(vec![(absolute_idx, wallpaper)]).into(),
                    Err(_) => {
                        // 创建失败状态，使用原始路径作为图片源
                        let mut failed_wallpaper = Wallpaper::new(path, "加载失败".to_string(), 0, 0, 0);
                        // 即使失败也创建 Handle，以便在 UI 中显示占位图
                        failed_wallpaper.image_handle = Some(iced::widget::image::Handle::from_path(&failed_wallpaper.thumbnail_path));
                        LocalMessage::LoadPageSuccess(vec![(absolute_idx, failed_wallpaper)]).into()
                    }
                },
            ));
        }

        // 更新当前页面的壁纸状态为加载中
        let page_start = self.local_state.current_page * self.local_state.page_size;
        let page_end = std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

        if self.local_state.current_page == 0 {
            // 第一页：初始化wallpapers数组
            self.local_state.wallpapers = vec![WallpaperLoadStatus::Loading; page_end];
        } else {
            // 后续页面：扩展wallpapers数组
            for _ in 0..(page_end - self.local_state.wallpapers.len()) {
                self.local_state.wallpapers.push(WallpaperLoadStatus::Loading);
            }
        }

        self.local_state.current_page += 1;
        Task::batch(tasks)
    }

    /// 处理本地壁纸页面加载成功
    pub(in crate::ui::local) fn load_local_page_success(
        &mut self,
        wallpapers_with_idx: Vec<(usize, Wallpaper)>,
    ) -> Task<AppMessage> {
        // 为每个加载完成的壁纸更新状态
        for (idx, wallpaper) in wallpapers_with_idx {
            if idx < self.local_state.wallpapers.len() {
                self.local_state.wallpapers[idx] = WallpaperLoadStatus::Loaded(wallpaper);
            }
        }

        // 检查是否所有壁纸都已加载完成，如果是则更新loading_page状态
        let page_start = (self.local_state.current_page - 1) * self.local_state.page_size; // 上一页的起始位置
        let page_end = std::cmp::min(page_start + self.local_state.page_size, self.local_state.total_count);

        let all_loaded = (page_start..page_end).all(|i| {
            i < self.local_state.wallpapers.len()
                && matches!(self.local_state.wallpapers[i], WallpaperLoadStatus::Loaded(_))
        });

        if all_loaded {
            self.local_state.loading_page = false;

            // 添加检查是否需要自动加载下一页的任务
            let check_task = Task::done(LocalMessage::CheckAndLoadNextPage.into());
            return check_task;
        }
        Task::none()
    }
}
