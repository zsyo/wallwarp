// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::download::message::DownloadMessage;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::style::ThemeConfig;
use iced::widget::{button, row, text};
use iced::{Alignment, Element, Length};
use iced_aw::{DropDown, drop_down};

/// 创建状态筛选下拉框
pub fn create_status_filter_dropdown<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    // 获取当前筛选状态的显示文本
    let filter_text = match &download_state.status_filter {
        None => i18n.t("download-tasks.filter-all"),
        Some(status) => i18n.t(status.get_translation_key()),
    };

    // 创建触发按钮
    let underlay = button(
        row![
            text(filter_text)
                .size(14)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(theme_colors.text),
                }),
            text(if download_state.status_filter_expanded {
                "\u{F282}" // ChevronUp
            } else {
                "\u{F285}" // ChevronDown
            })
            .font(iced::Font::with_name("bootstrap-icons"))
            .size(12)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(theme_colors.light_text_sub),
            }),
        ]
        .spacing(5)
        .align_y(Alignment::Center),
    )
    .style(
        move |_theme: &iced::Theme, _status: iced::widget::button::Status| iced::widget::button::Style {
            background: Some(iced::Background::Color(theme_colors.light_button)),
            text_color: theme_colors.text,
            border: iced::Border {
                color: theme_colors.border,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        },
    )
    .padding([5, 10])
    .on_press(AppMessage::Download(DownloadMessage::ToggleStatusFilter));

    // 创建下拉选项列表
    let filter_options = super::create_filter_options(i18n, download_state, theme_config);

    // 组合下拉框
    let dropdown = DropDown::new(underlay, filter_options, download_state.status_filter_expanded)
        .width(Length::Fill)
        .on_dismiss(AppMessage::Download(DownloadMessage::ToggleStatusFilter))
        .alignment(drop_down::Alignment::Bottom);

    dropdown.into()
}
