// Copyright (C) 2026 zsyo - GNU GPL v3.0

mod apply_group;
mod build_search_params;
mod check_thumbs;
mod copy_image_link;
mod download_from_cache;
mod download_wallpaper;
mod filter;
mod filter_basic;
mod filter_widget_state;
mod load_next_page;
mod load_page;
mod load_page_result;
mod load_wallpapers;
mod load_wallpapers_result;
mod modal_image;
mod modal_image_download;
mod modal_image_progress;
mod next_image;
mod previous_image;
mod refresh;
mod resolve_online_file;
mod scroll_to_bottom;
mod search;
mod set_online_wallpaper;
mod set_wallpaper_from_cache;
mod source;
mod thumb_load_failed;
mod thumb_loaded;
mod toggle_favorite;

pub(in crate::ui) use resolve_online_file::OnlineFileHit;
