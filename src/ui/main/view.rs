// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::{MainMessage, widget};
use crate::ui::style::ThemeColors;
use crate::ui::style::{APP_NAME_SIZE, LOGO_DISPLAY_SIZE, LOGO_SIZE, LOGO_SPACING, SIDEBAR_WIDTH};
use crate::ui::{ActivePage, App, AppMessage};
use crate::ui::{download, local, online, settings};
use crate::utils::assets;
use iced::widget::{Space, column, container, image, row, text};
use iced::{Alignment, Element, Length};

pub fn main_view(app: &App) -> Element<'_, AppMessage> {
    let functional_area_width = (app.main_state.current_window_width as f32 - SIDEBAR_WIDTH).max(1.0);

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
    let title_bar = widget::create_title_bar(
        app.title(),
        app.main_state.is_maximized,
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
            widget::create_menu_button(
                app.i18n.t("online-wallpapers.title"),
                app.active_page,
                ActivePage::OnlineWallpapers,
                &app.theme_config
            ),
            widget::create_menu_button(
                app.i18n.t("local-list.title"),
                app.active_page,
                ActivePage::LocalList,
                &app.theme_config
            ),
            widget::create_menu_button(
                app.i18n.t("download-tasks.title"),
                app.active_page,
                ActivePage::DownloadProgress,
                &app.theme_config
            ),
            widget::create_menu_button(
                app.i18n.t("settings"),
                app.active_page,
                ActivePage::Settings,
                &app.theme_config
            ),
            container(Space::new()).height(Length::Fill), // 占位符，将主题按钮推到底部
            widget::create_theme_toggle_button(app),
            container(Space::new()).height(Length::Fixed(20.0)),
        ]
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .style(widget::create_sidebar_container_style(&app.theme_config));

    let main_content = container(content)
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .padding(0)
        .style(widget::create_main_container_style(&app.theme_config));

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
    let resizable_layout = widget::create_resizable_container(
        full_layout.into(),
        5.0, // 边缘触发区域大小
        |direction| MainMessage::ResizeWindow(direction).into(),
        app.main_state.is_maximized, // 窗口是否已最大化
    );

    resizable_layout
}
