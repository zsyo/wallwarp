// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 模态窗口图片下载进度环图像生成
//!
//! 不使用 canvas 绘制：当前 iced 0.14 的 canvas 网格管线存在渲染缺陷
//! （通用路径填充与弧线描边不显示，详见隔离测试），而图片管线渲染可靠
//! （应用 logo 与壁纸缩略图均走此管线）。按像素生成 96x96 抗锯齿
//! 透明底环形图，经 `iced::widget::image` 显示。

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use iced::Color;
use iced::widget::image::Handle;

/// 环形指示器边长（逻辑像素）
const RING_SIZE: u32 = 96;

/// 环带线宽（逻辑像素）
const RING_STROKE: f32 = 6.0;

/// 进度量化步进（1%）：下载进行中消息高频，未量化则每次 view 重建
/// 都重新分配并绘制位图
const PROGRESS_STEP: f32 = 0.01;

/// 位图缓存键：1% 量化进度档位 + 弧/轨道颜色（8bit RGBA）
type RingCacheKey = (u32, [u8; 4], [u8; 4]);

/// 渲染环形进度指示器图像（透明底、边缘抗锯齿）
///
/// - `progress`: 进度 0.0~1.0，<=0 或未知时仅绘制轨道圈；
/// - `ring_color`: 进度弧颜色（强调色）；
/// - `track_color`: 轨道圈颜色（如遮罩文字色 25% 透明度）。
///
/// 结果按 1% 步进量化缓存，同一档位与配色的重复请求直接复用 `Handle`
pub fn progress_ring_image(progress: f32, ring_color: Color, track_color: Color) -> Handle {
    let steps = ((progress.clamp(0.0, 1.0) / PROGRESS_STEP).round() as u32).min(100);
    let key: RingCacheKey = (steps, color_key(ring_color), color_key(track_color));

    let mut cache = ring_cache().lock().unwrap();
    if let Some(handle) = cache.get(&key) {
        return handle.clone();
    }
    let handle = render_progress_ring(steps as f32 * PROGRESS_STEP, ring_color, track_color);
    // 进度环同一时刻仅存在一个实例，配色随明暗主题切换而变化：
    // 换配色时旧条目整体过期，清空以把缓存内存上界控制在单套配色（约 3.7MB）
    if !cache.is_empty() && !cache.keys().all(|k| k.1 == key.1 && k.2 == key.2) {
        cache.clear();
    }
    cache.insert(key, handle.clone());
    handle
}

fn ring_cache() -> &'static Mutex<HashMap<RingCacheKey, Handle>> {
    static CACHE: OnceLock<Mutex<HashMap<RingCacheKey, Handle>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 颜色转 8bit RGBA 作为缓存键（与位图像素写入口径一致）
fn color_key(color: Color) -> [u8; 4] {
    [
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
        (color.a * 255.0).round() as u8,
    ]
}

/// 按像素绘制环形进度位图
///
/// - `progress`: 进度 0.0~1.0，<=0 或未知时仅绘制轨道圈；
/// - `ring_color`: 进度弧颜色（强调色）；
/// - `track_color`: 轨道圈颜色（如遮罩文字色 25% 透明度）。
fn render_progress_ring(progress: f32, ring_color: Color, track_color: Color) -> Handle {
    let size = RING_SIZE;
    let mut pixels = vec![0u8; (size * size * 4) as usize];

    let half = size as f32 / 2.0;
    // 外缘预留 1px 抗锯齿余量，确保整个环带（含羽化边）完整落在图像边界内
    let r_outer = half - 1.0;
    let r_inner = r_outer - RING_STROKE;
    let progress = progress.clamp(0.0, 1.0);
    let sweep = 2.0 * std::f32::consts::PI * progress;
    let two_pi = 2.0 * std::f32::consts::PI;
    let arc_start = -std::f32::consts::FRAC_PI_2;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 + 0.5 - half;
            let dy = y as f32 + 0.5 - half;
            let dist = (dx * dx + dy * dy).sqrt();

            // 到环带的距离（0 表示在带内，外缘 0.5px 抗锯齿过渡）
            let band_dist = if dist < r_inner {
                r_inner - dist
            } else if dist > r_outer {
                dist - r_outer
            } else {
                0.0
            };
            if band_dist > 0.5 {
                continue;
            }
            let band_alpha = (1.0 - band_dist / 0.5).clamp(0.0, 1.0);

            // 自顶部（-90°）顺时针到该点的弧度（屏幕 y 轴向下，atan2 随顺时针增大）
            let angle = dy.atan2(dx).rem_euclid(two_pi);
            let from_top = (angle - arc_start).rem_euclid(two_pi);

            // 进度弧覆盖 [0, sweep]，其余为轨道；弧边界沿环向做 1px 抗锯齿
            let (color, arc_alpha) = if progress > 0.0 && from_top <= sweep {
                let edge = ((sweep - from_top) * dist).clamp(-0.5, 0.5) + 0.5;
                (ring_color, edge)
            } else {
                (track_color, 1.0)
            };

            let alpha = band_alpha * arc_alpha * color.a;
            if alpha <= 0.0 {
                continue;
            }

            let idx = ((y * size + x) * 4) as usize;
            pixels[idx] = (color.r * 255.0).round() as u8;
            pixels[idx + 1] = (color.g * 255.0).round() as u8;
            pixels[idx + 2] = (color.b * 255.0).round() as u8;
            pixels[idx + 3] = (alpha * 255.0).round() as u8;
        }
    }

    Handle::from_rgba(size, size, pixels)
}
