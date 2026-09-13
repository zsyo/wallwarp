// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{self, Displayable, dropdown_picker, flat_dropdown_trigger_button};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::{Element, Length};

/// 创建壁纸组选择器（应用 = 写回图源/筛选/铺满模式并刷新列表）
///
/// 组列表为 DB 内存缓存（App::wallpaper_groups），无选中态
pub fn create_group_picker<'a>(
    i18n: &'a I18n,
    app_groups: &'a [crate::services::database::WallpaperGroupDB],
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    // 触发按钮（underlay）
    let trigger = flat_dropdown_trigger_button(
        i18n.t("online-wallpapers.group-picker-label").to_string(),
        100.0,
        theme_colors,
        OnlineMessage::GroupPickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    if app_groups.is_empty() {
        return trigger.into();
    }

    let options: Vec<Displayable<i64>> = app_groups
        .iter()
        .map(|g| Displayable {
            value: g.id,
            display: g.name.clone(),
        })
        .collect();

    dropdown_picker(
        trigger.into(),
        options,
        |_| false,
        |id| OnlineMessage::ApplyGroup(id).into(),
        OnlineMessage::GroupPickerDismiss.into(),
        state.group_picker_expanded,
        160.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    )
}
