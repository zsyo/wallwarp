// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::TimeRange;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, flat_dropdown_trigger_button,
};
use crate::ui::online::{DisplayableTimeRange, OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 创建时间范围选择器
pub fn create_time_range_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let time_range_options: Vec<DisplayableTimeRange> = TimeRange::all()
        .iter()
        .map(|t| DisplayableTimeRange {
            value: *t,
            display: i18n.t(t.display_name()).leak(),
        })
        .collect();
    let current_time_range = DisplayableTimeRange {
        value: state.time_range,
        display: i18n.t(state.time_range.display_name()).leak(),
    };

    // 触发按钮（underlay）
    let time_range_trigger = flat_dropdown_trigger_button(
        current_time_range.display.to_string(),
        130.0,
        theme_colors,
        OnlineMessage::TimeRangePickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    // 时间范围选项（overlay）
    let time_range_options_content = column(time_range_options.iter().map(|option| {
        let is_selected = state.time_range == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(OnlineMessage::TimeRangeChanged(option.value).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(time_range_options_content)
        .padding(8)
        .width(Length::Fixed(150.0))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        time_range_trigger,
        opaque(picker_content),
        state.time_range_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(OnlineMessage::TimeRangePickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
