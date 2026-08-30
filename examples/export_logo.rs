// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 从 assets/logo.ico 导出各平台打包所需的 PNG 图标
//!
//! 用法：`cargo run --release --example export_logo`

use image::imageops::FilterType;

fn main() {
    let img =
        image::load_from_memory(include_bytes!("../assets/logo.ico")).expect("解码 logo.ico 失败");
    println!("源图标尺寸: {}x{}", img.width(), img.height());

    for size in [16u32, 32, 64, 128, 256, 512] {
        let resized = img.resize_exact(size, size, FilterType::Lanczos3);
        let path = format!("assets/logo-{size}.png");
        resized.save(&path).expect("保存 PNG 失败");
        println!("已生成 {path}");
    }
}
