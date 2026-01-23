// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use image;

const LOGO: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.ico"));
pub const ICON_FONT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons.ttf"));

// 通用函数：将资源文件转为 (RGBA字节, 宽度, 高度)
fn load_rgba_from_bytes(bytes: &[u8], size: u32) -> (Vec<u8>, u32, u32) {
    let mut img = image::load_from_memory(bytes).expect("无法解码图片数据");

    if size > 0 && (img.width() != size || img.height() != size) {
        img = img.resize(size, size, image::imageops::FilterType::Lanczos3);
    }

    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    (rgba, width, height)
}

pub fn get_logo(size: u32) -> (Vec<u8>, u32, u32) {
    load_rgba_from_bytes(LOGO, size)
}
