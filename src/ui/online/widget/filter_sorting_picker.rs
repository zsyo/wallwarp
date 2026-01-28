// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::services::wallhaven::Sorting;
use crate::ui::AppMessage;
use crate::ui::online::{DisplayableSorting, OnlineMessage, OnlineState};
use crate::ui::style::*;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

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

    // 创建触发按钮（underlay）
    let sorting_underlay = row![
        text(current_sorting.display).size(14),
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

    let sorting_trigger = button(sorting_underlay)
        .padding(6)
        .width(Length::Fixed(100.0))
        .on_press(AppMessage::Online(OnlineMessage::SortingPickerExpanded))
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

    // 创建排序选项（overlay）
    let sorting_options_content = column(sorting_options.iter().map(|option| {
        let is_selected = state.sorting == option.value;
        button(text(option.display).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(AppMessage::Online(OnlineMessage::SortingChanged(option.value)))
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

    let picker_content = container(sorting_options_content)
        .padding(8)
        .width(Length::Fixed(120.0))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.light_button)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(sorting_trigger, opaque(picker_content), state.sorting_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Online(OnlineMessage::SortingPickerDismiss))
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
