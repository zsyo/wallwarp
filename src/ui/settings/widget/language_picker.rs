// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::settings::SettingsMessage;
use crate::ui::style::{COLOR_SELECTED_BLUE, PICK_LIST_WIDTH};
use crate::ui::{App, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建语言选择器
pub fn create_language_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_lang = app.i18n.current_lang.clone();

    // 创建触发按钮（underlay）
    let lang_underlay = row![
        text(current_lang.clone()).size(14),
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

    let lang_trigger = button(lang_underlay)
        .padding(6)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .on_press(SettingsMessage::LanguagePickerExpanded.into())
        .style(move |_theme, _status| button::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            text_color: theme_colors.light_text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(4.0),
            },
            ..button::text(_theme, _status)
        });

    // 创建语言选项（overlay）
    let lang_options_content = column(app.i18n.available_langs.iter().map(|lang| {
        let is_selected = app.i18n.current_lang == *lang;
        button(text(lang).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(SettingsMessage::LanguageSelected(lang.clone()).into())
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

    let picker_content = container(lang_options_content)
        .padding(8)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(lang_trigger, opaque(picker_content), app.settings_state.language_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(SettingsMessage::LanguagePickerDismiss.into())
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
