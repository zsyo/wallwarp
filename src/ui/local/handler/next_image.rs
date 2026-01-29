// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::local::LocalMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

impl App {
    /// 显示下一张图片
    pub(in crate::ui::local) fn next_local_image(&mut self) -> Task<AppMessage> {
        // 显示下一张图片，跳过加载失败的图片
        if let Some(next_index) = self
            .local_state
            .find_next_valid_image_index(self.local_state.current_image_index, 1)
        {
            self.local_state.current_image_index = next_index;

            // 显式释放旧的图片数据: 先将 Handle 移出,然后让新值覆盖
            let _old_handle = std::mem::replace(&mut self.local_state.modal_image_handle, None);

            // 异步加载图片数据
            if let Some(path) = self.local_state.all_paths.get(next_index).cloned() {
                return Task::perform(
                    async move {
                        // 异步加载图片数据
                        Handle::from_path(&path)
                    },
                    |handle| LocalMessage::ModalImageLoaded(handle).into(),
                );
            }
        }
        Task::none()
    }
}
