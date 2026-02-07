// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::config::Theme;
use crate::utils::window_utils;
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::main) fn theme_selected(&mut self, theme: Theme) -> Task<AppMessage> {
        let old_theme = self.config.global.theme;
        tracing::info!("[设置] [主题] 修改: {:?} -> {:?}", old_theme, theme);

        // 更新配置
        self.config.global.theme = theme;
        self.config.save_to_file();

        // 关闭选择器
        self.settings_state.theme_picker_expanded = false;

        self.auto_change_state.auto_detect_color_mode = theme == Theme::Auto;

        self.toggle_theme(theme)
    }

    pub(super) fn toggle_theme(&mut self, theme: Theme) -> Task<AppMessage> {
        // 根据主题类型决定是否需要切换
        match theme {
            Theme::Dark => {
                // 暗色主题：如果当前不是暗色，则切换
                if !self.theme_config.is_dark() {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    info!("[设置] [主题] 切换到: {}", theme_name);
                    // 更新主题颜色缓存
                    self.theme_colors = self.theme_config.get_theme_colors();
                }
            }
            Theme::Light => {
                // 亮色主题：如果当前是暗色，则切换
                if self.theme_config.is_dark() {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    info!("[设置] [主题] 切换到: {}", theme_name);
                    // 更新主题颜色缓存
                    self.theme_colors = self.theme_config.get_theme_colors();
                }
            }
            Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = window_utils::get_system_color_mode();
                info!(
                    "[设置] [主题] 自动模式，系统主题: {}",
                    if is_system_dark { "深色" } else { "浅色" }
                );

                // 如果当前主题与系统主题不一致，则切换
                if self.theme_config.is_dark() != is_system_dark {
                    self.theme_config.toggle();
                    let theme_name = self.theme_config.get_theme().name();
                    info!("[设置] [主题] 切换到: {}", theme_name);
                    // 更新主题颜色缓存
                    self.theme_colors = self.theme_config.get_theme_colors();
                }
            }
        }
        Task::none()
    }
}
