// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::online::{OnlineMessage, ResolutionMode};
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn load_online_page(&mut self) -> Task<AppMessage> {
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

        let proxy = if self.config.global.proxy_enabled && !self.config.global.proxy.is_empty() {
            Some(self.config.global.proxy.clone())
        } else {
            None
        };

        // 计算分辨率参数
        let atleast = if self.online_state.resolution_mode == ResolutionMode::AtLeast {
            self.online_state.atleast_resolution.map(|r| r.value().to_string())
        } else {
            None
        };

        let resolutions = if self.online_state.resolution_mode == ResolutionMode::Exactly {
            if !self.online_state.selected_resolutions.is_empty() {
                let res_list: Vec<String> = self
                    .online_state
                    .selected_resolutions
                    .iter()
                    .map(|r| r.value().to_string())
                    .collect();
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

        Task::perform(
            async_task::async_load_online_wallpapers(
                categories,
                sorting,
                purities,
                color,
                query,
                time_range,
                atleast,
                resolutions,
                ratios,
                page,
                api_key,
                proxy,
                context,
            ),
            |result| match result {
                Ok((wallpapers, last_page, total_pages, current_page)) => {
                    OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page).into()
                }
                Err(e) => OnlineMessage::LoadPageFailed(e.to_string()).into(),
            },
        )
    }
}
