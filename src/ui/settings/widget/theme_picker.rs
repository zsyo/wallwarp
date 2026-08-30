// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::main::MainMessage;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::THEME_PICK_LIST_WIDTH;
use crate::ui::{App, AppMessage};
use crate::utils::config::Theme;
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 创建主题选择器
pub fn create_theme_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_theme = app.config.global.theme;

    // 根据当前主题获取对应的翻译文本
    let current_theme_text = match current_theme {
        Theme::Dark => app.i18n.t("theme-options.dark"),
        Theme::Light => app.i18n.t("theme-options.light"),
        Theme::Auto => app.i18n.t("theme-options.auto"),
    };

    // 触发按钮（underlay）
    let theme_trigger = dropdown_trigger_button(
        current_theme_text,
        THEME_PICK_LIST_WIDTH,
        theme_colors,
        SettingsMessage::ThemePickerExpanded.into(),
    );

    // 主题选项（overlay）
    let options = [
        (Theme::Dark, "theme-options.dark"),
        (Theme::Light, "theme-options.light"),
        (Theme::Auto, "theme-options.auto"),
    ];
    let theme_options_content = column(options.iter().map(|(theme, key)| {
        let is_selected = current_theme == *theme;
        button(text(app.i18n.t(key)).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(MainMessage::ThemeSelected(*theme).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(theme_options_content)
        .padding(8)
        .width(Length::Fixed(THEME_PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        theme_trigger,
        opaque(picker_content),
        app.settings_state.theme_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::ThemePickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
