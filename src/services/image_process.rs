// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 壁纸可读性处理：为壁纸生成暗化/模糊副本，避免浅色壁纸下桌面图标看不清
//!
//! 系统壁纸指向副本而非原图；副本以"源文件内容哈希 + 处理参数"为缓存键，
//! 源文件与参数未变时直接复用，不再重复处理

use crate::services::local::LocalWallpaperService;
use crate::utils::config::WallpaperReadability;
use std::path::Path;
use tracing::info;

/// 处理副本在缓存目录下的子目录名
const PROCESSED_DIR: &str = "processed";
/// 高斯模糊强度（sigma）
const BLUR_SIGMA: f32 = 8.0;
/// 暗化遮罩强度（0.0-1.0，即黑色遮罩的不透明度）
const DIM_OPACITY: f32 = 0.45;
/// JPEG 编码质量
const JPEG_QUALITY: u8 = 90;

/// 为壁纸生成可读性处理副本（阻塞操作，调用方需置于 spawn_blocking 中）
///
/// # 参数
/// - `image_path`: 源壁纸路径
/// - `option`: 处理选项（`None` 时原样返回源路径）
/// - `cache_path`: 缓存根目录（副本写入 `<cache_path>/processed/`）
///
/// # 返回
/// 处理后副本路径；选项为 `None` 时返回源路径
pub fn process_for_wallpaper(
    image_path: &str,
    option: WallpaperReadability,
    cache_path: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    if option == WallpaperReadability::None {
        return Ok(image_path.to_string());
    }

    let source = Path::new(image_path);
    // 源文件缺失交给调用方统一报错（如 set_wallpaper 的存在性检查），此处不兜底
    let file_hash = LocalWallpaperService::calculate_file_hash(source)?;

    let processed_dir = Path::new(cache_path).join(PROCESSED_DIR);
    std::fs::create_dir_all(&processed_dir)?;
    let output_path = processed_dir.join(format!("{}_{}.jpg", file_hash, option.as_str()));

    if output_path.exists() {
        return Ok(output_path.to_string_lossy().to_string());
    }

    let start = std::time::Instant::now();
    let img = image::open(source)?.into_rgb8();

    // 先模糊后暗化，遮罩在模糊结果上均匀叠加
    let img = if matches!(option, WallpaperReadability::Blur | WallpaperReadability::DimBlur) {
        image::imageops::blur(&img, BLUR_SIGMA)
    } else {
        img
    };
    let img = if matches!(option, WallpaperReadability::Dim | WallpaperReadability::DimBlur) {
        dim_image(img)
    } else {
        img
    };

    let file = std::fs::File::create(&output_path)?;
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(file, JPEG_QUALITY);
    img.write_with_encoder(encoder)?;

    info!(
        "[壁纸处理] [文件:{}] 生成 {:?} 副本: {}，耗时 {:?}",
        source.display(),
        option,
        output_path.display(),
        start.elapsed()
    );
    Ok(output_path.to_string_lossy().to_string())
}

/// 为 RGB 图像叠加暗化遮罩（各通道按 (1 - opacity) 线性压暗）
fn dim_image(
    mut img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let scale = 1.0 - DIM_OPACITY;
    for px in img.pixels_mut() {
        *px = image::Rgb([
            (px[0] as f32 * scale) as u8,
            (px[1] as f32 * scale) as u8,
            (px[2] as f32 * scale) as u8,
        ]);
    }
    img
}
