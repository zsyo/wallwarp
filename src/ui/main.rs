// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{ActivePage, App, AppMessage};
use crate::ui::style::{APP_NAME_SIZE, BUTTON_SPACING, LOGO_DISPLAY_SIZE, LOGO_SIZE, LOGO_SPACING, SIDEBAR_WIDTH};
use crate::utils::assets;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Padding};

pub fn view_internal(app: &App) -> Element<'_, AppMessage> {
    let functional_area_width = (app.current_window_width as f32 - SIDEBAR_WIDTH).max(1.0);

    let content: Element<'_, AppMessage> = match app.active_page {
        ActivePage::OnlineWallpapers => super::online::online_view(
            &app.i18n,
            functional_area_width as u32,
            &app.online_state,
            &app.config,
            &app.theme_config,
        ),
        ActivePage::LocalList => super::local::local_view(
            &app.i18n,
            &app.config,
            functional_area_width as u32,
            &app.local_state,
            &app.theme_config,
        ),
        ActivePage::DownloadProgress => super::download::download_view(
            &app.i18n,
            functional_area_width as u32,
            &app.download_state,
            &app.theme_config,
        ),
        ActivePage::Settings => super::settings::settings_view(app),
    };

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let theme_colors = crate::ui::style::ThemeColors::from_theme(app.theme_config.get_theme());
    let sidebar = container(
        column![
            container(iced::widget::Space::new()).height(Length::Fixed(20.0)),
            text(app.i18n.t("app-name"))
                .size(APP_NAME_SIZE)
                .color(theme_colors.text)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
            container(iced::widget::Space::new()).height(Length::Fixed(LOGO_SPACING)),
            create_menu_button(
                app.i18n.t("online-wallpapers.title"),
                app.active_page,
                ActivePage::OnlineWallpapers,
                &app.theme_config
            ),
            create_menu_button(
                app.i18n.t("local-list.title"),
                app.active_page,
                ActivePage::LocalList,
                &app.theme_config
            ),
            create_menu_button(
                app.i18n.t("download-tasks.title"),
                app.active_page,
                ActivePage::DownloadProgress,
                &app.theme_config
            ),
            create_menu_button(
                app.i18n.t("settings"),
                app.active_page,
                ActivePage::Settings,
                &app.theme_config
            ),
            container(iced::widget::Space::new()).height(Length::Fill), // 占位符，将主题按钮推到底部
            create_theme_toggle_button(app),
        ]
        .spacing(BUTTON_SPACING)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .style(create_sidebar_container_style(&app.theme_config));

    let main_content = container(content)
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .padding(0)
        .style(create_main_container_style(&app.theme_config));

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
    theme_config: &'a crate::ui::style::ThemeConfig,
) -> button::Button<'a, AppMessage> {
    use crate::ui::style::{SIDEBAR_INDICATOR_WIDTH, ThemeColors};

    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

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
                    background: Some(iced::Background::Color(theme_colors.sidebar_indicator)),
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
                theme_colors.sidebar_button_selected
            } else {
                match status {
                    iced::widget::button::Status::Hovered => theme_colors.sidebar_button_hover,
                    _ => theme_colors.sidebar_button_default,
                }
            };

            iced::widget::button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: theme_colors.text,
                border: iced::border::Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(0.0),
                },
                ..base
            }
        })
}

/// 创建侧边栏容器样式（无边框，根据主题设置背景色）
fn create_sidebar_container_style(
    theme_config: &crate::ui::style::ThemeConfig,
) -> impl Fn(&iced::Theme) -> iced::widget::container::Style + '_ {
    use crate::ui::style::{SEPARATOR_SHADOW_BLUR, SEPARATOR_SHADOW_OFFSET, ThemeColors};

    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: iced::border::Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::from(0.0),
        },
        shadow: iced::Shadow {
            color: theme_colors.separator_shadow,
            offset: iced::Vector::new(SEPARATOR_SHADOW_OFFSET, 0.0),
            blur_radius: SEPARATOR_SHADOW_BLUR,
        },
        ..Default::default()
    }
}

/// 创建主内容区容器样式（无边框，右侧添加分隔线）
fn create_main_container_style(
    theme_config: &crate::ui::style::ThemeConfig,
) -> impl Fn(&iced::Theme) -> iced::widget::container::Style + '_ {
    use crate::ui::style::{SEPARATOR_WIDTH, ThemeColors};

    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(theme_colors.background)),
        border: iced::border::Border {
            color: theme_colors.separator,
            width: SEPARATOR_WIDTH,
            radius: iced::border::Radius::from(0.0),
        },
        ..Default::default()
    }
}

/// 创建主题切换按钮
fn create_theme_toggle_button(app: &App) -> Element<'_, AppMessage> {
    use crate::ui::style::ThemeColors;

    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    let is_dark = app.theme_config.is_dark();

    // 使用太阳图标表示浅色主题，月亮图标表示深色主题
    let icon = if is_dark {
        "\u{F185}" // 太阳图标（当前是深色，点击切换到浅色）
    } else {
        "\u{F186}" // 月亮图标（当前是浅色，点击切换到深色）
    };

    let tooltip_text = if is_dark {
        app.i18n.t("theme.switch-to-light")
    } else {
        app.i18n.t("theme.switch-to-dark")
    };

    let btn = button(text(icon).size(20))
        .on_press(AppMessage::ToggleTheme)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0))
        .style(move |_theme: &iced::Theme, _status| iced::widget::button::Style {
            background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
            text_color: theme_colors.text,
            border: iced::border::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(20.0),
            },
            ..Default::default()
        });

    super::common::create_button_with_tooltip(btn, tooltip_text)
}
