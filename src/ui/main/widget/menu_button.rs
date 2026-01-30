// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::style::SIDEBAR_INDICATOR_WIDTH;
use crate::ui::style::{ThemeColors, ThemeConfig};
use crate::ui::{ActivePage, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{Space, button, container, row, text};
use iced::{Alignment, Length, Padding};

pub fn create_menu_button<'a>(
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
