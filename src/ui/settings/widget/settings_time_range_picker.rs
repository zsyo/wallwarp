// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::TimeRange;
use crate::ui::common::create_tooltip_style;
use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::{App, AppMessage};
use iced::widget::{button, column, container, opaque, text, tooltip};
use iced::{Element, Length};

/// 显示用的时间范围包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableTimeRange {
    pub value: TimeRange,
    pub display: &'static str,
}

impl std::fmt::Display for DisplayableTimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// 创建时间范围选择器
pub fn create_time_range_picker<'a>(
    app: &'a App,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let time_range_options: Vec<DisplayableTimeRange> = TimeRange::all()
        .iter()
        .map(|t| DisplayableTimeRange {
            value: *t,
            display: app.i18n.t(t.display_name()).leak(),
        })
        .collect();
    let current_time_range = DisplayableTimeRange {
        value: app.settings_state.auto_change_time_range,
        display: app
            .i18n
            .t(app.settings_state.auto_change_time_range.display_name())
            .leak(),
    };

    // 触发按钮（underlay）
    let time_range_trigger = dropdown_trigger_button(
        current_time_range.display.to_string(),
        130.0,
        theme_colors,
        SettingsMessage::TimeRangePickerExpanded.into(),
    );

    // 用 tooltip 包裹时间范围选择器
    let time_range_tooltip_text = text(app.i18n.t("settings.auto-change-time-range-tooltip"))
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let time_range_trigger_with_tooltip = tooltip(
        time_range_trigger,
        time_range_tooltip_text,
        tooltip::Position::Top,
    )
    .style(create_tooltip_style(theme_colors));

    // 时间范围选项（overlay）
    let time_range_options_content = column(time_range_options.iter().map(|option| {
        let is_selected = app.settings_state.auto_change_time_range == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::AutoChangeTimeRangeChanged(option.value).into())
            .style(dropdown_option_style(theme_colors, is_selected))
            .into()
    }))
    .spacing(2);

    let picker_content = container(time_range_options_content)
        .padding(8)
        .width(Length::Fixed(150.0))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        time_range_trigger_with_tooltip,
        opaque(picker_content),
        app.settings_state.time_range_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::TimeRangePickerDismiss.into())
    .alignment(drop_down::Alignment::Top)
    .into()
}
