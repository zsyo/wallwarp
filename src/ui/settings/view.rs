// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::settings::widget;
use crate::ui::style::{SCROLL_PADDING, SETTINGS_ROW_SPACING};
use crate::ui::{App, AppMessage};
use iced::widget::{Id, column, scrollable};
use iced::{Alignment, Element, Length};

pub fn settings_view(app: &App) -> Element<'_, AppMessage> {
    let system_config_section = widget::create_system_config_section(app);
    let data_config_section = widget::create_data_config_section(app);
    let api_config_section = widget::create_api_config_section(app);
    let wallpaper_config_section = widget::create_wallpaper_config_section(app);
    let about_info_section = widget::create_about_info_section(app);

    scrollable(
        column![
            system_config_section,
            data_config_section,
            api_config_section,
            wallpaper_config_section,
            about_info_section,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(SCROLL_PADDING)
        .spacing(SETTINGS_ROW_SPACING),
    )
    .height(Length::Fill)
    .id(Id::new("settings_scroll"))
    .into()
}
