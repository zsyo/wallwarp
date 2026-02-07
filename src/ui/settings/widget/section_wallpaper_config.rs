// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, INPUT_PADDING, ROW_SPACING, TOOLTIP_BG_COLOR, TOOLTIP_BORDER_COLOR, TOOLTIP_BORDER_RADIUS,
    TOOLTIP_BORDER_WIDTH,
};
use crate::ui::{App, AppMessage};
use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};
use iced::border::{Border, Radius};
use iced::widget::{container, radio, row, text, text_input, tooltip};
use iced::{Alignment, Color, Element, Length};

/// 创建壁纸配置区块
pub fn create_wallpaper_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    super::create_config_section(
        app.i18n.t("settings.wallpaper-config"),
        vec![
            super::create_setting_row(
                app.i18n.t("settings.wallpaper-mode"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.crop"),
                        WallpaperMode::Crop,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.crop-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.fit"),
                        WallpaperMode::Fit,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.fit-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.stretch"),
                        WallpaperMode::Stretch,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.stretch-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.tile"),
                        WallpaperMode::Tile,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.tile-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.center"),
                        WallpaperMode::Center,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.center-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.span"),
                        WallpaperMode::Span,
                        Some(app.settings_state.wallpaper_mode),
                        |mode| SettingsMessage::WallpaperModeSelected(mode).into(),
                        app.i18n.t("wallpaper-mode-options.span-tooltip"),
                        theme_colors
                    ),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.auto-change-mode"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.online"),
                        WallpaperAutoChangeMode::Online,
                        Some(app.settings_state.auto_change_mode),
                        |mode| SettingsMessage::AutoChangeModeSelected(mode).into(),
                        app.i18n.t("auto-change-mode-options.online-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.local"),
                        WallpaperAutoChangeMode::Local,
                        Some(app.settings_state.auto_change_mode),
                        |mode| SettingsMessage::AutoChangeModeSelected(mode).into(),
                        app.i18n.t("auto-change-mode-options.local-tooltip"),
                        theme_colors
                    ),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.auto-change-interval"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.off"),
                        WallpaperAutoChangeInterval::Off,
                        Some(app.settings_state.auto_change_interval.clone()),
                        |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
                        app.i18n.t("auto-change-interval-options.off-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.ten-min"),
                        WallpaperAutoChangeInterval::Minutes(10),
                        Some(app.settings_state.auto_change_interval.clone()),
                        |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
                        app.i18n.t("auto-change-interval-options.ten-min-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.thirty-min"),
                        WallpaperAutoChangeInterval::Minutes(30),
                        Some(app.settings_state.auto_change_interval.clone()),
                        |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
                        app.i18n.t("auto-change-interval-options.thirty-min-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.one-hour"),
                        WallpaperAutoChangeInterval::Minutes(60),
                        Some(app.settings_state.auto_change_interval.clone()),
                        |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
                        app.i18n.t("auto-change-interval-options.one-hour-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.two-hour"),
                        WallpaperAutoChangeInterval::Minutes(120),
                        Some(app.settings_state.auto_change_interval.clone()),
                        |interval| SettingsMessage::AutoChangeIntervalSelected(interval).into(),
                        app.i18n.t("auto-change-interval-options.two-hour-tooltip"),
                        theme_colors
                    ),
                    tooltip(
                        container(
                            row![
                                iced::widget::radio(
                                    app.i18n.t("auto-change-interval-options.custom"),
                                    WallpaperAutoChangeInterval::Custom(app.settings_state.custom_interval_minutes),
                                    Some(app.settings_state.auto_change_interval.clone()),
                                    |interval| {
                                        if let WallpaperAutoChangeInterval::Custom(minutes) = interval {
                                            SettingsMessage::AutoChangeIntervalSelected(
                                                WallpaperAutoChangeInterval::Custom(minutes),
                                            )
                                            .into()
                                        } else {
                                            SettingsMessage::AutoChangeIntervalSelected(interval).into()
                                        }
                                    }
                                )
                                .style(move |theme: &iced::Theme, status| {
                                    radio::Style {
                                        text_color: Some(theme_colors.text),
                                        background: iced::Background::Color(Color::TRANSPARENT),
                                        ..radio::default(theme, status)
                                    }
                                }),
                                container(
                                    row![
                                        iced_aw::NumberInput::new(
                                            &app.settings_state.custom_interval_minutes,
                                            1..=1440,
                                            |minutes| { SettingsMessage::CustomIntervalMinutesChanged(minutes).into() }
                                        )
                                        .width(Length::Fill)
                                        .padding(INPUT_PADDING)
                                        .input_style(move |_theme: &iced::Theme, _status| text_input::Style {
                                            background: iced::Background::Color(theme_colors.text_input_background),
                                            border: Border {
                                                color: Color::TRANSPARENT,
                                                width: 0.0,
                                                radius: Radius::from(4.0),
                                            },
                                            icon: theme_colors.light_text_sub,
                                            placeholder: theme_colors.light_text_sub,
                                            value: theme_colors.light_text,
                                            selection: theme_colors.text_input_selection_color,
                                        })
                                        .style(
                                            move |_theme: &iced::Theme, _status| iced_aw::number_input::Style {
                                                button_background: Some(iced::Background::Color(
                                                    theme_colors.text_input_background
                                                )),
                                                icon_color: theme_colors.light_text_sub,
                                            }
                                        ),
                                        text(app.i18n.t("settings.minutes"))
                                            .size(14)
                                            .color(theme_colors.light_text),
                                    ]
                                    .spacing(4)
                                    .align_y(Alignment::Center)
                                )
                                .width(Length::Fixed(120.0)),
                            ]
                            .spacing(ROW_SPACING)
                            .align_y(Alignment::Center)
                        ),
                        text(app.i18n.t("auto-change-interval-options.custom-tooltip")),
                        tooltip::Position::Top
                    )
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
                        border: Border {
                            color: TOOLTIP_BORDER_COLOR,
                            width: TOOLTIP_BORDER_WIDTH,
                            radius: Radius::from(TOOLTIP_BORDER_RADIUS),
                        },
                        ..Default::default()
                    }),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.auto-change-online-config"),
                row![
                    super::create_sorting_picker(app, theme_colors),
                    super::create_time_range_picker(app, theme_colors),
                    row![
                        text_input(
                            &app.i18n.t("settings.auto-change-query-placeholder"),
                            &app.settings_state.auto_change_query
                        )
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                        .padding(INPUT_PADDING)
                        .on_input(|query| SettingsMessage::AutoChangeQueryChanged(query).into())
                        .style(move |_theme: &iced::Theme, _status| text_input::Style {
                            background: iced::Background::Color(theme_colors.text_input_background),
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: Radius::from(4.0),
                            },
                            icon: theme_colors.light_text_sub,
                            placeholder: theme_colors.light_text_sub,
                            value: theme_colors.light_text,
                            selection: theme_colors.text_input_selection_color,
                        }),
                        common::create_colored_button(
                            app.i18n.t("settings.save"),
                            BUTTON_COLOR_BLUE,
                            SettingsMessage::SaveAutoChangeQuery.into()
                        )
                    ]
                    .spacing(ROW_SPACING / 2.0)
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
        ],
        &app.theme_config,
    )
}
