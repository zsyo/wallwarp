// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_filter_color_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换颜色选择器的展开/收起状态
        self.online_state.color_picker_expanded = !self.online_state.color_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_color_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭颜色选择器
        self.online_state.color_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换分辨率选择器的展开/收起状态
        self.online_state.resolution_picker_expanded = !self.online_state.resolution_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭分辨率选择器
        self.online_state.resolution_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换比例选择器的展开/收起状态
        self.online_state.ratio_picker_expanded = !self.online_state.ratio_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭比例选择器
        self.online_state.ratio_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_sorting_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换排序方式选择器的展开/收起状态
        self.online_state.sorting_picker_expanded = !self.online_state.sorting_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_sorting_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭排序方式选择器
        self.online_state.sorting_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_time_range_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换时间范围选择器的展开/收起状态
        self.online_state.time_range_picker_expanded = !self.online_state.time_range_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_time_range_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭时间范围选择器
        self.online_state.time_range_picker_expanded = false;
        Task::none()
    }
}
