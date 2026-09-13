// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, SettingsCategory};
use iced::Task;

impl App {
    pub(in crate::ui::settings) fn settings_category_selected(
        &mut self,
        category: SettingsCategory,
    ) -> Task<AppMessage> {
        self.settings_state.active_category = category;
        // 首次进入壁纸分类时加载显示器列表（多显示器独立壁纸区块）
        let mut task = Task::none();
        if category == SettingsCategory::Wallpaper && !self.settings_state.monitors_loaded {
            task = self.settings_load_monitors();
        }
        // 切换分类时收起全部下拉框，避免返回原分类时残留展开态
        self.settings_state.language_picker_expanded = false;
        self.settings_state.proxy_protocol_picker_expanded = false;
        self.settings_state.theme_picker_expanded = false;
        self.settings_state.log_level_picker_expanded = false;
        self.settings_state.sorting_picker_expanded = false;
        self.settings_state.time_range_picker_expanded = false;
        task
    }
}
