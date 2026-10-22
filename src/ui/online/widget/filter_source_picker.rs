// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::i18n::I18n;
use crate::services::source::SourceKind;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{Displayable, dropdown_picker, flat_dropdown_trigger_button};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::{Element, Length};

/// 图源显示名对应的词条键
fn source_display_name(i18n: &I18n, kind: SourceKind) -> String {
    match kind {
        SourceKind::Wallhaven => i18n.t("online-wallpapers.source-name-wallhaven"),
        SourceKind::Bing => i18n.t("online-wallpapers.source-name-bing"),
        SourceKind::Wallpaperscraft => i18n.t("online-wallpapers.source-name-wallpaperscraft"),
        SourceKind::Upx8 => i18n.t("online-wallpapers.source-name-upx8"),
    }
}

/// 创建图源选择器（顶栏最前，切换后在线页按图源重新加载与裁剪筛选项）
pub fn create_source_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let current_source = state.source;

    let trigger = flat_dropdown_trigger_button(
        source_display_name(i18n, current_source),
        120.0,
        theme_colors,
        OnlineMessage::SourcePickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    let options: Vec<Displayable<SourceKind>> = SourceKind::all()
        .iter()
        .map(|kind| Displayable {
            value: *kind,
            display: source_display_name(i18n, *kind),
        })
        .collect();

    dropdown_picker(
        trigger.into(),
        options,
        |kind| kind == current_source,
        |kind| OnlineMessage::SourceSelected(kind).into(),
        OnlineMessage::SourcePickerDismiss.into(),
        state.source_picker_expanded,
        140.0,
        crate::ui::common::drop_down::Alignment::Bottom,
        theme_colors,
    )
}
