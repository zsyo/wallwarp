// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::style::ThemeColors;
use crate::ui::style::{
    ABOUT_INFO_WIDTH, ABOUT_LOGO_SPACING, LOGO_DISPLAY_SIZE, LOGO_SIZE, ROW_SPACING, SECTION_PADDING, SECTION_SPACING,
    SECTION_TITLE_SIZE,
};
use crate::ui::{App, AppMessage};
use crate::utils::assets;
use iced::widget::{Space, column, container, image, row, text};
use iced::{Alignment, Element, Length};

/// 创建关于信息区块
pub fn create_about_info_section<'a>(app: &'a App) -> Element<'a, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    container(
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
                        super::create_info_row(
                            app.i18n.t("settings.about-name"),
                            app.i18n.t("app-title"),
                            theme_colors
                        ),
                        super::create_info_row(
                            app.i18n.t("settings.about-version"),
                            env!("CARGO_PKG_VERSION").to_string(),
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
                    .spacing(ROW_SPACING)
                )
                .width(Length::Fixed(ABOUT_INFO_WIDTH)),
                container(Space::new()).width(Length::Fill),
                image(image::Handle::from_rgba(width, height, img))
                    .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                    .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
                container(Space::new()).width(Length::Fixed(ABOUT_LOGO_SPACING)),
            ]
            .width(Length::Fill)
            .spacing(ROW_SPACING)
        )
        .padding(SECTION_PADDING)
        .spacing(SECTION_SPACING),
    )
    .width(Length::Fill)
    .style(common::create_bordered_container_style_with_bg(&app.theme_config))
    .into()
}
