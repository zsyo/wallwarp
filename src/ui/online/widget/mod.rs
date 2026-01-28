// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod diagonal_line;
mod filter;
mod filter_color_grid_options;
mod filter_color_picker;
mod filter_ratio_grid_options;
mod filter_ratio_picker;
mod filter_resolution_grid_options;
mod filter_resolution_picker;
mod filter_sorting_picker;
mod filter_time_range_picker;
mod list_loaded_wallpaper_with_thumb;
mod list_loading_placeholder;
mod list_page_separator;
mod list_wallpaper_grid;
mod list_wallpapers;
mod modal;
mod modal_loading_placeholder;

use {
    diagonal_line::DiagonalLine, filter_color_grid_options::create_color_grid_options,
    filter_color_picker::create_color_picker, filter_ratio_grid_options::create_ratio_grid_options,
    filter_ratio_picker::create_ratio_picker, filter_resolution_grid_options::create_resolution_grid_options,
    filter_resolution_picker::create_resolution_picker, filter_sorting_picker::create_sorting_picker,
    filter_time_range_picker::create_time_range_picker,
};

use {
    list_loaded_wallpaper_with_thumb::create_loaded_wallpaper_with_thumb,
    list_loading_placeholder::create_loading_placeholder, list_page_separator::create_page_separator,
    list_wallpaper_grid::create_wallpaper_grid,
};

use modal_loading_placeholder::create_modal_loading_placeholder;

pub(in crate::ui::online) use {
    filter::create_filter_bar, list_wallpapers::create_wallpaper_list, modal::create_modal,
};
