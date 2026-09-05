// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::LOG_LEVEL_PICK_LIST_WIDTH;
use crate::ui::{App, AppMessage};
use crate::utils::config::LogLevel;
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 日志等级对应的翻译词条键
fn log_level_option_key(level: LogLevel) -> String {
    format!("log-level-options.{}", level.as_str())
}

/// 创建日志等级选择器
pub fn create_log_level_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_level = app.config.global.log_level;

    // 触发按钮（underlay）
    let level_trigger = dropdown_trigger_button(
        app.i18n.t(&log_level_option_key(current_level)),
        LOG_LEVEL_PICK_LIST_WIDTH,
        theme_colors,
        SettingsMessage::LogLevelPickerExpanded.into(),
    );

    // 日志等级选项（overlay）
    let options_content = column(LogLevel::all().iter().map(|level| {
        let is_selected = current_level == *level;
        button(text(app.i18n.t(&log_level_option_key(*level))).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::LogLevelSelected(*level).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(options_content)
        .padding(8)
        .width(Length::Fixed(LOG_LEVEL_PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        level_trigger,
        opaque(picker_content),
        app.settings_state.log_level_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::LogLevelPickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
