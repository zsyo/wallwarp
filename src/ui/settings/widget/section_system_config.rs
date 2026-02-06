// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::ThemeColors;
use crate::ui::style::{BUTTON_COLOR_BLUE, INPUT_PADDING, PORT_INPUT_WIDTH, ROW_SPACING};
use crate::ui::{App, AppMessage};
use crate::utils::config::CloseAction;
use crate::utils::startup;
use iced::border::{Border, Radius};
use iced::widget::{Space, container, radio, row, text_input, toggler};
use iced::{Alignment, Color, Element, Length};

/// 创建系统配置区块
pub fn create_system_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    super::create_config_section(
        app.i18n.t("settings.system-config"),
        vec![
            super::create_setting_row(
                app.i18n.t("settings.app-language"),
                super::create_language_picker(app),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.theme-color"),
                super::create_theme_picker(app),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.auto-startup"),
                toggler(startup::is_auto_startup_enabled())
                    .on_toggle(|state| SettingsMessage::AutoStartupToggled(state).into()),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.enable-logging"),
                toggler(app.config.global.enable_logging)
                    .on_toggle(|state| SettingsMessage::LoggingToggled(state).into()),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.close-action"),
                row![
                    radio(
                        app.i18n.t("close-action-options.ask"),
                        CloseAction::Ask,
                        Some(app.config.global.close_action.clone()),
                        |act| SettingsMessage::CloseActionSelected(act).into()
                    )
                    .style(move |theme: &iced::Theme, status| radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..radio::default(theme, status)
                    }),
                    radio(
                        app.i18n.t("close-action-options.minimize-to-tray"),
                        CloseAction::MinimizeToTray,
                        Some(app.config.global.close_action.clone()),
                        |act| SettingsMessage::CloseActionSelected(act).into()
                    )
                    .style(move |theme: &iced::Theme, status| radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..radio::default(theme, status)
                    }),
                    radio(
                        app.i18n.t("close-action-options.close-app"),
                        CloseAction::CloseApp,
                        Some(app.config.global.close_action.clone()),
                        |act| SettingsMessage::CloseActionSelected(act).into()
                    )
                    .style(move |theme: &iced::Theme, status| radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..radio::default(theme, status)
                    })
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            super::create_setting_row(
                app.i18n.t("settings.proxy"),
                row![
                    toggler(app.settings_state.proxy_enabled)
                        .on_toggle(|state| SettingsMessage::ProxyToggled(state).into()),
                    container(Space::new()).width(Length::Fixed(ROW_SPACING)),
                    super::create_proxy_protocol_picker(app),
                    container(Space::new()).width(Length::Fixed(ROW_SPACING)),
                    text_input(
                        &app.i18n.t("settings.proxy-address-placeholder"),
                        &app.settings_state.proxy_address
                    )
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::Center)
                    .padding(INPUT_PADDING)
                    .on_input(if app.settings_state.proxy_enabled {
                        |s| SettingsMessage::ProxyAddressChanged(s).into()
                    } else {
                        |_s| SettingsMessage::ProxyToggled(false).into()
                    })
                    .style(move |_theme: &iced::Theme, _status| text_input::Style {
                        background: iced::Background::Color(if app.settings_state.proxy_enabled {
                            theme_colors.text_input_background
                        } else {
                            Color {
                                r: theme_colors.text_input_background.r,
                                g: theme_colors.text_input_background.g,
                                b: theme_colors.text_input_background.b,
                                a: 0.3,
                            }
                        }),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::from(4.0),
                        },
                        icon: if app.settings_state.proxy_enabled {
                            theme_colors.light_text_sub
                        } else {
                            Color {
                                r: theme_colors.light_text_sub.r,
                                g: theme_colors.light_text_sub.g,
                                b: theme_colors.light_text_sub.b,
                                a: 0.3,
                            }
                        },
                        placeholder: if app.settings_state.proxy_enabled {
                            theme_colors.light_text_sub
                        } else {
                            Color {
                                r: theme_colors.light_text_sub.r,
                                g: theme_colors.light_text_sub.g,
                                b: theme_colors.light_text_sub.b,
                                a: 0.3,
                            }
                        },
                        value: if app.settings_state.proxy_enabled {
                            theme_colors.light_text
                        } else {
                            Color {
                                r: theme_colors.light_text.r,
                                g: theme_colors.light_text.g,
                                b: theme_colors.light_text.b,
                                a: 0.3,
                            }
                        },
                        selection: theme_colors.text_input_selection_color,
                    }),
                    container(Space::new()).width(Length::Fixed(ROW_SPACING)),
                    {
                        let proxy_enabled = app.settings_state.proxy_enabled;
                        container(
                            iced_aw::NumberInput::new(&app.settings_state.proxy_port, 1..=65535, move |n| {
                                if proxy_enabled {
                                    SettingsMessage::ProxyPortChanged(n).into()
                                } else {
                                    SettingsMessage::ProxyToggled(false).into()
                                }
                            })
                            .width(Length::Fill)
                            .align_x(Alignment::Start)
                            .padding(INPUT_PADDING)
                            .input_style(move |_theme: &iced::Theme, _status| text_input::Style {
                                background: iced::Background::Color(if app.settings_state.proxy_enabled {
                                    theme_colors.text_input_background
                                } else {
                                    Color {
                                        r: theme_colors.text_input_background.r,
                                        g: theme_colors.text_input_background.g,
                                        b: theme_colors.text_input_background.b,
                                        a: 0.3,
                                    }
                                }),
                                border: Border {
                                    color: Color::TRANSPARENT,
                                    width: 0.0,
                                    radius: Radius::from(4.0),
                                },
                                icon: if app.settings_state.proxy_enabled {
                                    theme_colors.light_text_sub
                                } else {
                                    Color {
                                        r: theme_colors.light_text_sub.r,
                                        g: theme_colors.light_text_sub.g,
                                        b: theme_colors.light_text_sub.b,
                                        a: 0.3,
                                    }
                                },
                                placeholder: if app.settings_state.proxy_enabled {
                                    theme_colors.light_text_sub
                                } else {
                                    Color {
                                        r: theme_colors.light_text_sub.r,
                                        g: theme_colors.light_text_sub.g,
                                        b: theme_colors.light_text_sub.b,
                                        a: 0.3,
                                    }
                                },
                                value: if app.settings_state.proxy_enabled {
                                    theme_colors.light_text
                                } else {
                                    Color {
                                        r: theme_colors.light_text.r,
                                        g: theme_colors.light_text.g,
                                        b: theme_colors.light_text.b,
                                        a: 0.3,
                                    }
                                },
                                selection: theme_colors.text_input_selection_color,
                            })
                            .style(move |_theme: &iced::Theme, _status| iced_aw::number_input::Style {
                                button_background: Some(iced::Background::Color(if app.settings_state.proxy_enabled {
                                    theme_colors.text_input_background
                                } else {
                                    Color {
                                        r: theme_colors.text_input_background.r,
                                        g: theme_colors.text_input_background.g,
                                        b: theme_colors.text_input_background.b,
                                        a: 0.3,
                                    }
                                })),
                                icon_color: if app.settings_state.proxy_enabled {
                                    theme_colors.light_text_sub
                                } else {
                                    Color {
                                        r: theme_colors.light_text_sub.r,
                                        g: theme_colors.light_text_sub.g,
                                        b: theme_colors.light_text_sub.b,
                                        a: 0.3,
                                    }
                                },
                            }),
                        )
                        .width(Length::Fixed(PORT_INPUT_WIDTH))
                    },
                    container(Space::new()).width(Length::Fixed(ROW_SPACING)),
                    common::create_colored_button(
                        app.i18n.t("settings.proxy-save"),
                        BUTTON_COLOR_BLUE,
                        SettingsMessage::SaveProxy.into()
                    )
                ]
                .width(Length::FillPortion(2))
                .align_y(Alignment::Center)
                .spacing(0),
                &app.theme_config,
            ),
        ],
        &app.theme_config,
    )
}
