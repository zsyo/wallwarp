// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod content;
mod error_placeholder;
mod loaded_wallpaper;
mod loading_placeholder;
mod modal;
mod modal_loading_placeholder;

pub(in crate::ui::local) use {
    content::create_content, content::create_empty_content, error_placeholder::create_error_placeholder,
    loaded_wallpaper::create_loaded_wallpaper, loading_placeholder::create_loading_placeholder, modal::create_modal,
    modal_loading_placeholder::create_modal_loading_placeholder,
};
