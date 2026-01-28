// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::local::{LocalMessage, WallpaperLoadStatus};
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;

impl App {
    /// 显示本地壁纸模态窗口
    pub(in crate::ui::local) fn show_local_modal(&mut self, index: usize) -> Task<AppMessage> {
        // 检查要显示的图片是否为失败状态
        if let Some(wallpaper_status) = self.local_state.wallpapers.get(index) {
            if let WallpaperLoadStatus::Loaded(wallpaper) = wallpaper_status {
                if wallpaper.name == "加载失败" {
                    // 如果是失败的图片，不显示模态窗口
                    return Task::none();
                }
            }
        }

        // 显示模态窗口，设置当前图片索引
        self.local_state.current_image_index = index;
        self.local_state.modal_visible = true;

        // 显式释放旧的图片数据: 先将 Handle 移出,然后让新值覆盖
        let _old_handle = std::mem::replace(&mut self.local_state.modal_image_handle, None);

        // 异步加载图片数据
        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
            return Task::perform(async move { Handle::from_path(&path) }, |handle| {
                AppMessage::Local(LocalMessage::ModalImageLoaded(handle))
            });
        }
        Task::none()
    }

    /// 模态窗口图片加载完成
    pub(in crate::ui::local) fn local_modal_image_loaded(&mut self, handle: Handle) -> Task<AppMessage> {
        // 模态窗口图片加载完成，保存图片数据
        if !self.local_state.modal_visible {
            return Task::none();
        }
        self.local_state.modal_image_handle = Some(handle);
        Task::none()
    }

    /// 关闭模态窗口
    pub fn close_local_modal(&mut self) -> Task<AppMessage> {
        // 关闭模态窗口
        self.local_state.modal_visible = false;

        // 显式释放图片数据: 先将 Handle 移出,然后让新值覆盖
        let _old_handle = std::mem::replace(&mut self.local_state.modal_image_handle, None);
        Task::none()
    }
}
