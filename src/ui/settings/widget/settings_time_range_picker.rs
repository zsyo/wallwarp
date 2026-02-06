// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven::TimeRange;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::*;
use crate::ui::{App, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text, tooltip};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

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
pub fn create_time_range_picker<'a>(app: &'a App, theme_colors: ThemeColors) -> Element<'a, AppMessage> {
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

    // 创建触发按钮（underlay）
    let time_range_underlay = row![
        text(current_time_range.display).size(14),
        Space::new().width(Length::Fill),
        container(text("⏷").color(theme_colors.light_text_sub))
            .height(Length::Fill)
            .padding(iced::Padding {
                top: -2.0,
                bottom: 0.0,
                left: 0.0,
                right: 0.0,
            }),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .padding(iced::Padding {
        top: 0.0,
        bottom: 0.0,
        left: 0.0,
        right: -2.0,
    });

    let time_range_trigger = button(time_range_underlay)
        .padding(6)
        .width(Length::Fixed(130.0))
        .on_press(SettingsMessage::TimeRangePickerExpanded.into())
        .style(move |_theme, _status| button::Style {
            background: Some(iced::Background::Color(theme_colors.light_button)),
            text_color: theme_colors.light_text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            ..button::text(_theme, _status)
        });

    // 用 tooltip 包裹时间范围选择器
    let time_range_tooltip_text = text(app.i18n.t("settings.auto-change-time-range-tooltip"))
        .style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        });

    let time_range_trigger_with_tooltip = tooltip(
        time_range_trigger,
        time_range_tooltip_text,
        tooltip::Position::Top
    )
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.tooltip_bg_color)),
        border: Border {
            color: theme_colors.tooltip_border_color,
            width: TOOLTIP_BORDER_WIDTH,
            radius: Radius::from(TOOLTIP_BORDER_RADIUS),
        },
        ..Default::default()
    });

    // 创建时间范围选项（overlay）
    let time_range_options_content = column(time_range_options.iter().map(|option| {
        let is_selected = app.settings_state.auto_change_time_range == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::AutoChangeTimeRangeChanged(option.value).into())
            .style(move |_theme, _status| button::Style {
                background: if is_selected {
                    Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                } else {
                    Some(iced::Background::Color(Color::TRANSPARENT))
                },
                text_color: if is_selected {
                    Color::WHITE
                } else {
                    theme_colors.light_text
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(4.0),
                },
                ..button::text(_theme, _status)
            })
            .into()
    }))
    .spacing(2);

    let picker_content = container(time_range_options_content)
        .padding(8)
        .width(Length::Fixed(150.0))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.light_button)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(
        time_range_trigger_with_tooltip,
        opaque(picker_content),
        app.settings_state.time_range_picker_expanded,
    )
    .width(Length::Fill)
    .on_dismiss(SettingsMessage::TimeRangePickerDismiss.into())
    .alignment(drop_down::Alignment::Top)
    .into()
}
