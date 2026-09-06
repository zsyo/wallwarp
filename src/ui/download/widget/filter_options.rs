// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common::drop_down::{dropdown_option_style, dropdown_panel_style};
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::{DownloadStateFull, DownloadStatus};
use crate::ui::style::ThemeConfig;
use iced::widget::{button, column, container, opaque, row, text};
use iced::{Alignment, Element, Length};

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
        (
            Some(DownloadStatus::Waiting),
            i18n.t("download-tasks.status-waiting"),
        ),
        (
            Some(DownloadStatus::Downloading),
            i18n.t("download-tasks.status-downloading"),
        ),
        (
            Some(DownloadStatus::Paused),
            i18n.t("download-tasks.status-paused"),
        ),
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
                .is_some_and(|s| status.as_ref().is_some_and(|opt| opt.matches(s)));
            let is_selected_fixed = is_selected;

            button(
                row![
                    text(label).size(14),
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
            .style(dropdown_option_style(theme_colors, is_selected_fixed))
            .padding([8, 12])
            .width(Length::Fill)
            .on_press(AppMessage::Download(DownloadMessage::SetStatusFilter(
                status,
            )))
            .into()
        })
        .collect();

    opaque(
        container(
            column(options)
                .width(Length::Fixed(120.0))
                .padding(5)
                .spacing(2),
        )
        .style(dropdown_panel_style(theme_colors)),
    )
}
