// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::TimeRange;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, Displayable, dropdown_picker, flat_dropdown_trigger_button,
};
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::{Element, Length};

/// 创建时间范围选择器
pub fn create_time_range_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let options: Vec<Displayable<TimeRange>> = TimeRange::all()
        .iter()
        .map(|t| Displayable {
            value: *t,
            display: i18n.t(t.display_name()),
        })
        .collect();

    // 触发按钮（underlay）
    let trigger = flat_dropdown_trigger_button(
        i18n.t(state.time_range.display_name()).to_string(),
        130.0,
        theme_colors,
        OnlineMessage::TimeRangePickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    dropdown_picker(
        trigger.into(),
        options,
        |v| v == state.time_range,
        |v| OnlineMessage::TimeRangeChanged(v).into(),
        OnlineMessage::TimeRangePickerDismiss.into(),
        state.time_range_picker_expanded,
        150.0,
        drop_down::Alignment::Bottom,
        theme_colors,
    )
}
