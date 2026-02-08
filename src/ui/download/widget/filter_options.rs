// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::{DownloadStateFull, DownloadStatus};
use crate::ui::style::ThemeConfig;
use iced::widget::{button, column, opaque, row, text};
use iced::{Alignment, Color, Element, Length};

/// 创建筛选选项列表
pub fn create_filter_options<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 所有状态选项
    let status_options = vec![
        (None, i18n.t("download-tasks.filter-all")),
        (Some(DownloadStatus::Waiting), i18n.t("download-tasks.status-waiting")),
        (
            Some(DownloadStatus::Downloading),
            i18n.t("download-tasks.status-downloading"),
        ),
        (Some(DownloadStatus::Paused), i18n.t("download-tasks.status-paused")),
        (
            Some(DownloadStatus::Completed),
            i18n.t("download-tasks.status-completed"),
        ),
        (
            Some(DownloadStatus::Failed(String::new())),
            i18n.t("download-tasks.status-failed"),
        ),
        (
            Some(DownloadStatus::Cancelled),
            i18n.t("download-tasks.status-cancelled"),
        ),
    ];

    let options: Vec<Element<'a, AppMessage>> = status_options
        .into_iter()
        .map(|(status, label)| {
            let is_selected = download_state
                .status_filter
                .as_ref()
                .map_or(false, |s| status.as_ref().map_or(false, |opt| opt.matches(s)));
            let is_selected_fixed = is_selected;

            button(
                row![
                    text(label)
                        .size(14)
                        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(theme_colors.text),
                        }),
                    if is_selected_fixed {
                        text("\u{F26E}") // Check
                            .font(iced::Font::with_name("bootstrap-icons"))
                            .size(14)
                            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(theme_colors.primary),
                            })
                    } else {
                        text("")
                    }
                ]
                .spacing(10)
                .align_y(Alignment::Center)
                .width(Length::Fill),
            )
            .style(
                move |_theme: &iced::Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
                    background: if is_selected_fixed {
                        Some(iced::Background::Color(Color {
                            r: theme_colors.primary.r * 0.1,
                            g: theme_colors.primary.g * 0.1,
                            b: theme_colors.primary.b * 0.1,
                            a: 1.0,
                        }))
                    } else {
                        Some(iced::Background::Color(theme_colors.light_button))
                    },
                    text_color: theme_colors.text,
                    border: iced::Border {
                        color: theme_colors.border,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                },
            )
            .padding([8, 12])
            .width(Length::Fill)
            .on_press(AppMessage::Download(DownloadMessage::SetStatusFilter(status)))
            .into()
        })
        .collect();

    opaque(
        column(options)
            .width(Length::Fixed(120.0))
            // .width(Length::Fill)
            .padding(5)
            .spacing(2),
    )
    .into()
}
