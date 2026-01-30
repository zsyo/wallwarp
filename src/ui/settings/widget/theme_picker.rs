// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{COLOR_SELECTED_BLUE, PICK_LIST_WIDTH};
use crate::ui::{App, AppMessage};
use crate::utils::config::Theme;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, opaque, row, text};
use iced::{Alignment, Color, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建主题选择器
pub fn create_theme_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    let current_theme = app.config.global.theme.clone();

    // 根据当前主题获取对应的翻译文本
    let current_theme_text = match current_theme {
        Theme::Dark => app.i18n.t("theme-options.dark"),
        Theme::Light => app.i18n.t("theme-options.light"),
        Theme::Auto => app.i18n.t("theme-options.auto"),
    };

    // 创建触发按钮（underlay）
    let theme_underlay = row![
        text(current_theme_text).size(14),
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

    let theme_trigger = button(theme_underlay)
        .padding(6)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .on_press(SettingsMessage::ThemePickerExpanded.into())
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

    // 创建主题选项（overlay）
    let theme_options_content = {
        let theme_colors = theme_colors.clone();
        let current_theme = app.config.global.theme;

        column([
            button(text(app.i18n.t("theme-options.dark")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(MainMessage::ThemeSelected(Theme::Dark).into())
                .style(move |_theme, _status| button::Style {
                    background: if current_theme == Theme::Dark {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == Theme::Dark {
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
                .into(),
            button(text(app.i18n.t("theme-options.light")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(MainMessage::ThemeSelected(Theme::Light).into())
                .style(move |_theme, _status| button::Style {
                    background: if current_theme == Theme::Light {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == Theme::Light {
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
                .into(),
            button(text(app.i18n.t("theme-options.auto")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(MainMessage::ThemeSelected(Theme::Auto).into())
                .style(move |_theme, _status| button::Style {
                    background: if current_theme == Theme::Auto {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == Theme::Auto {
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
                .into(),
        ])
        .spacing(2)
    };

    let picker_content = container(theme_options_content)
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

    DropDown::new(theme_trigger, opaque(picker_content), app.theme_picker_expanded)
        .width(Length::Fill)
        .on_dismiss(SettingsMessage::ThemePickerDismiss.into())
        .alignment(drop_down::Alignment::Bottom)
        .into()
}
