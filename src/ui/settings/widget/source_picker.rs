// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::source::SourceKind;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::LANG_PICK_LIST_WIDTH;
use crate::ui::{App, AppMessage};
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 图源显示名对应的词条键
fn source_display_name(i18n: &crate::i18n::I18n, kind: SourceKind) -> String {
    match kind {
        SourceKind::Wallhaven => i18n.t("settings.source-name-wallhaven"),
        SourceKind::Bing => i18n.t("settings.source-name-bing"),
    }
}

/// 创建在线图源选择器（设置页图源区块，切换后在线页按图源刷新）
pub fn create_source_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_source = app.online_state.source;

    // 触发按钮（underlay）
    let trigger = dropdown_trigger_button(
        source_display_name(&app.i18n, current_source),
        LANG_PICK_LIST_WIDTH,
        theme_colors,
        SettingsMessage::OnlineSourcePickerExpanded.into(),
    );

    // 图源选项（overlay）
    let options_content = column(SourceKind::all().iter().map(|kind| {
        let is_selected = current_source == *kind;
        button(text(source_display_name(&app.i18n, *kind)).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::OnlineSourceSelected(*kind).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(options_content)
        .padding(8)
        .width(Length::Fixed(LANG_PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        trigger,
        opaque(picker_content),
        app.settings_state.online_source_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::OnlineSourcePickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
