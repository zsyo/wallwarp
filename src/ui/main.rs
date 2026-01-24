// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{ActivePage, App, AppMessage};
use crate::ui::style::{APP_NAME_SIZE, BUTTON_SPACING, LOGO_DISPLAY_SIZE, LOGO_SIZE, LOGO_SPACING, SIDEBAR_WIDTH};
use crate::utils::assets;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Padding};

pub fn view_internal(app: &App) -> Element<'_, AppMessage> {
    let functional_area_width = (app.current_window_width as f32 - SIDEBAR_WIDTH).max(1.0);

    let content: Element<'_, AppMessage> = match app.active_page {
        ActivePage::OnlineWallpapers => {
            super::online::online_view(&app.i18n, functional_area_width as u32, &app.online_state, &app.config)
        }
        ActivePage::LocalList => {
            super::local::local_view(&app.i18n, &app.config, functional_area_width as u32, &app.local_state)
        }
        ActivePage::DownloadProgress => {
            super::download::download_view(&app.i18n, functional_area_width as u32, &app.download_state)
        }
        ActivePage::Settings => super::settings::settings_view(app),
    };

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let sidebar = container(
        column![
            container(iced::widget::Space::new()).height(Length::Fixed(20.0)),
            text(app.i18n.t("app-name"))
                .size(APP_NAME_SIZE)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
            container(iced::widget::Space::new()).height(Length::Fixed(LOGO_SPACING)),
            create_menu_button(
                app.i18n.t("online-wallpapers.title"),
                app.active_page,
                ActivePage::OnlineWallpapers
            ),
            create_menu_button(app.i18n.t("local-list.title"), app.active_page, ActivePage::LocalList),
            create_menu_button(
                app.i18n.t("download-tasks.title"),
                app.active_page,
                ActivePage::DownloadProgress
            ),
            create_menu_button(app.i18n.t("settings"), app.active_page, ActivePage::Settings),
        ]
        .spacing(BUTTON_SPACING)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .style(create_sidebar_container_style);

    let main_content = container(content)
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .padding(0)
        .style(create_main_container_style);

    let layout = row![sidebar, main_content]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    layout.into()
}

fn create_menu_button<'a>(
    label: String,
    current_page: ActivePage,
    target_page: ActivePage,
) -> button::Button<'a, AppMessage> {
    use crate::ui::style::{
        COLOR_SIDEBAR_BUTTON_DEFAULT, COLOR_SIDEBAR_BUTTON_HOVER, COLOR_SIDEBAR_BUTTON_SELECTED,
        COLOR_SIDEBAR_INDICATOR, SIDEBAR_INDICATOR_WIDTH,
    };

    let is_selected = current_page == target_page;
    let icon = match target_page {
        ActivePage::OnlineWallpapers => "🏠",
        ActivePage::LocalList => "📁",
        ActivePage::DownloadProgress => "⬇️",
        ActivePage::Settings => "⚙️",
    };

    let button_content = row![
        row![text(icon), text(label)].spacing(8).align_y(Alignment::Center),
        iced::widget::Space::new().width(Length::Fill),
        if is_selected {
            container(iced::widget::Space::new())
                .width(Length::Fixed(SIDEBAR_INDICATOR_WIDTH))
                .height(Length::Fill)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(COLOR_SIDEBAR_INDICATOR)),
                    border: iced::border::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(4.0),
                    },
                    ..Default::default()
                })
        } else {
            container(iced::widget::Space::new())
                .width(Length::Fixed(SIDEBAR_INDICATOR_WIDTH))
                .height(Length::Fill)
        }
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    button(button_content)
        .on_press_maybe(if current_page != target_page {
            Some(AppMessage::PageSelected(target_page))
        } else {
            None
        })
        .padding(Padding {
            top: 6.0,
            right: 1.0,
            bottom: 6.0,
            left: 12.0,
        })
        .height(Length::Fixed(60.0))
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme, status| {
            let base = iced::widget::button::text(_theme, status);
            let bg_color = if is_selected {
                COLOR_SIDEBAR_BUTTON_SELECTED
            } else {
                match status {
                    iced::widget::button::Status::Hovered => COLOR_SIDEBAR_BUTTON_HOVER,
                    _ => COLOR_SIDEBAR_BUTTON_DEFAULT,
                }
            };

            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: crate::ui::style::COLOR_TEXT_DARK,
                border: iced::border::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(0.0),
                },
                ..base
            }
        })
}

/// 创建侧边栏容器样式（无边框，浅灰色背景）
fn create_sidebar_container_style(_theme: &iced::Theme) -> iced::widget::container::Style {
    use crate::ui::style::{COLOR_SIDEBAR_BG, SEPARATOR_SHADOW_BLUR, SEPARATOR_SHADOW_OFFSET};

    iced::widget::container::Style {
        background: Some(iced::Background::Color(COLOR_SIDEBAR_BG)),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(0.0),
        },
        shadow: iced::Shadow {
            color: iced::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.1,
            },
            offset: iced::Vector::new(SEPARATOR_SHADOW_OFFSET, 0.0),
            blur_radius: SEPARATOR_SHADOW_BLUR,
        },
        ..Default::default()
    }
}

/// 创建主内容区容器样式（无边框，右侧添加分隔线）
fn create_main_container_style(_theme: &iced::Theme) -> iced::widget::container::Style {
    use crate::ui::style::{COLOR_SEPARATOR, SEPARATOR_WIDTH};

    iced::widget::container::Style {
        background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
        border: iced::border::Border {
            color: COLOR_SEPARATOR,
            width: SEPARATOR_WIDTH,
            radius: iced::border::Radius::from(0.0),
        },
        ..Default::default()
    }
}
