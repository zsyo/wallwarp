// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::App;
use super::AppMessage;
use super::common;
use crate::ui::style::{
    ABOUT_INFO_WIDTH, ABOUT_LOGO_SPACING, ABOUT_ROW_HEIGHT, BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN,
    BUTTON_COLOR_RED, BUTTON_SPACING, COLOR_SELECTED_BLUE, INPUT_HEIGHT, INPUT_PADDING, LOGO_DISPLAY_SIZE, LOGO_SIZE,
    PICK_LIST_WIDTH, PORT_INPUT_WIDTH, ROW_SPACING, SCROLL_PADDING, SECTION_PADDING, SECTION_SPACING,
    SECTION_TITLE_SIZE, SETTINGS_ROW_SPACING, TEXT_INPUT_SIZE, TOOLTIP_BG_COLOR, TOOLTIP_BORDER_COLOR,
    TOOLTIP_BORDER_RADIUS, TOOLTIP_BORDER_WIDTH,
};
use crate::utils::assets;
use crate::utils::config::CloseAction;
use iced::widget::{button, column, container, row, scrollable, text, text_input, toggler, tooltip};
use iced::{Alignment, Color, Length};
use iced_aw::{DropDown, drop_down};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProxyProtocol {
    Http,
    Socks5,
}

impl std::fmt::Display for ProxyProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyProtocol::Http => write!(f, "http"),
            ProxyProtocol::Socks5 => write!(f, "socks5"),
        }
    }
}

impl ProxyProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyProtocol::Http => "http",
            ProxyProtocol::Socks5 => "socks5",
        }
    }
}

impl FromStr for ProxyProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "socks5" => Ok(ProxyProtocol::Socks5),
            _ => Ok(ProxyProtocol::Http),
        }
    }
}

