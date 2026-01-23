// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use tracing::info;

const DEFAULT_FRAME_DELAY_MS: u64 = 100;
const GIF_DELAY_MULTIPLIER: u64 = 10;
const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, Clone, PartialEq)]
pub enum AnimatedImageType {
    Gif,
    Static,
}

pub fn detect_animated_image(path: &Path) -> AnimatedImageType {
    let extension = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()).unwrap_or_default();

    match extension.as_str() {
        "gif" => AnimatedImageType::Gif,
        _ => AnimatedImageType::Static,
    }
}

#[derive(Debug, Clone)]
pub struct AnimatedFrame {
    pub handle: iced::widget::image::Handle,
    pub delay: Duration,
}

#[derive(Debug)]
pub struct AnimatedDecoder {
    frames: Vec<AnimatedFrame>,
    current_frame: usize,
    last_frame_time: std::time::Instant,
}

impl AnimatedDecoder {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let image_type = detect_animated_image(path);

        match image_type {
            AnimatedImageType::Gif => Self::from_gif(path),
            AnimatedImageType::Static => Err("Not an animated image".to_string()),
        }
    }

    fn from_gif(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open GIF file: {}", e))?;

        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);

        let decoder = decoder.read_info(BufReader::new(file)).map_err(|e| format!("Failed to decode GIF: {}", e))?;

        let mut frames = Vec::new();
        let mut canvas: Option<Vec<u8>> = None;
        let mut previous_canvas: Option<Vec<u8>> = None;
        let mut width = 0;
        let mut height = 0;

        for frame_result in decoder.into_iter() {
            let frame = frame_result.map_err(|e| format!("Failed to read GIF frame: {}", e))?;

            if canvas.is_none() {
                width = frame.width as usize;
                height = frame.height as usize;
                canvas = Some(vec![0u8; width * height * BYTES_PER_PIXEL]);
            }

            let canvas = canvas.as_mut().unwrap();

            match frame.dispose {
                gif::DisposalMethod::Keep => {}
                gif::DisposalMethod::Background => {
                    canvas.fill(0);
                }
                gif::DisposalMethod::Previous => {
                    if let Some(ref prev) = previous_canvas {
                        canvas.copy_from_slice(prev);
                    } else {
                        canvas.fill(0);
                    }
                }
                gif::DisposalMethod::Any => {}
            }

            let frame_data = &frame.buffer;
            let frame_width = frame.width as usize;
            let frame_height = frame.height as usize;
            let frame_left = frame.left as usize;
            let frame_top = frame.top as usize;

            for y in 0..frame_height {
                let canvas_y = frame_top + y;
                if canvas_y < height {
                    let canvas_row_start = canvas_y * width * BYTES_PER_PIXEL;
                    let frame_row_start = y * frame_width * BYTES_PER_PIXEL;

                    for x in 0..frame_width {
                        let canvas_x = frame_left + x;
                        if canvas_x < width {
                            let canvas_pixel_start = canvas_row_start + canvas_x * BYTES_PER_PIXEL;
                            let frame_pixel_start = frame_row_start + x * BYTES_PER_PIXEL;

                            let alpha = frame_data[frame_pixel_start + 3];
                            if alpha > 0 {
                                canvas[canvas_pixel_start] = frame_data[frame_pixel_start];
                                canvas[canvas_pixel_start + 1] = frame_data[frame_pixel_start + 1];
                                canvas[canvas_pixel_start + 2] = frame_data[frame_pixel_start + 2];
                                canvas[canvas_pixel_start + 3] = alpha;
                            }
                        }
                    }
                }
            }

            let handle = iced::widget::image::Handle::from_rgba(width as u32, height as u32, canvas.to_vec());

            let delay = if frame.delay == 0 {
                Duration::from_millis(DEFAULT_FRAME_DELAY_MS)
            } else {
                Duration::from_millis(frame.delay as u64 * GIF_DELAY_MULTIPLIER)
            };

            frames.push(AnimatedFrame { handle, delay });

            previous_canvas = Some(canvas.to_vec());
        }

        if frames.is_empty() {
            return Err("GIF has no frames".to_string());
        }

        info!("GIF 解码成功，{path:?} 共 {} 帧", frames.len());

        Ok(Self {
            frames,
            current_frame: 0,
            last_frame_time: std::time::Instant::now(),
        })
    }

    pub fn current_frame(&self) -> &AnimatedFrame {
        &self.frames[self.current_frame]
    }

    pub fn update(&mut self) -> bool {
        if self.frames.len() <= 1 {
            return false;
        }

        let elapsed = self.last_frame_time.elapsed();
        let current_delay = self.frames[self.current_frame].delay;

        if elapsed >= current_delay {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_frame_time = std::time::Instant::now();
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = std::time::Instant::now();
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}
