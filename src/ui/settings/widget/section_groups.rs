// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸组设置区块：把当前在线筛选快照保存为命名组，支持删除
//!
//! 组的应用（快速切换）在主界面在线页筛选栏的"壁纸组"选择器中进行

use crate::services::source::SourceKind;
use crate::ui::common;
use crate::ui::common::styled_text_input;
use crate::ui::settings::SettingsMessage;
use crate::ui::style::{BUTTON_COLOR_BLUE, INPUT_PADDING, ROW_SPACING, TEXT_INPUT_SIZE};
use crate::ui::{App, AppMessage};
use iced::widget::{row, text, text_input};
use iced::{Alignment, Element, Length};

/// 创建壁纸组区块
pub fn create_groups_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;

    let mut rows = vec![
        // 保存当前筛选为组（组名输入 + 保存按钮，占满整行）
        super::create_full_width_row(
            app.i18n.t("settings.save-current-as-group"),
            Some(app.i18n.t("settings.save-current-as-group-desc")),
            row![
                text_input(
                    &app.i18n.t("settings.group-name-placeholder"),
                    &app.settings_state.group_name_input,
                )
                .width(Length::Fill)
                .size(TEXT_INPUT_SIZE)
                .padding(INPUT_PADDING)
                .on_input(|name| SettingsMessage::GroupNameChanged(name).into())
                .style(styled_text_input(theme_colors)),
                common::create_colored_button(
                    app.i18n.t("settings.save-as-group"),
                    BUTTON_COLOR_BLUE,
                    SettingsMessage::SaveGroupFromCurrent.into(),
                ),
            ]
            .spacing(ROW_SPACING)
            .align_y(Alignment::Center),
            theme_colors,
        ),
    ];

    // 组列表（每组一行：组名 + 来源图源说明 + 删除按钮）
    if app.wallpaper_groups.is_empty() {
        rows.push(super::create_setting_row(
            app.i18n.t("settings.group-empty"),
            None,
            text(""),
            theme_colors,
        ));
    } else {
        for group in &app.wallpaper_groups {
            let source_name = match SourceKind::parse(&group.source) {
                SourceKind::Wallhaven => app.i18n.t("settings.source-name-wallhaven"),
                SourceKind::Bing => app.i18n.t("settings.source-name-bing"),
            };
            rows.push(super::create_setting_row(
                group.name.clone(),
                Some(source_name),
                common::create_icon_button_with_tooltip(
                    "\u{F78B}", // trash3 (删除)
                    theme_colors.notification_error_bg,
                    SettingsMessage::DeleteGroup(group.id).into(),
                    app.i18n.t("settings.group-delete"),
                    theme_colors,
                ),
                theme_colors,
            ));
        }
    }

    super::create_config_section(
        "\u{F785}", // collection
        app.i18n.t("settings.category-groups"),
        rows,
        &app.theme_config,
    )
}
