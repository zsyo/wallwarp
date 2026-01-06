use super::App;
use super::AppMessage;
use crate::utils::assets;
use crate::utils::config::CloseAction;
use iced::{
    Alignment, Length,
    widget::{
        Id, button, column, container, pick_list, row, scrollable, text, text_input, toggler,
    },
};
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
            _ => Ok(ProxyProtocol::Http), // 默认为http
        }
    }
}

/// 渲染设置页面的UI组件
pub fn settings_view(app: &App) -> iced::Element<'_, AppMessage> {
    let system_config_section = container(
        column!(
            text(app.i18n.t("settings.system-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::row!(
                text(app.i18n.t("settings.app-language")).width(Length::Fill),
                pick_list(
                    &app.i18n.available_langs[..],
                    Some(app.i18n.current_lang.clone()),
                    AppMessage::LanguageSelected
                )
                .width(Length::Fixed(80.0))
            )
            .align_y(Alignment::Center)
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
            iced::widget::row!(
                text(app.i18n.t("settings.auto-startup")).width(Length::FillPortion(1)),
                toggler(app.config.global.auto_startup).on_toggle(AppMessage::AutoStartupToggled)
            )
            .align_y(Alignment::Center)
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
            iced::widget::row!(
                text(app.i18n.t("settings.close-action")).width(Length::FillPortion(1)),
                iced::widget::row![
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
                .spacing(10)
            )
            .align_y(Alignment::Center)
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
            // 代理设置
            iced::widget::row!(
                text(app.i18n.t("settings.proxy")).width(Length::FillPortion(1)),
                row!(
                    // 协议选择
                    pick_list(
                        [ProxyProtocol::Http, ProxyProtocol::Socks5],
                        ProxyProtocol::from_str(&app.proxy_protocol).ok(),
                        |protocol| AppMessage::ProxyProtocolChanged(protocol.as_str().to_string())
                    )
                    .width(Length::Fixed(80.0)),
                    container(iced::widget::Space::new()).width(Length::Fixed(5.0)),
                    // 地址输入
                    text_input(
                        &app.i18n.t("settings.proxy-address-placeholder"),
                        &app.proxy_address
                    )
                    .width(Length::FillPortion(2))
                    .align_x(Alignment::Center)
                    .padding(5)
                    .on_input(AppMessage::ProxyAddressChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(5.0)),
                    // 端口输入
                    text_input(
                        &app.i18n.t("settings.proxy-port-placeholder"),
                        &app.proxy_port
                    )
                    .width(Length::Fixed(80.0))
                    .align_x(Alignment::Center)
                    .padding(5)
                    .on_input(AppMessage::ProxyPortChanged),
                    container(iced::widget::Space::new()).width(Length::Fixed(5.0)),
                    // 保存按钮
                    button(text(app.i18n.t("settings.proxy-save")).size(14))
                        .on_press(AppMessage::SaveProxy)
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    0, 123, 255,
                                ))), // 蓝色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        })
                )
                .width(Length::FillPortion(2))
                .spacing(0) // 我们手动控制间距，所以设置为0
            )
            .align_y(Alignment::Center)
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10)
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    let data_config_section = container(
        column!(
            text(app.i18n.t("settings.data-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            // 数据路径配置
            row!(
                text(app.i18n.t("settings.data-path")).width(Length::FillPortion(1)),
                row!(
                    iced::widget::text_input("", &get_absolute_path(&app.config.data.data_path))
                        .width(Length::Fill)
                        .size(14)
                        .align_x(Alignment::Center)
                        .on_input(|_| AppMessage::DataPathSelected("".to_string())) // 不响应输入，实现只读效果
                        .padding(5),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.select-path")).size(14).center())
                        .on_press(AppMessage::DataPathSelected("SELECT_DATA_PATH".to_string()))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    0, 123, 255,
                                ))), // 蓝色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.open-path")).size(14).center())
                        .on_press(AppMessage::OpenPath("data".to_string()))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    40, 167, 69,
                                ))), // 绿色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.clear-path")).size(14).center())
                        .on_press(AppMessage::ShowPathClearConfirmation("data".to_string()))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    220, 53, 69,
                                ))), // 红色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(
                        text(app.i18n.t("settings.restore-default"))
                            .size(14)
                            .center()
                    )
                    .on_press(AppMessage::RestoreDefaultPath("data".to_string()))
                    .style(|_theme: &iced::Theme, status| {
                        let base = iced::widget::button::text(_theme, status);
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                108, 117, 125,
                            ))), // 灰色
                            text_color: iced::Color::WHITE,
                            ..base
                        }
                    })
                )
                .width(Length::FillPortion(4))
                .spacing(0) // 我们手动控制间距，所以设置为0
            )
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
            // 缓存路径配置
            row!(
                text(app.i18n.t("settings.cache-path")).width(Length::FillPortion(1)),
                row!(
                    iced::widget::text_input("", &get_absolute_path(&app.config.data.cache_path))
                        .width(Length::Fill)
                        .size(14)
                        .align_x(Alignment::Center)
                        .on_input(|_| AppMessage::CachePathSelected("".to_string())) // 不响应输入，实现只读效果
                        .padding(5),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.select-path")).size(14).center())
                        .on_press(AppMessage::CachePathSelected(
                            "SELECT_CACHE_PATH".to_string()
                        ))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    0, 123, 255,
                                ))), // 蓝色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.open-path")).size(14).center())
                        .on_press(AppMessage::OpenPath("cache".to_string()))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    40, 167, 69,
                                ))), // 绿色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.clear-path")).size(14).center())
                        .on_press(AppMessage::ShowPathClearConfirmation("cache".to_string()))
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    220, 53, 69,
                                ))), // 红色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        }),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(
                        text(app.i18n.t("settings.restore-default"))
                            .size(14)
                            .center()
                    )
                    .on_press(AppMessage::RestoreDefaultPath("cache".to_string()))
                    .style(|_theme: &iced::Theme, status| {
                        let base = iced::widget::button::text(_theme, status);
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                108, 117, 125,
                            ))), // 灰色
                            text_color: iced::Color::WHITE,
                            ..base
                        }
                    })
                )
                .width(Length::FillPortion(4))
                .spacing(0) // 我们手动控制间距，所以设置为0
            )
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    let api_config_section = container(
        column!(
            text(app.i18n.t("settings.api-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            // WallHeven API KEY 配置
            iced::widget::row!(
                text(app.i18n.t("settings.wallhaven-api-key")).width(Length::FillPortion(1)),
                iced::widget::row!(
                    iced::widget::text_input(
                        &app.i18n.t("settings.wallhaven-api-key-placeholder"),
                        &app.wallhaven_api_key
                    )
                    .width(Length::Fill)
                    .size(14)
                    .align_x(Alignment::Center)
                    .on_input(AppMessage::WallhavenApiKeyChanged)
                    .padding(5),
                    container(iced::widget::Space::new()).width(Length::Fixed(2.0)),
                    button(text(app.i18n.t("settings.save")).size(14).center())
                        .on_press(AppMessage::SaveWallhavenApiKey)
                        .style(|_theme: &iced::Theme, status| {
                            let base = iced::widget::button::text(_theme, status);
                            iced::widget::button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    0, 123, 255,
                                ))), // 蓝色
                                text_color: iced::Color::WHITE,
                                ..base
                            }
                        })
                )
                .width(Length::FillPortion(3))
                .spacing(0) // 我们手动控制间距，所以设置为0
            )
            .align_y(Alignment::Center)
            .height(Length::Fixed(30.0))
            .width(Length::Fill)
            .spacing(10),
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    let (img, width, height) = assets::get_logo(128);
    let about_config_section = container(
        column!(
            text(app.i18n.t("settings.about-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::row!(
                // 左侧信息
                container(
                    column!(
                        iced::widget::row!(
                            text(app.i18n.t("settings.about-name")),
                            text(app.i18n.t("app-title"))
                                .width(Length::Fill)
                                .align_x(Alignment::Center)
                        )
                        .width(Length::Fill)
                        .spacing(10),
                        iced::widget::row!(
                            text(app.i18n.t("settings.about-version")),
                            text(env!("CARGO_PKG_VERSION"))
                                .width(Length::Fill)
                                .align_x(Alignment::Center)
                        )
                        .width(Length::Fill)
                        .spacing(10),
                        iced::widget::row!(
                            text(app.i18n.t("settings.about-author")),
                            button(text("zsyo").width(Length::Fill).align_x(Alignment::Center))
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
                                .on_press(AppMessage::OpenUrl(
                                    "https://github.com/zsyo".to_string()
                                ))
                        )
                        .height(Length::Fixed(16.0))
                        .width(Length::Fill)
                        .align_y(Alignment::Center)
                        .spacing(10),
                        iced::widget::row!(
                            text(app.i18n.t("settings.about-repo")),
                            button(
                                text("https://github.com/zsyo/wallwarp")
                                    .width(Length::Fill)
                                    .align_x(Alignment::Center)
                            )
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
                            .on_press(AppMessage::OpenUrl(
                                "https://github.com/zsyo/wallwarp".to_string()
                            ))
                        )
                        .height(Length::Fixed(16.0))
                        .width(Length::Fill)
                        .align_y(Alignment::Center)
                        .spacing(10)
                    )
                    .spacing(15)
                )
                .width(Length::Fixed(350.0)),
                container(iced::widget::Space::new()).width(Length::Fill),
                // 右侧图标
                iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                    .width(Length::Fixed(128.0))
                    .height(Length::Fixed(128.0)),
                container(iced::widget::Space::new()).width(Length::Fixed(40.0)),
            )
            .width(Length::Fill)
            .spacing(15)
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    scrollable(
        column!(
            system_config_section,
            data_config_section,
            api_config_section,
            about_config_section,
        )
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(20)
        .spacing(10),
    )
    .height(Length::Fill)
    .id(Id::new("settings_scroll"))
    .into()
}

/// 将路径转换为绝对路径进行展示
fn get_absolute_path(path: &str) -> String {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let path_buf = std::path::PathBuf::from(path);

    if path_buf.is_absolute() {
        path.to_string()
    } else {
        current_dir.join(path_buf).to_string_lossy().to_string()
    }
}
