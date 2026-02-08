// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::{DownloadStateFull, DownloadStatus};
use crate::ui::style::{BUTTON_COLOR_RED, ThemeConfig};
use iced::widget::{button, row, text};
use iced::{Alignment, Element};

/// 创建删除所有已完成任务按钮
pub fn create_clear_completed_button<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 检查是否有已完成的任务
    let has_completed = download_state
        .tasks
        .iter()
        .any(|task| matches!(task.task.status, DownloadStatus::Completed));

    let button_content = row![
        text("\u{F78B}") // trash-fill
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(14)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if has_completed {
                    iced::Color::WHITE
                } else {
                    theme_colors.light_text_sub
                }),
            }),
        text(i18n.t("download-tasks.clear-completed"))
            .size(13)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(if has_completed {
                    iced::Color::WHITE
                } else {
                    theme_colors.light_text_sub
                }),
            }),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    let btn = if has_completed {
        button(button_content).style(move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                text_color: iced::Color::WHITE,
                background: Some(iced::Background::Color(BUTTON_COLOR_RED)),
                border: iced::Border {
                    color: BUTTON_COLOR_RED,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
    } else {
        button(button_content).style(move |_theme: &iced::Theme, _status: iced::widget::button::Status| {
            iced::widget::button::Style {
                text_color: theme_colors.light_text_sub,
                background: Some(iced::Background::Color(theme_colors.light_button)),
                border: iced::Border {
                    color: theme_colors.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
    };

    btn.on_press_maybe(if has_completed {
        Some(AppMessage::Download(DownloadMessage::ClearCompleted))
    } else {
        None
    })
    .padding([6, 12])
    .into()
}
