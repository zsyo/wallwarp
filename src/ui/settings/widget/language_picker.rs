// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::LANG_PICK_LIST_WIDTH;
use crate::ui::{App, AppMessage};
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 创建语言选择器
pub fn create_language_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_lang_code = app.i18n.current_lang.clone();
    let current_lang_name = app
        .i18n
        .available_langs
        .iter()
        .find(|info| info.code == current_lang_code)
        .map(|info| info.name.clone())
        .unwrap_or_else(|| current_lang_code.clone());

    // 触发按钮（underlay）
    let lang_trigger = dropdown_trigger_button(
        current_lang_name,
        LANG_PICK_LIST_WIDTH,
        theme_colors,
        SettingsMessage::LanguagePickerExpanded.into(),
    );

    // 语言选项（overlay）
    let lang_options = app.i18n.lang_codes_and_names();
    let lang_options_content = column(lang_options.iter().map(|(code, name)| {
        let is_selected = current_lang_code == *code;
        button(text(name.clone()).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::LanguageSelected(code.clone()).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(lang_options_content)
        .padding(8)
        .width(Length::Fixed(LANG_PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        lang_trigger,
        opaque(picker_content),
        app.settings_state.language_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::LanguagePickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
