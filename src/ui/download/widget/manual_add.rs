// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 手动添加下载任务的输入行

use crate::i18n::I18n;
use crate::ui::common;
use crate::ui::common::create_colored_button;
use crate::ui::download::state::DownloadStateFull;
use crate::ui::download::DownloadMessage;
use crate::ui::style::{BUTTON_COLOR_BLUE, ThemeConfig};
use crate::ui::AppMessage;
use iced::widget::{column, container, row, space, text_input};
use iced::{Alignment, Element, Length};

/// 创建"手动添加任务"输入行
pub fn create_manual_add_row<'a>(
    i18n: &'a I18n,
    download_state: &'a DownloadStateFull,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let theme_colors = theme_config.get_theme_colors();

    let url_input = text_input(
        i18n.t("download-tasks.manual-url-placeholder").as_str(),
        &download_state.manual_url,
    )
    .width(Length::Fill)
    .on_input(|url| DownloadMessage::ManualUrlChanged(url).into())
    .on_submit(DownloadMessage::ManualUrlSubmitted.into())
    .padding([6, 10])
    .size(14)
    .style(common::styled_text_input(theme_colors));

    let add_button = create_colored_button(
        i18n.t("download-tasks.add-task").to_string(),
        BUTTON_COLOR_BLUE,
        DownloadMessage::ManualUrlSubmitted.into(),
    );

    let input_row = row![url_input, space::Space::new().width(10), add_button,]
        .align_y(Alignment::Center);

    container(column![input_row.width(Length::Fill), space::Space::new().height(4)].width(Length::Fill))
        .width(Length::Fill)
        .padding(iced::Padding::new(10.0).top(14.0))
        .into()
}
