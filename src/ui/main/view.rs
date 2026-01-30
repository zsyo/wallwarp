// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common;
use crate::ui::main::MainMessage;
use crate::ui::style::{
    APP_NAME_SIZE, BUTTON_COLOR_YELLOW, LOGO_DISPLAY_SIZE, LOGO_SIZE, LOGO_SPACING, SEPARATOR_WIDTH,
    SIDEBAR_INDICATOR_WIDTH, SIDEBAR_WIDTH,
};
use crate::ui::style::{ThemeColors, ThemeConfig};
use crate::ui::{ActivePage, App, AppMessage};
use crate::ui::{download, local, online, settings};
use crate::utils::assets;
use crate::utils::config::Theme;
use iced::border::{Border, Radius};
use iced::widget::{Space, button, column, container, image, row, text, tooltip};
use iced::{Alignment, Element, Length, Padding};

pub fn main_view(app: &App) -> Element<'_, AppMessage> {
    let functional_area_width = (app.current_window_width as f32 - SIDEBAR_WIDTH).max(1.0);

    let content: Element<'_, AppMessage> = match app.active_page {
        ActivePage::OnlineWallpapers => online::online_view(
            &app.i18n,
            functional_area_width as u32,
            &app.online_state,
            &app.config,
            &app.theme_config,
        ),
        ActivePage::LocalList => local::local_view(
            &app.i18n,
            &app.config,
            functional_area_width as u32,
            &app.local_state,
            &app.theme_config,
        ),
        ActivePage::DownloadProgress => download::download_view(
            &app.i18n,
            functional_area_width as u32,
            &app.download_state,
            &app.theme_config,
        ),
        ActivePage::Settings => settings::settings_view(app),
    };

    // 创建自定义标题栏
    let title_bar = common::create_title_bar(
        app.title(),
        app.is_maximized,
        &app.theme_config,
        MainMessage::TitleBarDrag.into(),
        MainMessage::MinimizeToTray.into(),
        app.i18n.t("titlebar.minimize-to-tray"),
        MainMessage::TitleBarMinimize.into(),
        MainMessage::TitleBarMaximize.into(),
        MainMessage::TitleBarClose.into(),
    );

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());
    let sidebar = container(
        column![
            container(Space::new()).height(Length::Fixed(20.0)),
            text(app.i18n.t("app-name"))
                .size(APP_NAME_SIZE)
                .color(theme_colors.text)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            image(image::Handle::from_rgba(width, height, img))
                .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
            container(Space::new()).height(Length::Fixed(LOGO_SPACING)),
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
            container(Space::new()).height(Length::Fill), // 占位符，将主题按钮推到底部
            create_theme_toggle_button(app),
            container(Space::new()).height(Length::Fixed(20.0)),
        ]
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

    // 创建主布局
    let layout = row![sidebar, main_content]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    // 将标题栏和主内容组合
    let full_layout = column![title_bar, layout]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill);

    // 使用带边缘调整大小功能的容器包裹整个界面
    // 边缘触发区域大小为 5 像素
    // 当窗口最大化时,禁用边缘调整大小功能
    let resizable_layout = common::create_resizable_container(
        full_layout.into(),
        5.0, // 边缘触发区域大小
        |direction| MainMessage::ResizeWindow(direction).into(),
        app.is_maximized, // 窗口是否已最大化
    );

    resizable_layout
}

fn create_menu_button<'a>(
    label: String,
    current_page: ActivePage,
    target_page: ActivePage,
    theme_config: &'a ThemeConfig,
) -> button::Button<'a, AppMessage> {
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
        Space::new().width(Length::Fill),
        if is_selected {
            container(Space::new())
                .width(Length::Fixed(SIDEBAR_INDICATOR_WIDTH))
                .height(Length::Fill)
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(theme_colors.sidebar_indicator)),
                    border: Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: Radius::from(4.0),
                    },
                    ..Default::default()
                })
        } else {
            container(Space::new())
                .width(Length::Fixed(SIDEBAR_INDICATOR_WIDTH))
                .height(Length::Fill)
        }
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    button(button_content)
        .on_press_maybe(if current_page != target_page {
            Some(MainMessage::PageSelected(target_page).into())
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
            let base = button::text(_theme, status);
            let bg_color = if is_selected {
                theme_colors.sidebar_button_selected
            } else {
                match status {
                    button::Status::Hovered => theme_colors.sidebar_button_hover,
                    _ => theme_colors.sidebar_button_default,
                }
            };

            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color: theme_colors.text,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(0.0),
                },
                ..base
            }
        })
}

/// 创建侧边栏容器样式（无边框，根据主题设置背景色）
fn create_sidebar_container_style(theme_config: &ThemeConfig) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.sidebar_bg)),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(0.0),
        },
        shadow: iced::Shadow::default(),
        ..Default::default()
    }
}

/// 创建主内容区容器样式（无边框，右侧添加分隔线）
fn create_main_container_style(theme_config: &ThemeConfig) -> impl Fn(&iced::Theme) -> container::Style + '_ {
    let theme_colors = ThemeColors::from_theme(theme_config.get_theme());

    move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(theme_colors.background)),
        border: Border {
            color: theme_colors.separator,
            width: SEPARATOR_WIDTH,
            radius: Radius::from(0.0),
        },
        ..Default::default()
    }
}

/// 创建主题切换按钮
fn create_theme_toggle_button(app: &App) -> Element<'_, AppMessage> {
    let theme_colors = ThemeColors::from_theme(app.theme_config.get_theme());

    let (icon_char, tooltip_text, target_theme) = if app.theme_config.is_dark() {
        ("\u{F5A1}", app.i18n.t("theme.switch-to-light"), Theme::Light)
    } else {
        ("\u{F494}", app.i18n.t("theme.switch-to-dark"), Theme::Dark)
    };

    let btn = button(
        text(icon_char)
            .color(BUTTON_COLOR_YELLOW)
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(20),
    )
    .on_press(MainMessage::ThemeSelected(target_theme).into())
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .style(move |_theme: &iced::Theme, _status| button::Style {
        background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
        text_color: theme_colors.text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(20.0),
        },
        ..Default::default()
    });

    common::create_button_with_tooltip(btn, tooltip_text, tooltip::Position::Top, &app.theme_config)
}
