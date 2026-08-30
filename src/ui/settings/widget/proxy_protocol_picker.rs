// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::drop_down::{
    self, DropDown, dropdown_option_style, dropdown_panel_style, dropdown_trigger_button,
};
use crate::ui::settings::ProxyProtocol;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::PICK_LIST_WIDTH;
use crate::ui::{App, AppMessage};
use iced::widget::{button, column, container, opaque, text};
use iced::{Element, Length};
use std::str::FromStr;

/// 创建代理协议选择器
pub fn create_proxy_protocol_picker<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let current_protocol = ProxyProtocol::from_str(&app.settings_state.proxy_protocol).ok();

    let protocol_text = current_protocol
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| "http".to_string());

    // 触发按钮（underlay）
    let protocol_trigger = dropdown_trigger_button(
        protocol_text,
        PICK_LIST_WIDTH,
        theme_colors,
        SettingsMessage::ProxyProtocolPickerExpanded.into(),
    );

    // 代理协议选项（overlay）
    let protocol_options_content = column(
        [
            ProxyProtocol::Http,
            ProxyProtocol::Socks5,
            ProxyProtocol::Socks5h,
        ]
        .iter()
        .map(|protocol| {
            let is_selected = current_protocol == Some(*protocol);
            button(text(protocol.as_str()).size(14))
                .padding(6)
                .width(Length::Fill)
                .on_press(
                    SettingsMessage::ProxyProtocolChanged(protocol.as_str().to_string()).into(),
                )
                .style(dropdown_option_style(theme_colors, is_selected))
                .into()
        }),
    )
    .spacing(2);

    let picker_content = container(protocol_options_content)
        .padding(8)
        .width(Length::Fixed(PICK_LIST_WIDTH))
        .style(dropdown_panel_style(theme_colors));

    DropDown::new(
        protocol_trigger,
        opaque(picker_content),
        app.settings_state.proxy_protocol_picker_expanded,
    )
    .width(Length::Shrink)
    .on_dismiss(SettingsMessage::ProxyProtocolPickerDismiss.into())
    .alignment(drop_down::Alignment::Bottom)
    .into()
}