pub fn settings_view(app: &App) -> iced::Element<'_, AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(app.theme_config.get_theme());

    let system_config_section = common::create_config_section(
        app.i18n.t("settings.system-config"),
        vec![
            common::create_setting_row(
                app.i18n.t("settings.app-language"),
                create_language_picker(app),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.theme-color"),
                create_theme_picker(app),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-startup"),
                toggler(crate::utils::startup::is_auto_startup_enabled()).on_toggle(AppMessage::AutoStartupToggled),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.enable-logging"),
                toggler(app.config.global.enable_logging).on_toggle(AppMessage::LoggingToggled),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.close-action"),
                row![
                    iced::widget::radio(
                        app.i18n.t("close-action-options.ask"),
                        CloseAction::Ask,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    )
                    .style(move |theme: &iced::Theme, status| iced::widget::radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..iced::widget::radio::default(theme, status)
                    }),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.minimize-to-tray"),
                        CloseAction::MinimizeToTray,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    )
                    .style(move |theme: &iced::Theme, status| iced::widget::radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..iced::widget::radio::default(theme, status)
                    }),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.close-app"),
                        CloseAction::CloseApp,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    )
                    .style(move |theme: &iced::Theme, status| iced::widget::radio::Style {
                        text_color: Some(theme_colors.text),
                        background: iced::Background::Color(Color::TRANSPARENT),
                        ..iced::widget::radio::default(theme, status)
                    })
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.proxy"),
                row![
                    create_proxy_protocol_picker(app),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    text_input(&app.i18n.t("settings.proxy-address-placeholder"), &app.proxy_address)
                        .width(Length::FillPortion(2))
                        .align_x(Alignment::Center)
                        .padding(INPUT_PADDING)
                        .on_input(AppMessage::ProxyAddressChanged)
                        .style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                            background: iced::Background::Color(theme_colors.text_input_background),
                            border: iced::border::Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: iced::border::Radius::from(4.0),
                            },
                            icon: theme_colors.light_text_sub,
                            placeholder: theme_colors.light_text_sub,
                            value: theme_colors.light_text,
                            selection: theme_colors.text_input_selection_color,
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    container(
                        iced_aw::NumberInput::new(&app.proxy_port, 1..=65535, AppMessage::ProxyPortChanged)
                            .width(Length::Fill)
                            .align_x(Alignment::Start)
                            .padding(INPUT_PADDING)
                            .input_style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                                background: iced::Background::Color(theme_colors.text_input_background),
                                border: iced::border::Border {
                                    color: Color::TRANSPARENT,
                                    width: 0.0,
                                    radius: iced::border::Radius::from(4.0),
                                },
                                icon: theme_colors.light_text_sub,
                                placeholder: theme_colors.light_text_sub,
                                value: theme_colors.light_text,
                                selection: theme_colors.text_input_selection_color,
                            })
                            .style(move |_theme: &iced::Theme, _status| iced_aw::number_input::Style {
                                button_background: Some(iced::Background::Color(theme_colors.text_input_background)),
                                icon_color: theme_colors.light_text_sub,
                            })
                    )
                    .width(Length::Fixed(PORT_INPUT_WIDTH)),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    common::create_colored_button(
                        app.i18n.t("settings.proxy-save"),
                        BUTTON_COLOR_BLUE,
                        AppMessage::SaveProxy
                    )
                ]
                .width(Length::FillPortion(2))
                .spacing(0),
                &app.theme_config,
            ),
        ],
        &app.theme_config,
    );

    let data_config_section = common::create_config_section(
        app.i18n.t("settings.data-config"),
        vec![
            create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.data-path"),
                &common::get_absolute_path(&app.config.data.data_path),
                AppMessage::DataPathSelected("SELECT_DATA_PATH".to_string()),
                AppMessage::OpenPath("data".to_string()),
                AppMessage::ShowPathClearConfirmation("data".to_string()),
                AppMessage::RestoreDefaultPath("data".to_string()),
                theme_colors,
            ),
            create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.cache-path"),
                &common::get_absolute_path(&app.config.data.cache_path),
                AppMessage::CachePathSelected("SELECT_CACHE_PATH".to_string()),
                AppMessage::OpenPath("cache".to_string()),
                AppMessage::ShowPathClearConfirmation("cache".to_string()),
                AppMessage::RestoreDefaultPath("cache".to_string()),
                theme_colors,
            ),
            create_logs_path_row(&app.i18n, app.i18n.t("settings.logs-path"), theme_colors),
        ],
        &app.theme_config,
    );

    let api_config_section = common::create_config_section(
        app.i18n.t("settings.api-config"),
        vec![common::create_setting_row(
            app.i18n.t("settings.wallhaven-api-key"),
            row![
                text_input(
                    &app.i18n.t("settings.wallhaven-api-key-placeholder"),
                    &app.wallhaven_api_key
                )
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(AppMessage::WallhavenApiKeyChanged)
                .padding(INPUT_PADDING)
                .style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(theme_colors.text_input_background),
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    icon: theme_colors.light_text_sub,
                    placeholder: theme_colors.light_text_sub,
                    value: theme_colors.light_text,
                    selection: theme_colors.text_input_selection_color,
                }),
                container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
                common::create_colored_button(
                    app.i18n.t("settings.save"),
                    BUTTON_COLOR_BLUE,
                    AppMessage::SaveWallhavenApiKey
                )
            ]
            .width(Length::FillPortion(3))
            .spacing(0),
            &app.theme_config,
        )],
        &app.theme_config,
    );

    let wallpaper_config_section = common::create_config_section(
        app.i18n.t("settings.wallpaper-config"),
        vec![
            common::create_setting_row(
                app.i18n.t("settings.wallpaper-mode"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.crop"),
                        crate::utils::config::WallpaperMode::Crop,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.crop-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.fit"),
                        crate::utils::config::WallpaperMode::Fit,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.fit-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.stretch"),
                        crate::utils::config::WallpaperMode::Stretch,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.stretch-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.tile"),
                        crate::utils::config::WallpaperMode::Tile,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.tile-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.center"),
                        crate::utils::config::WallpaperMode::Center,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.center-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.span"),
                        crate::utils::config::WallpaperMode::Span,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.span-tooltip"),
                        theme_colors
                    ),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-change-mode"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.online"),
                        crate::utils::config::WallpaperAutoChangeMode::Online,
                        Some(app.auto_change_mode),
                        |mode| AppMessage::AutoChangeModeSelected(mode),
                        app.i18n.t("auto-change-mode-options.online-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.local"),
                        crate::utils::config::WallpaperAutoChangeMode::Local,
                        Some(app.auto_change_mode),
                        |mode| AppMessage::AutoChangeModeSelected(mode),
                        app.i18n.t("auto-change-mode-options.local-tooltip"),
                        theme_colors
                    ),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-change-interval"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.off"),
                        crate::utils::config::WallpaperAutoChangeInterval::Off,
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.off-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.ten-min"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(10),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.ten-min-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.thirty-min"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(30),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.thirty-min-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.one-hour"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(60),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.one-hour-tooltip"),
                        theme_colors
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.two-hour"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(120),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.two-hour-tooltip"),
                        theme_colors
                    ),
                    tooltip(
                        container(
                            row![
                                iced::widget::radio(
                                    app.i18n.t("auto-change-interval-options.custom"),
                                    crate::utils::config::WallpaperAutoChangeInterval::Custom(
                                        app.custom_interval_minutes
                                    ),
                                    Some(app.auto_change_interval.clone()),
                                    |interval| {
                                        if let crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes) =
                                            interval
                                        {
                                            AppMessage::AutoChangeIntervalSelected(
                                                crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes),
                                            )
                                        } else {
                                            AppMessage::AutoChangeIntervalSelected(interval)
                                        }
                                    }
                                )
                                .style(move |theme: &iced::Theme, status| {
                                    iced::widget::radio::Style {
                                        text_color: Some(theme_colors.text),
                                        background: iced::Background::Color(Color::TRANSPARENT),
                                        ..iced::widget::radio::default(theme, status)
                                    }
                                }),
                                container(
                                    iced_aw::NumberInput::new(&app.custom_interval_minutes, 1..=1440, |minutes| {
                                        AppMessage::CustomIntervalMinutesChanged(minutes)
                                    })
                                    .width(Length::Fill)
                                    .padding(INPUT_PADDING)
                                    .input_style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                                        background: iced::Background::Color(theme_colors.text_input_background),
                                        border: iced::border::Border {
                                            color: Color::TRANSPARENT,
                                            width: 0.0,
                                            radius: iced::border::Radius::from(4.0),
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
                                    )
                                )
                                .width(Length::Fixed(80.0)),
                            ]
                            .spacing(ROW_SPACING)
                            .align_y(Alignment::Center)
                        ),
                        text(app.i18n.t("auto-change-interval-options.custom-tooltip")),
                        tooltip::Position::Top
                    )
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(TOOLTIP_BG_COLOR)),
                        border: iced::border::Border {
                            color: TOOLTIP_BORDER_COLOR,
                            width: TOOLTIP_BORDER_WIDTH,
                            radius: iced::border::Radius::from(TOOLTIP_BORDER_RADIUS),
                        },
                        ..Default::default()
                    }),
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-change-query"),
                row![
                    text_input(
                        &app.i18n.t("settings.auto-change-query-placeholder"),
                        &app.auto_change_query
                    )
                    .width(Length::Fixed(400.0))
                    .align_x(Alignment::Center)
                    .padding(INPUT_PADDING)
                    .on_input(|query| AppMessage::AutoChangeQueryChanged(query))
                    .style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                        background: iced::Background::Color(theme_colors.text_input_background),
                        border: iced::border::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: iced::border::Radius::from(4.0),
                        },
                        icon: theme_colors.light_text_sub,
                        placeholder: theme_colors.light_text_sub,
                        value: theme_colors.light_text,
                        selection: theme_colors.text_input_selection_color,
                    }),
                    common::create_colored_button(
                        app.i18n.t("settings.save"),
                        BUTTON_COLOR_BLUE,
                        AppMessage::SaveAutoChangeQuery
                    )
                ]
                .spacing(ROW_SPACING),
                &app.theme_config,
            ),
        ],
        &app.theme_config,
    );

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let about_config_section = container(
        column!(
            text(app.i18n.t("settings.about-config"))
                .size(SECTION_TITLE_SIZE)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .style(move |_theme: &iced::Theme| text::Style {
                    color: Some(theme_colors.text),
                }),
            row![
                container(
                    column![
                        common::create_info_row(
                            app.i18n.t("settings.about-name"),
                            app.i18n.t("app-title"),
                            theme_colors
                        ),
                        common::create_info_row(
                            app.i18n.t("settings.about-version"),
                            env!("CARGO_PKG_VERSION").to_string(),
                            theme_colors
                        ),
                        create_about_link_row(
                            app.i18n.t("settings.about-author"),
                            "zsyo",
                            "https://github.com/zsyo",
                            theme_colors
                        ),
                        create_about_link_row(
                            app.i18n.t("settings.about-repo"),
                            "https://github.com/zsyo/wallwarp",
                            "https://github.com/zsyo/wallwarp",
                            theme_colors
                        ),
                    ]
                    .spacing(ROW_SPACING)
                )
                .width(Length::Fixed(ABOUT_INFO_WIDTH)),
                container(iced::widget::Space::new()).width(Length::Fill),
                iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                    .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                    .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
                container(iced::widget::Space::new()).width(Length::Fixed(ABOUT_LOGO_SPACING)),
            ]
            .width(Length::Fill)
            .spacing(ROW_SPACING)
        )
        .padding(SECTION_PADDING)
        .spacing(SECTION_SPACING),
    )
    .width(Length::Fill)
    .style(common::create_bordered_container_style_with_bg(&app.theme_config));

    scrollable(
        column![
            system_config_section,
            data_config_section,
            api_config_section,
            wallpaper_config_section,
            about_config_section,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(SCROLL_PADDING)
        .spacing(SETTINGS_ROW_SPACING),
    )
    .height(Length::Fill)
    .id(iced::widget::Id::new("settings_scroll"))
    .into()
}

