// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_language_selected(
        &mut self,
        lang: String,
    ) -> Task<AppMessage> {
        let old_lang = self.config.global.language.clone();
        info!("[设置] [语言] 修改: {} -> {}", old_lang, lang);
        self.i18n.set_language(lang.clone());
        self.tray_manager.update_i18n(&self.i18n);
        self.floating_ball.update_i18n(&self.i18n);
        // 同时更新配置，磁盘写入经防抖合并
        self.config.set_language(lang);
        // 自动收起语言选择器
        self.settings_state.language_picker_expanded = false;
        self.request_config_save()
    }
}
