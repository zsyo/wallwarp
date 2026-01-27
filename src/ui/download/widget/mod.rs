// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod download_display;
mod empty;
mod operation_buttons;
mod separator;
mod status_display;
mod table;
mod table_header;
mod table_row;

pub(self) use {
    download_display::create_download_display, operation_buttons::create_operation_buttons,
    separator::create_horizontal_separator, separator::create_vertical_separator,
    status_display::create_status_display, table_header::create_table_header, table_row::create_table_row,
};

pub(in crate::ui::download) use {empty::create_empty_state, table::create_table};
