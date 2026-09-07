// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{BUTTON_COLOR_GRAY, BUTTON_TEXT_SIZE, ThemeColors};
use crate::ui::{App, AppMessage};
use crate::utils::hotkey_manager::{self, HotkeyAction};
use iced::widget::{row, text};
use iced::{Alignment, Element};

/// 快捷键区块图标（bootstrap-icons keyboard，码点已验证）
const HOTKEYS_ICON: &str = "\u{F451}";

/// 创建快捷键配置区块
///
/// 每个动作一行：动作名 + 热键按钮（点击进入录制）+ 清除按钮；
/// Wayland 等不支持全局热键的环境下整块替换为不可用提示。
pub fn create_hotkeys_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    if !crate::platform::supports_global_hotkeys() {
        return super::create_placeholder_section(
            HOTKEYS_ICON,
            app.i18n.t("settings.hotkeys-title"),
            app.i18n.t("settings.hotkeys-unsupported"),
            &app.theme_config,
        );
    }

    let rows: Vec<Element<'a, AppMessage>> = HotkeyAction::ALL
        .iter()
        .map(|&action| create_hotkey_row(app, action))
        .collect();

    super::create_config_section(
        HOTKEYS_ICON,
        app.i18n.t("settings.hotkeys-title"),
        rows,
        &app.theme_config,
    )
}

/// 创建单行动作配置行
fn create_hotkey_row<'a>(app: &'a App, action: HotkeyAction) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;
    let recording = app.settings_state.hotkey_recording == Some(action);
    let configured = hotkey_config_str(app, action);

    let mut controls = row![].spacing(8).align_y(Alignment::Center);
    controls = controls.push(create_hotkey_button(app, action, recording, &configured));

    // 已设置时提供清除按钮；录制中不允许清除（先取消录制）
    if !configured.is_empty() && !recording {
        controls = controls.push(common::create_colored_button(
            app.i18n.t("settings.hotkey-clear"),
            BUTTON_COLOR_GRAY,
            SettingsMessage::HotkeyCleared(action).into(),
        ));
    }

    super::create_setting_row(
        app.i18n.t(action.i18n_key()),
        Some(app.i18n.t("settings.hotkey-row-desc")),
        controls,
        theme_colors,
    )
}

/// 动作当前配置的热键字符串（未设置为空）
fn hotkey_config_str(app: &App, action: HotkeyAction) -> String {
    match action {
        HotkeyAction::SwitchNext => app.config.global.hotkey_switch_next.clone(),
        HotkeyAction::SwitchPrevious => app.config.global.hotkey_switch_previous.clone(),
        HotkeyAction::ShowWindow => app.config.global.hotkey_show_window.clone(),
        HotkeyAction::SaveCurrent => app.config.global.hotkey_save_current.clone(),
    }
}

/// 创建热键按钮：未设置显示"设置"，录制中显示提示文本，已设置显示按键组合
fn create_hotkey_button<'a>(
    app: &'a App,
    action: HotkeyAction,
    recording: bool,
    configured: &str,
) -> Element<'a, AppMessage> {
    let theme_colors: ThemeColors = app.theme_colors;

    if recording {
        // 录制中：占位提示文本（禁用样式，Esc 取消）
        return common::create_disabled_colored_button(
            app.i18n.t("settings.hotkey-recording"),
            theme_colors.primary,
        )
        .into();
    }

    if configured.is_empty() {
        return common::create_colored_button(
            app.i18n.t("settings.hotkey-set"),
            BUTTON_COLOR_GRAY,
            SettingsMessage::HotkeyRecordStarted(action).into(),
        )
        .into();
    }

    // 已设置：展示按键组合，点击重新录制
    common::create_colored_button_with_text(
        text(hotkey_manager::format_hotkey_display(configured))
            .size(BUTTON_TEXT_SIZE)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into(),
        BUTTON_COLOR_GRAY,
        SettingsMessage::HotkeyRecordStarted(action).into(),
    )
    .into()
}
