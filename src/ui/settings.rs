use super::App;
use super::AppMessage;
use super::common;
use crate::ui::style::{
    ABOUT_INFO_WIDTH, ABOUT_LOGO_SPACING, ABOUT_ROW_HEIGHT, BUTTON_COLOR_BLUE, BUTTON_COLOR_GRAY, BUTTON_COLOR_GREEN, BUTTON_COLOR_RED, BUTTON_SPACING,
    INPUT_HEIGHT, INPUT_PADDING, LOGO_DISPLAY_SIZE, LOGO_SIZE, PICK_LIST_WIDTH, PORT_INPUT_WIDTH, ROW_SPACING, SCROLL_PADDING, SECTION_PADDING,
    SECTION_SPACING, SECTION_TITLE_SIZE, TEXT_INPUT_SIZE, TOOLTIP_BG_COLOR, TOOLTIP_BORDER_COLOR, TOOLTIP_BORDER_RADIUS, TOOLTIP_BORDER_WIDTH,
};
use crate::utils::assets;
use crate::utils::config::CloseAction;
use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, toggler, tooltip};
use iced::{Alignment, Length};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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
    let system_config_section = common::create_config_section(
        app.i18n.t("settings.system-config"),
        vec![
            common::create_setting_row(
                app.i18n.t("settings.app-language"),
                pick_list(&app.i18n.available_langs[..], Some(app.i18n.current_lang.clone()), AppMessage::LanguageSelected)
                    .width(Length::Fixed(PICK_LIST_WIDTH)),
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-startup"),
                toggler(crate::utils::startup::is_auto_startup_enabled()).on_toggle(AppMessage::AutoStartupToggled),
            ),
            common::create_setting_row(
                app.i18n.t("settings.close-action"),
                row![
                    iced::widget::radio(
                        app.i18n.t("close-action-options.ask"),
                        CloseAction::Ask,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    ),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.minimize-to-tray"),
                        CloseAction::MinimizeToTray,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    ),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.close-app"),
                        CloseAction::CloseApp,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    )
                ]
                .spacing(ROW_SPACING),
            ),
            common::create_setting_row(
                app.i18n.t("settings.proxy"),
                row![
                    pick_list(
                        [ProxyProtocol::Http, ProxyProtocol::Socks5],
                        ProxyProtocol::from_str(&app.proxy_protocol).ok(),
                        |protocol| AppMessage::ProxyProtocolChanged(protocol.as_str().to_string())
                    )
                    .width(Length::Fixed(PICK_LIST_WIDTH)),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    text_input(&app.i18n.t("settings.proxy-address-placeholder"), &app.proxy_address)
                        .width(Length::FillPortion(2))
                        .align_x(Alignment::Center)
                        .padding(INPUT_PADDING)
                        .on_input(AppMessage::ProxyAddressChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    text_input(&app.i18n.t("settings.proxy-port-placeholder"), &app.proxy_port)
                        .width(Length::Fixed(PORT_INPUT_WIDTH))
                        .align_x(Alignment::Center)
                        .padding(INPUT_PADDING)
                        .on_input(AppMessage::ProxyPortChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(ROW_SPACING)),
                    common::create_colored_button(app.i18n.t("settings.proxy-save"), BUTTON_COLOR_BLUE, AppMessage::SaveProxy)
                ]
                .width(Length::FillPortion(2))
                .spacing(0),
            ),
        ],
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
            ),
            create_path_config_row(
                &app.i18n,
                app.i18n.t("settings.cache-path"),
                &common::get_absolute_path(&app.config.data.cache_path),
                AppMessage::CachePathSelected("SELECT_CACHE_PATH".to_string()),
                AppMessage::OpenPath("cache".to_string()),
                AppMessage::ShowPathClearConfirmation("cache".to_string()),
                AppMessage::RestoreDefaultPath("cache".to_string()),
            ),
        ],
    );

    let api_config_section = common::create_config_section(
        app.i18n.t("settings.api-config"),
        vec![common::create_setting_row(
            app.i18n.t("settings.wallhaven-api-key"),
            row![
                text_input(&app.i18n.t("settings.wallhaven-api-key-placeholder"), &app.wallhaven_api_key)
                    .width(Length::Fill)
                    .size(TEXT_INPUT_SIZE)
                    .align_x(Alignment::Center)
                    .on_input(AppMessage::WallhavenApiKeyChanged)
                    .padding(INPUT_PADDING),
                container(iced::widget::Space::new()).width(Length::Fixed(BUTTON_SPACING)),
                common::create_colored_button(app.i18n.t("settings.save"), BUTTON_COLOR_BLUE, AppMessage::SaveWallhavenApiKey)
            ]
            .width(Length::FillPortion(3))
            .spacing(0),
        )],
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
                        app.i18n.t("wallpaper-mode-options.crop-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.fit"),
                        crate::utils::config::WallpaperMode::Fit,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.fit-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.stretch"),
                        crate::utils::config::WallpaperMode::Stretch,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.stretch-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.tile"),
                        crate::utils::config::WallpaperMode::Tile,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.tile-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.center"),
                        crate::utils::config::WallpaperMode::Center,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.center-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("wallpaper-mode-options.span"),
                        crate::utils::config::WallpaperMode::Span,
                        Some(app.wallpaper_mode),
                        |mode| AppMessage::WallpaperModeSelected(mode),
                        app.i18n.t("wallpaper-mode-options.span-tooltip")
                    ),
                ]
                .spacing(ROW_SPACING),
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-change-mode"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.online"),
                        crate::utils::config::WallpaperAutoChangeMode::Online,
                        Some(app.auto_change_mode),
                        |mode| AppMessage::AutoChangeModeSelected(mode),
                        app.i18n.t("auto-change-mode-options.online-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-mode-options.local"),
                        crate::utils::config::WallpaperAutoChangeMode::Local,
                        Some(app.auto_change_mode),
                        |mode| AppMessage::AutoChangeModeSelected(mode),
                        app.i18n.t("auto-change-mode-options.local-tooltip")
                    ),
                ]
                .spacing(ROW_SPACING),
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-change-interval"),
                row![
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.off"),
                        crate::utils::config::WallpaperAutoChangeInterval::Off,
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.off-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.ten-min"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(10),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.ten-min-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.thirty-min"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(30),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.thirty-min-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.one-hour"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(60),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.one-hour-tooltip")
                    ),
                    common::create_radio_with_tooltip(
                        app.i18n.t("auto-change-interval-options.two-hour"),
                        crate::utils::config::WallpaperAutoChangeInterval::Minutes(120),
                        Some(app.auto_change_interval.clone()),
                        |interval| AppMessage::AutoChangeIntervalSelected(interval),
                        app.i18n.t("auto-change-interval-options.two-hour-tooltip")
                    ),
                    tooltip(
                        container(
                            row![
                                iced::widget::radio(
                                    app.i18n.t("auto-change-interval-options.custom"),
                                    crate::utils::config::WallpaperAutoChangeInterval::Custom(app.custom_interval_minutes),
                                    Some(app.auto_change_interval.clone()),
                                    |interval| {
                                        if let crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes) = interval {
                                            AppMessage::AutoChangeIntervalSelected(crate::utils::config::WallpaperAutoChangeInterval::Custom(minutes))
                                        } else {
                                            AppMessage::AutoChangeIntervalSelected(interval)
                                        }
                                    }
                                ),
                                iced_aw::NumberInput::new(&app.custom_interval_minutes, 1..=9999, |minutes| AppMessage::CustomIntervalMinutesChanged(
                                    minutes
                                ))
                                .width(Length::Fixed(80.0))
                                .padding(INPUT_PADDING)
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
            ),
        ],
    );

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let about_config_section = container(
        column!(
            text(app.i18n.t("settings.about-config"))
                .size(SECTION_TITLE_SIZE)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            row![
                container(
                    column![
                        common::create_info_row(app.i18n.t("settings.about-name"), app.i18n.t("app-title")),
                        common::create_info_row(app.i18n.t("settings.about-version"), env!("CARGO_PKG_VERSION").to_string()),
                        create_about_link_row(app.i18n.t("settings.about-author"), "zsyo", "https://github.com/zsyo"),
                        create_about_link_row(
                            app.i18n.t("settings.about-repo"),
                            "https://github.com/zsyo/wallwarp",
                            "https://github.com/zsyo/wallwarp"
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
    .style(common::create_bordered_container_style);

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
        .spacing(ROW_SPACING),
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
) -> iced::Element<'a, AppMessage> {
    row![
        text(label).width(Length::FillPortion(1)),
        row![
            iced::widget::text_input("", path)
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(|_| AppMessage::DataPathSelected("".to_string()))
                .padding(INPUT_PADDING),
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

fn create_about_link_row<'a>(label: String, text_value: &'a str, url: &'a str) -> iced::Element<'a, AppMessage> {
    row![
        text(label),
        button(text(text_value).width(Length::Fill).align_x(Alignment::Center))
            .padding(0)
            .style(|theme: &iced::Theme, _status| {
                let palette = theme.extended_palette();
                iced::widget::button::Style {
                    text_color: palette.primary.base.color,
                    ..iced::widget::button::text(theme, iced::widget::button::Status::Active)
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
