// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::style::ThemeConfig;
use iced::widget::{Space, column, text};
use iced::{Alignment, Element, Length};

/// 创建筛选后的空状态界面（保留表头）
pub fn create_filtered_empty_state<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let empty_text = if download_state.tasks.is_empty() {
        // 完全没有任务
        i18n.t("download-tasks.no-tasks")
    } else {
        // 有任务但被筛选掉了
        match &download_state.status_filter {
            None => i18n.t("download-tasks.no-tasks"),
            Some(status) => {
                let status_key = status.get_translation_key();
                let status_text = i18n.t(status_key);
                format!(
                    "{}{}{}",
                    i18n.t("download-tasks.no"),
                    status_text,
                    i18n.t("download-tasks.tasks")
                )
            }
        }
    };

    let hint_text = if download_state.tasks.is_empty() {
        i18n.t("download-tasks.no-tasks-hint")
    } else {
        i18n.t("download-tasks.no-filtered-tasks-hint")
    };

    // 创建表头
    let header = super::create_table_header(i18n, theme_config);

    let icon = text("\u{F30A}")
        .font(iced::Font::with_name("bootstrap-icons"))
        .size(48.0)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    let empty_text_elem = text(empty_text)
        .size(16)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme_colors.text),
        });

    let hint_text_elem = text(hint_text)
        .size(14)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(theme_colors.light_text_sub),
        });

    column![
        header,
        super::create_horizontal_separator(theme_config),
        Space::new().height(Length::Fixed(250.0)),
        icon,
        empty_text_elem,
        hint_text_elem
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .into()
}
