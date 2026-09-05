// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::Sorting;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, flat_dropdown_trigger_button,
};
use crate::ui::online::{DisplayableSorting, OnlineMessage, OnlineState};
use crate::ui::style::{FILTER_CONTROL_HEIGHT, ThemeColors};
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};

/// 创建排序方式选择器
pub fn create_sorting_picker<'a>(
    i18n: &'a I18n,
    state: &'a OnlineState,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let sorting_options: Vec<DisplayableSorting> = Sorting::all()
        .iter()
        .map(|s| DisplayableSorting {
            value: *s,
            display: i18n.t(s.display_name()).leak(),
        })
        .collect();
    let current_sorting = DisplayableSorting {
        value: state.sorting,
        display: i18n.t(state.sorting.display_name()).leak(),
    };

    // 触发按钮（underlay）
    let sorting_trigger = flat_dropdown_trigger_button(
        current_sorting.display.to_string(),
        100.0,
        theme_colors,
        OnlineMessage::SortingPickerExpanded.into(),
    )
    .height(Length::Fixed(FILTER_CONTROL_HEIGHT));

    // 排序选项（overlay）
    let sorting_options_content = column(sorting_options.iter().map(|option| {
        let is_selected = state.sorting == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(OnlineMessage::SortingChanged(option.value).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(sorting_options_content)
        .padding(8)
        .width(Length::Fixed(120.0))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        sorting_trigger,
        opaque(picker_content),
        state.sorting_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(OnlineMessage::SortingPickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
