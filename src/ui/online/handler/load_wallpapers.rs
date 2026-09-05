// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::online::OnlineMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn load_online_wallpapers(&mut self) -> Task<AppMessage> {
        // 设置加载状态
        self.online_state.loading_page = true;
        // 取消所有缩略图加载任务
        self.online_state.cancel_thumb_loads();
        // 清空当前数据，准备加载新数据
        self.online_state.wallpapers.clear();
        self.online_state.wallpapers_data.clear();
        self.online_state.page_info.clear();
        self.online_state.has_loaded = false;

        // 创建新的请求上下文并取消之前的请求
        self.online_state.cancel_and_new_context();
        let context = self.online_state.request_context.clone();

        // 异步加载在线壁纸
        let params = self.build_online_search_params(context);

        Task::perform(
            async_task::async_load_online_wallpapers(params),
            |result| match result {
                Ok((wallpapers, last_page, total_pages, current_page)) => {
                    OnlineMessage::LoadWallpapersSuccess(
                        wallpapers,
                        last_page,
                        total_pages,
                        current_page,
                    )
                    .into()
                }
                Err(e) => OnlineMessage::LoadWallpapersFailed(e.to_string()).into(),
            },
        )
    }
}
