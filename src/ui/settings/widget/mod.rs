// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod about_link;
mod config_section;
mod info_row;
mod language_picker;
mod log_level_picker;
mod logs_path;
mod path_config;
mod placeholder_section;
mod proxy_protocol_picker;
mod section_about_info;
mod section_data_config;
mod section_general;
mod section_hotkeys;
mod section_network;
mod section_sources;
mod section_wallpaper_config;
mod setting_row;
mod settings_sorting_picker;
mod settings_time_range_picker;
mod theme_picker;

// 供本目录各区块文件经 `super::` 使用的内部构件
use {
    about_link::create_about_link_row,
    config_section::create_config_section,
    info_row::create_info_row,
    language_picker::create_language_picker,
    log_level_picker::create_log_level_picker,
    logs_path::create_logs_path_row,
    path_config::{PathRowActions, create_path_config_row},
    placeholder_section::{create_coming_soon_badge, create_placeholder_section},
    proxy_protocol_picker::create_proxy_protocol_picker,
    setting_row::{create_full_width_row, create_setting_row},
    settings_sorting_picker::create_sorting_picker,
    settings_time_range_picker::create_time_range_picker,
    theme_picker::create_theme_picker,
};

// 供 settings::view 使用的区块入口
pub(in crate::ui::settings) use {
    section_about_info::create_about_info_section,
    section_data_config::create_data_config_section,
    section_general::create_general_section,
    section_hotkeys::create_hotkeys_section,
    section_network::create_network_section,
    section_sources::create_sources_section,
    section_wallpaper_config::create_wallpaper_config_section,
};
