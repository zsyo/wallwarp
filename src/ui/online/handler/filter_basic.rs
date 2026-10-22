// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::services::source::{SiteCategory, SourceColor};
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_filter_source_color_changed(
        &mut self,
        color: SourceColor,
    ) -> Task<AppMessage> {
        self.online_state.source_color = color;
        // 同步到配置（内存），磁盘写入经防抖合并
        self.online_state.save_to_config(&mut self.config);
        // 选择颜色后自动关闭选择器
        self.online_state.source_color_picker_expanded = false;
        self.request_config_save()
    }

    pub(in crate::ui::online) fn online_filter_source_color_picker_expanded(
        &mut self,
    ) -> Task<AppMessage> {
        self.online_state.source_color_picker_expanded =
            !self.online_state.source_color_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_source_color_picker_dismiss(
        &mut self,
    ) -> Task<AppMessage> {
        self.online_state.source_color_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_site_category_changed(
        &mut self,
        category: SiteCategory,
    ) -> Task<AppMessage> {
        self.online_state.site_category = category;
        // 同步到配置（内存），磁盘写入经防抖合并
        self.online_state.save_to_config(&mut self.config);
        // 选择分类后自动关闭选择器
        self.online_state.site_category_picker_expanded = false;
        self.request_config_save()
    }

    pub(in crate::ui::online) fn online_filter_site_category_picker_expanded(
        &mut self,
    ) -> Task<AppMessage> {
        self.online_state.site_category_picker_expanded =
            !self.online_state.site_category_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_site_category_picker_dismiss(
        &mut self,
    ) -> Task<AppMessage> {
        self.online_state.site_category_picker_expanded = false;
        Task::none()
    }
}
