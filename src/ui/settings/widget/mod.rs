// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod about_link;
mod config_section;
mod info_row;
mod language_picker;
mod logs_path;
mod path_config;
mod proxy_protocol_picker;
mod section_about_info;
mod section_api_config;
mod section_data_config;
mod section_system_config;
mod section_wallpaper_config;
mod setting_row;
mod theme_picker;

use {
    about_link::create_about_link_row, config_section::create_config_section, info_row::create_info_row,
    language_picker::create_language_picker, logs_path::create_logs_path_row, path_config::create_path_config_row,
    proxy_protocol_picker::create_proxy_protocol_picker, setting_row::create_setting_row,
    theme_picker::create_theme_picker,
};

pub(in crate::ui::settings) use {
    section_about_info::create_about_info_section, section_api_config::create_api_config_section,
    section_data_config::create_data_config_section, section_system_config::create_system_config_section,
    section_wallpaper_config::create_wallpaper_config_section,
};
