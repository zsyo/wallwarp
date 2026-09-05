// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::style::{
    MENU_ICON_SIZE, MENU_ITEM_HEIGHT, RADIUS_MD, SIDEBAR_INDICATOR_WIDTH, ThemeConfig, tint,
};
use crate::ui::{ActivePage, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Font, Length, Padding};

/// 菜单项对应的 bootstrap 图标
/// （码点已对照 assets/icons.ttf 验证：house-door=f423, folder=f3d7,
///  download=f30a, gear=f3e5）
fn page_icon(page: ActivePage) -> &'static str {
    match page {
        ActivePage::OnlineWallpapers => "\u{F423}", // bootstrap-icons: house-door
        ActivePage::LocalList => "\u{F3D7}",        // bootstrap-icons: folder
        ActivePage::DownloadProgress => "\u{F30A}", // bootstrap-icons: download
        ActivePage::WallpaperHistory => "\u{F292}", // bootstrap-icons: clock-history
        ActivePage::Settings => "\u{F3E5}",         // bootstrap-icons: gear
    }
}

/// 创建侧边栏菜单按钮
///
/// 选中态：强调色淡染底 + 强调色文字 + 右侧指示条；悬停：中性淡染底。
pub fn create_menu_button<'a>(
    label: String,
    current_page: ActivePage,
    target_page: ActivePage,
    theme_config: &'a ThemeConfig,
) -> button::Button<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();
    let is_selected = current_page == target_page;

    let text_color = if is_selected {
        theme_colors.primary
    } else {
        theme_colors.text
    };

    let button_content = row![
        text(page_icon(target_page))
            .font(Font::with_name("bootstrap-icons"))
            .size(MENU_ICON_SIZE)
            .color(text_color),
        text(label).color(text_color),
        Space::new().width(Length::Fill),
        container(Space::new())
            .width(Length::Fixed(SIDEBAR_INDICATOR_WIDTH))
            .height(Length::Fixed(MENU_ICON_SIZE))
            .style(move |_theme| container::Style {
                background: if is_selected {
                    Some(iced::Background::Color(theme_colors.sidebar_indicator))
                } else {
                    None
                },
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(SIDEBAR_INDICATOR_WIDTH / 2.0),
                },
                ..Default::default()
            }),
    ]
    .spacing(10)
    // 撑满按钮内容区（按钮高度固定，内容在 padding 内顶对齐），
    // 使图标/文字/指示条在按钮块内垂直居中
    .height(Length::Fill)
    .align_y(Alignment::Center);

    button(button_content)
        .on_press_maybe(if current_page != target_page {
            Some(MainMessage::PageSelected(target_page).into())
        } else {
            None
        })
        .padding(Padding {
            top: 6.0,
            right: 8.0,
            bottom: 6.0,
            left: 14.0,
        })
        .height(Length::Fixed(MENU_ITEM_HEIGHT))
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme, status| {
            let bg_color = if is_selected {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        tint(theme_colors.primary, 0.18)
                    }
                    _ => theme_colors.sidebar_button_selected,
                }
            } else {
                match status {
                    button::Status::Hovered => theme_colors.sidebar_button_hover,
                    button::Status::Pressed => tint(theme_colors.primary, 0.08),
                    _ => theme_colors.sidebar_button_default,
                }
            };

            button::Style {
                background: Some(iced::Background::Color(bg_color)),
                text_color,
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(RADIUS_MD),
                },
                ..button::text(_theme, status)
            }
        })
}
