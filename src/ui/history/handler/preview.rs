// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 历史记录预览模态（打开/翻页/关闭）

use crate::ui::history::HistoryMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

impl App {
    /// 打开预览模态并异步加载原图
    pub(in crate::ui::history) fn preview_history_entry(&mut self, index: usize) -> Task<AppMessage> {
        if let Some(entry) = self.history_state.entries.get(index) {
            self.history_state.modal_visible = true;
            self.history_state.modal_index = index;
            self.history_state.modal_handle = None;

            let path = crate::utils::helpers::get_absolute_path(&entry.path);
            return Task::perform(
                async move { Handle::from_path(&path) },
                |handle| HistoryMessage::ModalImageLoaded(handle).into(),
            );
        }
        Task::none()
    }

    /// 预览上一张
    pub(in crate::ui::history) fn previous_history_image(&mut self) -> Task<AppMessage> {
        let index = self.history_state.modal_index;
        if index > 0 {
            return self.preview_history_entry(index - 1);
        }
        Task::none()
    }

    /// 预览下一张
    pub(in crate::ui::history) fn next_history_image(&mut self) -> Task<AppMessage> {
        let index = self.history_state.modal_index;
        if index + 1 < self.history_state.entries.len() {
            return self.preview_history_entry(index + 1);
        }
        Task::none()
    }

    /// 预览原图加载完成
    pub(in crate::ui::history) fn history_modal_image_loaded(&mut self, handle: Handle) -> Task<AppMessage> {
        if self.history_state.modal_visible {
            self.history_state.modal_handle = Some(handle);
        }
        Task::none()
    }
}
