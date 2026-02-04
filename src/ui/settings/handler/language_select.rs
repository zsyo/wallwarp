// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_language_selected(&mut self, lang: String) -> Task<AppMessage> {
        let old_lang = self.config.global.language.clone();
        info!("[设置] [语言] 修改: {} -> {}", old_lang, lang);
        self.i18n.set_language(lang.clone());
        self.tray_manager.update_i18n(&self.i18n);
        // 同时更新配置
        self.config.set_language(lang);
        Task::none()
    }
}
