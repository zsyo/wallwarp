// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::style::{
    ABOUT_LOGO_SPACING, ABOUT_INFO_WIDTH, BUTTON_COLOR_BLUE, LOGO_DISPLAY_SIZE, ROW_SPACING,
    BUTTON_SPACING,
};
use crate::ui::{App, AppMessage};
use iced::widget::{column, container, image, row, Space};
use iced::{Alignment, Element, Length};

/// 创建关于信息区块（预留检查更新入口）
pub fn create_about_info_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = app.theme_colors;

    let info_column = column![
        super::create_info_row(
            app.i18n.t("settings.about-name"),
            app.i18n.t("app-title"),
            theme_colors
        ),
        super::create_info_row(
            app.i18n.t("settings.about-version"),
            // build.rs 注入的显示版本：预发布 tag 下为
            // "1.5.1_beta.1" 形式，本地开发为 Cargo.toml 版本
            env!("WALLWARP_VERSION").to_string(),
            theme_colors
        ),
        super::create_about_link_row(
            app.i18n.t("settings.about-author"),
            "zsyo",
            "https://github.com/zsyo",
            theme_colors
        ),
        super::create_about_link_row(
            app.i18n.t("settings.about-repo"),
            "https://github.com/zsyo/wallwarp",
            "https://github.com/zsyo/wallwarp",
            theme_colors
        ),
    ]
    .spacing(ROW_SPACING);

    let rows = vec![
        row![
            container(info_column).width(Length::Fixed(ABOUT_INFO_WIDTH)),
            Space::new().width(Length::Fill),
            image(&app.logo_handle)
                .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
            Space::new().width(Length::Fixed(ABOUT_LOGO_SPACING)),
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .into(),
        // 预留：检查更新
        super::create_setting_row(
            app.i18n.t("settings.check-update"),
            Some(app.i18n.t("settings.check-update-desc")),
            row![
                common::create_disabled_colored_button(
                    app.i18n.t("settings.check-update-button"),
                    BUTTON_COLOR_BLUE,
                ),
                super::create_coming_soon_badge(app.i18n.t("settings.coming-soon"), theme_colors),
            ]
            .spacing(BUTTON_SPACING)
            .align_y(Alignment::Center),
            theme_colors,
        ),
    ];

    super::create_config_section(
        "\u{F431}", // info-circle
        app.i18n.t("settings.category-about"),
        rows,
        &app.theme_config,
    )
}