fn create_path_config_row<'a>(
    i18n: &crate::i18n::I18n,
    label: String,
    path: &str,
    select_msg: AppMessage,
    open_msg: AppMessage,
    clear_msg: AppMessage,
    restore_msg: AppMessage,
    theme_colors: crate::ui::style::ThemeColors,
) -> iced::Element<'a, AppMessage> {
    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        row![
            iced::widget::text_input("", path)
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(|_| AppMessage::DataPathSelected("".to_string()))
                .padding(INPUT_PADDING)
                .style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(theme_colors.text_input_background),
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    icon: theme_colors.light_text_sub,
                    placeholder: theme_colors.light_text_sub,
                    value: theme_colors.light_text,
                    selection: theme_colors.text_input_selection_color,
                }),
            container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(i18n.t("settings.select-path"), BUTTON_COLOR_BLUE, select_msg),
            container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(i18n.t("settings.open-path"), BUTTON_COLOR_GREEN, open_msg),
            container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(i18n.t("settings.clear-path"), BUTTON_COLOR_RED, clear_msg),
            container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(i18n.t("settings.restore-default"), BUTTON_COLOR_GRAY, restore_msg),
        ]
        .width(Length::FillPortion(4))
        .spacing(0),
    ]
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}

fn create_logs_path_row<'a>(
    i18n: &crate::i18n::I18n,
    label: String,
    theme_colors: crate::ui::style::ThemeColors,
) -> iced::Element<'a, AppMessage> {
    let logs_path = common::get_absolute_path("logs");

    row![
        text(label)
            .width(Length::FillPortion(1))
            .style(move |_theme: &iced::Theme| text::Style {
                color: Some(theme_colors.text),
            }),
        row![
            iced::widget::text_input("", &logs_path)
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .padding(INPUT_PADDING)
                .style(move |_theme: &iced::Theme, _status| iced::widget::text_input::Style {
                    background: iced::Background::Color(theme_colors.text_input_background),
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    icon: theme_colors.light_text_sub,
                    placeholder: theme_colors.light_text_sub,
                    value: theme_colors.light_text,
                    selection: theme_colors.text_input_selection_color,
                }),
            container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.open-path"),
                BUTTON_COLOR_GREEN,
                AppMessage::OpenLogsPath
            )
            .width(Length::Fixed(210.0)),
        ]
        .width(Length::FillPortion(4))
        .spacing(0),
    ]
    .height(Length::Fixed(INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(ROW_SPACING)
    .into()
}

fn create_about_link_row<'a>(
    label: String,
    text_value: &'a str,
    url: &'a str,
    theme_colors: crate::ui::style::ThemeColors,
) -> iced::Element<'a, AppMessage> {
    row![
        text(label).style(move |_theme: &iced::Theme| text::Style {
            color: Some(theme_colors.text),
        }),
        button(text(text_value).width(Length::Fill).align_x(Alignment::Center))
            .padding(0)
            .style(move |_theme: &iced::Theme, _status| {
                let palette = _theme.extended_palette();
                iced::widget::button::Style {
                    text_color: palette.primary.base.color,
                    ..iced::widget::button::text(_theme, _status)
                }
            })
            .on_press(AppMessage::OpenUrl(url.to_string())),
    ]
    .height(Length::Fixed(ABOUT_ROW_HEIGHT))
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .spacing(ROW_SPACING)
    .into()
}

