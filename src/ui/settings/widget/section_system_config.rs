// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, INPUT_PADDING, PORT_INPUT_WIDTH, ROW_SPACING, with_alpha,
};
use crate::ui::{App, AppMessage};
use crate::utils::config::CloseAction;
use crate::utils::startup;
use iced::widget::{Space, container, radio, row, text_input, toggler};
use iced::{Alignment, Color, Element, Length};

/// 关闭动作单选按钮样式（透明背景 + 指定文字色）
fn radio_style(
    text_color: Color,
) -> impl Fn(&iced::Theme, iced::widget::radio::Status) -> iced::widget::radio::Style {
    move |theme: &iced::Theme, status| iced::widget::radio::Style {
        text_color: Some(text_color),
        background: iced::Background::Color(Color::TRANSPARENT),
        ..iced::widget::radio::default(theme, status)
    }
}

/// 创建系统配置区块
pub fn create_system_config_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let mut rows = vec![
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
            app.i18n.t("settings.enable-logging"),
            row![
                toggler(app.config.global.enable_logging)
                    .on_toggle(|state| SettingsMessage::LoggingToggled(state).into()),
                container(Space::new()).width(Length::Fixed(ROW_SPACING)),
                super::create_log_level_picker(app),
            ]
            .align_y(Alignment::Center),
            &app.theme_config,
        ),
        super::create_setting_row(
            app.i18n.t("settings.auto-startup"),
            toggler(startup::is_auto_startup_enabled())
                .on_toggle(|state| SettingsMessage::AutoStartupToggled(state).into()),
            &app.theme_config,
        ),
    ];
    // Wayland 会话下窗口定位/置顶受限，悬浮球不可用，隐藏该设置项
    if crate::platform::supports_floating_ball() {
        rows.push(super::create_setting_row(
            app.i18n.t("settings.show-floating-ball"),
            toggler(app.config.global.show_floating_ball)
                .on_toggle(|state| SettingsMessage::FloatingBallToggled(state).into()),
            &app.theme_config,
        ));
    }
    rows.push(super::create_setting_row(
        app.i18n.t("settings.close-action"),
        row![
            radio(
                app.i18n.t("close-action-options.ask"),
                CloseAction::Ask,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(radio_style(theme_colors.text)),
            radio(
                app.i18n.t("close-action-options.minimize-to-tray"),
                CloseAction::MinimizeToTray,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(radio_style(theme_colors.text)),
            radio(
                app.i18n.t("close-action-options.close-app"),
                CloseAction::CloseApp,
                Some(app.config.global.close_action),
                |act| SettingsMessage::CloseActionSelected(act).into()
            )
            .style(radio_style(theme_colors.text)),
        ]
        .spacing(ROW_SPACING),
        &app.theme_config,
    ));
    rows.push(super::create_setting_row(
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
            .on_input_maybe(
                app.settings_state
                    .proxy_enabled
                    .then_some(move |s: String| { SettingsMessage::ProxyAddressChanged(s).into() })
            )
            .style(styled_text_input(theme_colors)),
            container(Space::new()).width(Length::Fixed(ROW_SPACING)),
            {
                let proxy_enabled = app.settings_state.proxy_enabled;
                container(
                    iced_aw::NumberInput::new(
                        &app.settings_state.proxy_port,
                        1..=65535,
                        move |n| {
                            if proxy_enabled {
                                SettingsMessage::ProxyPortChanged(n).into()
                            } else {
                                SettingsMessage::ProxyToggled(false).into()
                            }
                        },
                    )
                    .width(Length::Fill)
                    .align_x(Alignment::Start)
                    .padding(INPUT_PADDING)
                    .input_style(styled_text_input(theme_colors))
                    .style(
                        move |_theme: &iced::Theme, _status| iced_aw::number_input::Style {
                            button_background: Some(iced::Background::Color(with_alpha(
                                theme_colors.text_input_background,
                                if proxy_enabled { 1.0 } else { 0.45 },
                            ))),
                            icon_color: if proxy_enabled {
                                theme_colors.light_text_sub
                            } else {
                                theme_colors.disabled_color
                            },
                        },
                    ),
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
    ));

    super::create_config_section(
        app.i18n.t("settings.system-config"),
        rows,
        &app.theme_config,
    )
}
