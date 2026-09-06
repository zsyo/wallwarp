// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{
    BUTTON_COLOR_BLUE, INPUT_PADDING, PORT_INPUT_WIDTH, ROW_SPACING, with_alpha,
};
use crate::ui::{App, AppMessage};
use iced::widget::{container, row, text_input, toggler};
use iced::{Alignment, Element, Length};

/// 创建网络配置区块（代理）
pub fn create_network_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let rows = vec![
        // 代理开关行
        super::create_setting_row(
            app.i18n.t("settings.proxy"),
            Some(app.i18n.t("settings.proxy-desc")),
            toggler(app.settings_state.proxy_enabled)
                .on_toggle(|state| SettingsMessage::ProxyToggled(state).into()),
            theme_colors,
        ),
        // 服务器配置行（开关关闭时输入框呈禁用态，仍可见）
        super::create_full_width_row(
            app.i18n.t("settings.proxy-server"),
            Some(app.i18n.t("settings.proxy-server-desc")),
            proxy_server_row(app),
            theme_colors,
        ),
    ];

    super::create_config_section(
        "\u{F3EE}", // globe
        app.i18n.t("settings.category-network"),
        rows,
        &app.theme_config,
    )
}

/// 代理服务器配置行：协议选择 + 地址输入 + 端口输入 + 保存按钮
fn proxy_server_row<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let proxy_enabled = app.settings_state.proxy_enabled;

    row![
        super::create_proxy_protocol_picker(app),
        text_input(
            &app.i18n.t("settings.proxy-address-placeholder"),
            &app.settings_state.proxy_address
        )
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(INPUT_PADDING)
        .on_input_maybe(
            proxy_enabled
                .then_some(move |s: String| SettingsMessage::ProxyAddressChanged(s).into())
        )
        .style(styled_text_input(theme_colors)),
        {
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
        common::create_colored_button(
            app.i18n.t("settings.proxy-save"),
            BUTTON_COLOR_BLUE,
            SettingsMessage::SaveProxy.into()
        ),
    ]
    .spacing(ROW_SPACING / 2.0)
    .align_y(Alignment::Center)
    .into()
}
