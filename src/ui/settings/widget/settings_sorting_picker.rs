// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::Sorting;
use crate::ui::common::create_tooltip_style;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::{App, AppMessage};
use iced::widget::{button, column, container, opaque, text, tooltip};
use iced::{Element, Length};

/// 显示用的排序方式包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableSorting {
    pub value: Sorting,
    pub display: &'static str,
}

impl std::fmt::Display for DisplayableSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// 创建排序方式选择器
pub fn create_sorting_picker<'a>(
    app: &'a App,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let sorting_options: Vec<DisplayableSorting> = Sorting::all()
        .iter()
        .map(|s| DisplayableSorting {
            value: *s,
            display: app.i18n.t(s.display_name()).leak(),
        })
        .collect();
    let current_sorting = DisplayableSorting {
        value: app.settings_state.auto_change_sorting,
        display: app
            .i18n
            .t(app.settings_state.auto_change_sorting.display_name())
            .leak(),
    };

    // 触发按钮（underlay）
    let sorting_trigger = dropdown_trigger_button(
        current_sorting.display.to_string(),
        100.0,
        theme_colors,
        SettingsMessage::SortingPickerExpanded.into(),
    );

    // 用 tooltip 包裹排序方式选择器
    let sorting_tooltip_text = text(app.i18n.t("settings.auto-change-sorting-tooltip")).style(
        move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        },
    );

    let sorting_trigger_with_tooltip = tooltip(
        sorting_trigger,
        sorting_tooltip_text,
        tooltip::Position::Top,
    )
    .style(create_tooltip_style(theme_colors));

    // 排序选项（overlay）
    let sorting_options_content = column(sorting_options.iter().map(|option| {
        let is_selected = app.settings_state.auto_change_sorting == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::AutoChangeSortingChanged(option.value).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(sorting_options_content)
        .padding(8)
        .width(Length::Fixed(120.0))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        sorting_trigger_with_tooltip,
        opaque(picker_content),
        app.settings_state.sorting_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::SortingPickerDismiss.into())
    .alignment(drop_down::Alignment::Top)
    .into()
}
