// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::TimeRange;
use crate::ui::common::create_tooltip_style;
use crate::ui::common::drop_down::{self, Displayable, dropdown_picker, dropdown_trigger_button};
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::{App, AppMessage};
use iced::Element;
use iced::widget::{text, tooltip};

/// 创建时间范围选择器
pub fn create_time_range_picker<'a>(
    app: &'a App,
    theme_colors: ThemeColors,
) -> Element<'a, AppMessage> {
    let options: Vec<Displayable<TimeRange>> = TimeRange::all()
        .iter()
        .map(|t| Displayable {
            value: *t,
            display: app.i18n.t(t.display_name()),
        })
        .collect();

    // 触发按钮（underlay），用 tooltip 包裹
    let trigger = dropdown_trigger_button(
        app.i18n
            .t(app.settings_state.auto_change_time_range.display_name())
            .to_string(),
        130.0,
        theme_colors,
        SettingsMessage::TimeRangePickerExpanded.into(),
    );

    let tooltip_text = text(app.i18n.t("settings.auto-change-time-range-tooltip")).style(
        move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        },
    );

    let trigger_with_tooltip = tooltip(trigger, tooltip_text, tooltip::Position::Top)
        .style(create_tooltip_style(theme_colors));

    dropdown_picker(
        trigger_with_tooltip.into(),
        options,
        |v| v == app.settings_state.auto_change_time_range,
        |v| SettingsMessage::AutoChangeTimeRangeChanged(v).into(),
        SettingsMessage::TimeRangePickerDismiss.into(),
        app.settings_state.time_range_picker_expanded,
        150.0,
        drop_down::Alignment::Top,
        theme_colors,
    )
}
