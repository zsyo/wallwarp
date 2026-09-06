// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::Sorting;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, Displayable, dropdown_picker, flat_dropdown_trigger_button,
};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::{Element, Length};

/// 创建排序方式选择器
pub fn create_sorting_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let options: Vec<Displayable<Sorting>> = Sorting::all()
        .iter()
        .map(|s| Displayable {
            value: *s,
            display: i18n.t(s.display_name()),
        })
        .collect();

    // 触发按钮（underlay）
    let trigger = flat_dropdown_trigger_button(
        i18n.t(state.sorting.display_name()).to_string(),
        100.0,
        theme_colors,
        OnlineMessage::SortingPickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    dropdown_picker(
        trigger.into(),
        options,
        |v| v == state.sorting,
        |v| OnlineMessage::SortingChanged(v).into(),
        OnlineMessage::SortingPickerDismiss.into(),
        state.sorting_picker_expanded,
        120.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    )
}