/// 创建语言选择器
fn create_language_picker<'a>(app: &'a super::App) -> iced::Element<'a, super::AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(app.theme_config.get_theme());
    let current_lang = app.i18n.current_lang.clone();

    // 创建触发按钮（underlay）
    let lang_underlay = row![
        text(current_lang.clone()).size(14),
        iced::widget::Space::new().width(Length::Fill),
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
        .on_press(super::AppMessage::LanguagePickerExpanded)
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            text_color: theme_colors.light_text,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建语言选项（overlay）
    let lang_options_content = column(app.i18n.available_langs.iter().map(|lang| {
        let is_selected = app.i18n.current_lang == *lang;
        button(text(lang).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(super::AppMessage::LanguageSelected(lang.clone()))
            .style(move |_theme, _status| iced::widget::button::Style {
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
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            })
            .into()
    }))
    .spacing(2);

    let picker_content = container(lang_options_content)
        .padding(8)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(
        lang_trigger,
        iced::widget::opaque(picker_content),
        app.language_picker_expanded,
    )
    .width(Length::Fill)
    .on_dismiss(super::AppMessage::LanguagePickerDismiss)
    .alignment(drop_down::Alignment::Bottom)
    .into()
}

/// 创建主题选择器
fn create_theme_picker<'a>(app: &'a super::App) -> iced::Element<'a, super::AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(app.theme_config.get_theme());
    let current_theme = app.config.global.theme.clone();

    // 根据当前主题获取对应的翻译文本
    let current_theme_text = match current_theme {
        crate::utils::config::Theme::Dark => app.i18n.t("theme-options.dark"),
        crate::utils::config::Theme::Light => app.i18n.t("theme-options.light"),
        crate::utils::config::Theme::Auto => app.i18n.t("theme-options.auto"),
    };

    // 创建触发按钮（underlay）
    let theme_underlay = row![
        text(current_theme_text).size(14),
        iced::widget::Space::new().width(Length::Fill),
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
        .on_press(super::AppMessage::ThemePickerExpanded)
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            text_color: theme_colors.light_text,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建主题选项（overlay）
    let theme_options_content = {
        let theme_colors = theme_colors.clone();
        let current_theme = app.config.global.theme;

        column([
            button(text(app.i18n.t("theme-options.dark")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(super::AppMessage::ThemeSelected(crate::utils::config::Theme::Dark))
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: if current_theme == crate::utils::config::Theme::Dark {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == crate::utils::config::Theme::Dark {
                        Color::WHITE
                    } else {
                        theme_colors.light_text
                    },
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                })
                .into(),
            button(text(app.i18n.t("theme-options.light")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(super::AppMessage::ThemeSelected(crate::utils::config::Theme::Light))
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: if current_theme == crate::utils::config::Theme::Light {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == crate::utils::config::Theme::Light {
                        Color::WHITE
                    } else {
                        theme_colors.light_text
                    },
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
                })
                .into(),
            button(text(app.i18n.t("theme-options.auto")).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(super::AppMessage::ThemeSelected(crate::utils::config::Theme::Auto))
                .style(move |_theme, _status| iced::widget::button::Style {
                    background: if current_theme == crate::utils::config::Theme::Auto {
                        Some(iced::Background::Color(COLOR_SELECTED_BLUE))
                    } else {
                        Some(iced::Background::Color(Color::TRANSPARENT))
                    },
                    text_color: if current_theme == crate::utils::config::Theme::Auto {
                        Color::WHITE
                    } else {
                        theme_colors.light_text
                    },
                    border: iced::border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..iced::widget::button::text(_theme, _status)
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
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(
        theme_trigger,
        iced::widget::opaque(picker_content),
        app.theme_picker_expanded,
    )
    .width(Length::Fill)
    .on_dismiss(super::AppMessage::ThemePickerDismiss)
    .alignment(drop_down::Alignment::Bottom)
    .into()
}

/// 创建代理协议选择器
fn create_proxy_protocol_picker<'a>(app: &'a super::App) -> iced::Element<'a, super::AppMessage> {
    let theme_colors = crate::ui::style::ThemeColors::from_theme(app.theme_config.get_theme());
    let current_protocol = ProxyProtocol::from_str(&app.proxy_protocol).ok();

    // 创建触发按钮（underlay）
    let protocol_text = current_protocol
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| "http".to_string());
    let protocol_underlay = row![
        text(protocol_text).size(14),
        iced::widget::Space::new().width(Length::Fill),
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

    let protocol_trigger = button(protocol_underlay)
        .padding(6)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .on_press(super::AppMessage::ProxyProtocolPickerExpanded)
        .style(move |_theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            text_color: theme_colors.light_text,
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(4.0),
            },
            ..iced::widget::button::text(_theme, _status)
        });

    // 创建代理协议选项（overlay）
    let protocol_options_content = column([ProxyProtocol::Http, ProxyProtocol::Socks5].iter().map(|protocol| {
        let is_selected = current_protocol == Some(*protocol);
        button(text(protocol.as_str()).size(14))
            .padding(6)
            .width(Length::Fill)
            .on_press(super::AppMessage::ProxyProtocolChanged(protocol.as_str().to_string()))
            .style(move |_theme, _status| iced::widget::button::Style {
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
                border: iced::border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(4.0),
                },
                ..iced::widget::button::text(_theme, _status)
            })
            .into()
    }))
    .spacing(2);

    let picker_content = container(protocol_options_content)
        .padding(8)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(theme_colors.settings_dropdown_bg)),
            border: iced::border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(8.0),
            },
            ..Default::default()
        });

    DropDown::new(
        protocol_trigger,
        iced::widget::opaque(picker_content),
        app.proxy_protocol_picker_expanded,
    )
    .width(Length::Fill)
    .on_dismiss(super::AppMessage::ProxyProtocolPickerDismiss)
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
