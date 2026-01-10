use super::App;
use super::AppMessage;
use super::common;
use crate::utils::assets;
use crate::utils::config::CloseAction;
use iced::{Alignment, Length};
use iced::widget::{
    button, column, container, pick_list, row, scrollable, text, text_input, toggler,
};
use std::str::FromStr;

// 布局常量
const SCROLL_PADDING: f32 = 20.0;

// 尺寸常量
const PICK_LIST_WIDTH: f32 = 80.0;
const PORT_INPUT_WIDTH: f32 = 80.0;
const ABOUT_INFO_WIDTH: f32 = 350.0;
const LOGO_SIZE: u32 = 128;
const LOGO_DISPLAY_SIZE: f32 = 128.0;
const ABOUT_LOGO_SPACING: f32 = 40.0;
const ABOUT_ROW_HEIGHT: f32 = 16.0;

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
                pick_list(
                    &app.i18n.available_langs[..],
                    Some(app.i18n.current_lang.clone()),
                    AppMessage::LanguageSelected
                )
                .width(Length::Fixed(PICK_LIST_WIDTH)),
            ),
            common::create_setting_row(
                app.i18n.t("settings.auto-startup"),
                toggler(app.config.global.auto_startup)
                    .on_toggle(AppMessage::AutoStartupToggled),
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
                .spacing(common::ROW_SPACING),
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
                    container(iced::widget::Space::new()).width(Length::Fixed(common::ROW_SPACING)),
                    text_input(
                        &app.i18n.t("settings.proxy-address-placeholder"),
                        &app.proxy_address
                    )
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::Center)
                    .padding(common::INPUT_PADDING)
                    .on_input(AppMessage::ProxyAddressChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(common::ROW_SPACING)),
                    text_input(
                        &app.i18n.t("settings.proxy-port-placeholder"),
                        &app.proxy_port
                    )
                    .width(Length::Fixed(PORT_INPUT_WIDTH))
                    .align_x(Alignment::Center)
                    .padding(common::INPUT_PADDING)
                    .on_input(AppMessage::ProxyPortChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(common::ROW_SPACING)),
                    common::create_colored_button(
                        app.i18n.t("settings.proxy-save"),
                        common::BUTTON_COLOR_BLUE,
                        AppMessage::SaveProxy
                    )
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
                text_input(
                    &app.i18n.t("settings.wallhaven-api-key-placeholder"),
                    &app.wallhaven_api_key
                )
                .width(Length::Fill)
                .size(common::TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(AppMessage::WallhavenApiKeyChanged)
                .padding(common::INPUT_PADDING),
                container(iced::widget::Space::new()).width(Length::Fixed(common::BUTTON_SPACING)),
                common::create_colored_button(
                    app.i18n.t("settings.save"),
                    common::BUTTON_COLOR_BLUE,
                    AppMessage::SaveWallhavenApiKey
                )
            ]
            .width(Length::FillPortion(3))
            .spacing(0),
        )],
    );

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let about_config_section = container(
        column!(
            text(app.i18n.t("settings.about-config"))
                .size(common::SECTION_TITLE_SIZE)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            row![
                container(
                    column![
                        common::create_info_row(
                            app.i18n.t("settings.about-name"),
                            app.i18n.t("app-title")
                        ),
                        common::create_info_row(
                            app.i18n.t("settings.about-version"),
                            env!("CARGO_PKG_VERSION").to_string()
                        ),
                        create_about_link_row(
                            app.i18n.t("settings.about-author"),
                            "zsyo",
                            "https://github.com/zsyo"
                        ),
                        create_about_link_row(
                            app.i18n.t("settings.about-repo"),
                            "https://github.com/zsyo/wallwarp",
                            "https://github.com/zsyo/wallwarp"
                        ),
                    ]
                    .spacing(common::ROW_SPACING)
                )
                .width(Length::Fixed(ABOUT_INFO_WIDTH)),
                container(iced::widget::Space::new()).width(Length::Fill),
                iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                    .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                    .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
                container(iced::widget::Space::new()).width(Length::Fixed(ABOUT_LOGO_SPACING)),
            ]
            .width(Length::Fill)
            .spacing(common::ROW_SPACING)
        )
        .padding(common::SECTION_PADDING)
        .spacing(common::SECTION_SPACING),
    )
    .width(Length::Fill)
    .style(common::create_bordered_container_style);

    scrollable(
        column![
            system_config_section,
            data_config_section,
            api_config_section,
            about_config_section,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(SCROLL_PADDING)
        .spacing(common::ROW_SPACING),
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
                .size(common::TEXT_INPUT_SIZE)
                .align_x(Alignment::Center)
                .on_input(|_| AppMessage::DataPathSelected("".to_string()))
                .padding(common::INPUT_PADDING),
            container(iced::widget::Space::new()).width(Length::Fixed(common::BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.select-path"),
                common::BUTTON_COLOR_BLUE,
                select_msg
            ),
            container(iced::widget::Space::new()).width(Length::Fixed(common::BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.open-path"),
                common::BUTTON_COLOR_GREEN,
                open_msg
            ),
            container(iced::widget::Space::new()).width(Length::Fixed(common::BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.clear-path"),
                common::BUTTON_COLOR_RED,
                clear_msg
            ),
            container(iced::widget::Space::new()).width(Length::Fixed(common::BUTTON_SPACING)),
            common::create_colored_button(
                i18n.t("settings.restore-default"),
                common::BUTTON_COLOR_GRAY,
                restore_msg
            ),
        ]
        .width(Length::FillPortion(4))
        .spacing(0),
    ]
    .height(Length::Fixed(common::INPUT_HEIGHT))
    .width(Length::Fill)
    .spacing(common::ROW_SPACING)
    .into()
}

fn create_about_link_row<'a>(
    label: String,
    text_value: &'a str,
    url: &'a str,
) -> iced::Element<'a, AppMessage> {
    row![
        text(label),
        button(text(text_value).width(Length::Fill).align_x(Alignment::Center))
            .padding(0)
            .style(|theme: &iced::Theme, _status| {
                let palette = theme.extended_palette();
                iced::widget::button::Style {
                    text_color: palette.primary.base.color,
                    ..iced::widget::button::text(
                        theme,
                        iced::widget::button::Status::Active,
                    )
                }
            })
            .on_press(AppMessage::OpenUrl(url.to_string())),
    ]
    .height(Length::Fixed(ABOUT_ROW_HEIGHT))
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .spacing(common::ROW_SPACING)
    .into()
}
