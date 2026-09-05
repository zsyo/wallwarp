// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::online::OnlineMessage;
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

        // 异步加载下一页在线壁纸
        let params = self.build_online_search_params(context);

        Task::perform(
            async_task::async_load_online_wallpapers(params),
            |result| match result {
                Ok((wallpapers, last_page, total_pages, current_page)) => {
                    OnlineMessage::LoadPageSuccess(wallpapers, last_page, total_pages, current_page)
                        .into()
                }
                Err(e) => OnlineMessage::LoadPageFailed(e.to_string()).into(),
            },
        )
    }
}
