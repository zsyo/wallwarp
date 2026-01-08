use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

/// 动态图类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AnimatedImageType {
    Gif,
    Static,
}

/// 检测图片是否为动态图
pub fn detect_animated_image(path: &Path) -> AnimatedImageType {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "gif" => AnimatedImageType::Gif,
        _ => AnimatedImageType::Static,
    }
}

/// 动态图帧信息
#[derive(Debug, Clone)]
pub struct AnimatedFrame {
    pub handle: iced::widget::image::Handle,
    pub delay: Duration,
}

/// 动态图解码器
#[derive(Debug)]
pub struct AnimatedDecoder {
    frames: Vec<AnimatedFrame>,
    current_frame: usize,
    last_frame_time: std::time::Instant,
}

impl AnimatedDecoder {
    /// 从文件路径创建动态图解码器
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let image_type = detect_animated_image(path);

        match image_type {
            AnimatedImageType::Gif => Self::from_gif(path),
            AnimatedImageType::Static => Err("Not an animated image".to_string()),
        }
    }

    /// 解码 GIF 动态图
    fn from_gif(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open GIF file: {}", e))?;

        // 配置 GIF 解码选项，设置为 RGBA 格式
        let mut decoder = gif::DecodeOptions::new();
        decoder.set_color_output(gif::ColorOutput::RGBA);

        let decoder = decoder
            .read_info(BufReader::new(file))
            .map_err(|e| format!("Failed to decode GIF: {}", e))?;

        let mut frames = Vec::new();
        let mut canvas: Option<Vec<u8>> = None;
        let mut previous_canvas: Option<Vec<u8>> = None;
        let mut width = 0;
        let mut height = 0;

        // 使用 into_iter() 来迭代帧
        for frame_result in decoder.into_iter() {
            let frame = frame_result.map_err(|e| format!("Failed to read GIF frame: {}", e))?;

            // 初始化画布
            if canvas.is_none() {
                width = frame.width as usize;
                height = frame.height as usize;
                canvas = Some(vec![0u8; width * height * 4]);
            }

            let canvas = canvas.as_mut().unwrap();

            // 根据 dispose 方法处理帧
            match frame.dispose {
                gif::DisposalMethod::Keep => {
                    // 保留当前画布，新帧覆盖在上面
                }
                gif::DisposalMethod::Background => {
                    // 恢复到背景色（透明）
                    canvas.fill(0);
                }
                gif::DisposalMethod::Previous => {
                    // 恢复到前一帧
                    if let Some(ref prev) = previous_canvas {
                        canvas.copy_from_slice(prev);
                    } else {
                        canvas.fill(0);
                    }
                }
                gif::DisposalMethod::Any => {
                    // 任意方式，通常等同于 Keep
                }
            }

            // 将当前帧的数据绘制到画布上
            // frame.buffer 包含当前帧的像素数据
            // 需要将这些像素绘制到正确的位置
            let frame_data = &frame.buffer;
            let frame_width = frame.width as usize;
            let frame_height = frame.height as usize;
            let frame_left = frame.left as usize;
            let frame_top = frame.top as usize;

            // 逐行复制帧数据到画布
            for y in 0..frame_height {
                let canvas_y = frame_top + y;
                if canvas_y < height {
                    let canvas_row_start = canvas_y * width * 4;
                    let frame_row_start = y * frame_width * 4;

                    for x in 0..frame_width {
                        let canvas_x = frame_left + x;
                        if canvas_x < width {
                            let canvas_pixel_start = canvas_row_start + canvas_x * 4;
                            let frame_pixel_start = frame_row_start + x * 4;

                            // 检查是否为透明像素
                            let alpha = frame_data[frame_pixel_start + 3];
                            if alpha > 0 {
                                // 非透明像素，复制到画布
                                canvas[canvas_pixel_start] = frame_data[frame_pixel_start];
                                canvas[canvas_pixel_start + 1] = frame_data[frame_pixel_start + 1];
                                canvas[canvas_pixel_start + 2] = frame_data[frame_pixel_start + 2];
                                canvas[canvas_pixel_start + 3] = alpha;
                            }
                        }
                    }
                }
            }

            // 创建图像句柄 - 使用 from_rgba 方法
            let handle = iced::widget::image::Handle::from_rgba(width as u32, height as u32, canvas.to_vec());

            // GIF 帧延迟单位通常是 1/100 秒
            let delay = if frame.delay == 0 {
                Duration::from_millis(100) // 默认 100ms
            } else {
                Duration::from_millis(frame.delay as u64 * 10)
            };

            frames.push(AnimatedFrame { handle, delay });

            // 保存当前画布作为前一帧
            previous_canvas = Some(canvas.to_vec());
        }

        if frames.is_empty() {
            return Err("GIF has no frames".to_string());
        }

        println!("GIF 解码成功，共 {} 帧", frames.len());

        Ok(Self {
            frames,
            current_frame: 0,
            last_frame_time: std::time::Instant::now(),
        })
    }

    /// 获取当前帧
    pub fn current_frame(&self) -> &AnimatedFrame {
        &self.frames[self.current_frame]
    }

    /// 更新到下一帧（如果需要）
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

    /// 重置播放到第一帧
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.last_frame_time = std::time::Instant::now();
    }

    /// 获取帧总数
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}
