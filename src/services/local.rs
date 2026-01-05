use fast_image_resize as fr;
use image;
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use std::num::NonZeroU32;
use std::path::Path;
use xxhash_rust::xxh3::xxh3_128;

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub thumbnail_path: String, // 添加缩略图路径
}

impl Wallpaper {
    pub fn new(path: String, name: String) -> Self {
        // 使用原始路径作为缩略图路径，后续会由服务更新
        Self {
            path: path.clone(),
            name,
            thumbnail_path: path, // 临时设置，后续更新
        }
    }

    pub fn with_thumbnail(path: String, name: String, thumbnail_path: String) -> Self {
        Self {
            path,
            name,
            thumbnail_path,
        }
    }
}

pub struct LocalWallpaperService;

impl LocalWallpaperService {
    /// 从指定路径读取图片文件列表，仅返回支持的图片格式
    pub fn load_wallpapers_from_path(
        data_path: &str,
        cache_path: &str,
    ) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error + Send + Sync>> {
        let path = Path::new(data_path);
        let cache_dir = Path::new(cache_path);

        // 确保缓存目录存在
        fs::create_dir_all(cache_dir)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut wallpapers = Vec::new();

        // 支持的图片格式扩展名
        let supported_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];

        let entries = fs::read_dir(path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        for entry in entries {
            let entry =
                entry.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let file_path = entry.path();

            // 检查是否为文件
            if file_path.is_file() {
                // 检查文件扩展名是否为支持的图片格式
                if let Some(extension) = file_path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            // 只检查文件是否存在和可读，而不是立即解码整个文件
                            // 解码操作将推迟到生成缩略图时进行
                            if let Some(file_name) = file_path.file_name() {
                                if let Some(name) = file_name.to_str() {
                                    wallpapers.push(Wallpaper::new(
                                        file_path.to_string_lossy().to_string(),
                                        name.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // 使用rayon进行并行处理，限制最多3个并发任务以减少资源消耗
        use rayon::ThreadPoolBuilder;

        // 设置线程池大小为最多3个线程，减少资源竞争
        let pool = ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        // 使用线程池并行处理缩略图生成
        let results: Result<Vec<Wallpaper>, Box<dyn std::error::Error + Send + Sync>> = pool
            .install(|| {
                wallpapers
                    .into_par_iter()
                    .map(|wallpaper| {
                        // 使用 Result 包装内部操作，避免线程池因单个错误而中断
                        (|| -> Result<Wallpaper, Box<dyn std::error::Error + Send + Sync>> {
                            let cache_dir_clone = cache_dir.to_path_buf();
                            let thumbnail_path = Self::generate_thumbnail(
                                &Path::new(&wallpaper.path),
                                &cache_dir_clone,
                            )?;

                            Ok(Wallpaper::with_thumbnail(
                                wallpaper.path,
                                wallpaper.name,
                                thumbnail_path,
                            ))
                        })()
                    })
                    .collect()
            });

        results
    }

    /// 获取指定路径下的壁纸文件路径列表（不生成缩略图），用于懒加载
    pub fn get_wallpaper_paths(
        data_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let path = Path::new(data_path);

        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut wallpaper_paths = Vec::new();

        // 支持的图片格式扩展名
        let supported_extensions = ["jpg", "jpeg", "png", "bmp", "gif", "webp"];

        let entries = fs::read_dir(path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        for entry in entries {
            let entry =
                entry.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let file_path = entry.path();

            // 检查是否为文件
            if file_path.is_file() {
                // 检查文件扩展名是否为支持的图片格式
                if let Some(extension) = file_path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            wallpaper_paths.push(file_path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        Ok(wallpaper_paths)
    }

    /// 为指定路径的壁纸生成缩略图
    pub fn generate_thumbnail_for_path(
        wallpaper_path: &str,
        cache_path: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cache_dir = Path::new(cache_path);
        fs::create_dir_all(cache_dir)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        Self::generate_thumbnail(&Path::new(wallpaper_path), cache_dir)
    }

    /// 生成图片的XXH3_128哈希值 - 优化版本，只读取文件头部和尾部的一部分数据
    fn calculate_file_hash(
        file_path: &Path,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        use std::io::Seek;

        let mut file = fs::File::open(file_path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        let file_size = file
            .metadata()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            .len();

        // 定义读取的块大小（例如64KB）
        let chunk_size = 64 * 1024u64; // 64KB

        if file_size <= chunk_size * 2 {
            // 如果文件较小，直接读取整个文件
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let hash = xxh3_128(&buffer);
            Ok(format!("{:x}", hash))
        } else {
            // 对于较大的文件，读取头部和尾部的块进行哈希计算
            let mut buffer = Vec::with_capacity((chunk_size * 2) as usize);

            // 读取文件头部
            file.seek(std::io::SeekFrom::Start(0))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let mut head_chunk = vec![0u8; chunk_size as usize];
            file.read_exact(&mut head_chunk)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            buffer.extend_from_slice(&head_chunk);

            // 读取文件尾部
            file.seek(std::io::SeekFrom::End(-(chunk_size as i64)))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            let mut tail_chunk = vec![0u8; chunk_size as usize];
            file.read_exact(&mut tail_chunk)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            buffer.extend_from_slice(&tail_chunk);

            // 再添加一个文件的大小
            buffer.extend_from_slice(&file_size.to_be_bytes());

            let hash = xxh3_128(&buffer);
            Ok(format!("{:x}", hash))
        }
    }

    /// 生成缩略图，使用fast_image_resize加速
    fn generate_thumbnail(
        file_path: &Path,
        cache_dir: &Path,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let file_hash = Self::calculate_file_hash(file_path)?;
        let thumbnail_path = cache_dir.join(format!("{}.webp", file_hash));

        if thumbnail_path.exists() {
            return Ok(thumbnail_path.to_string_lossy().to_string());
        }

        let start = std::time::Instant::now();
        // 1. 一次性加载并解码（优化点：避免重复 open 和 dimensions 获取）
        // 使用 .into_rgba8() 直接获取底层 Vec<u8>，避免后续 to_rgba8() 的拷贝
        let img = image::open(file_path)?.into_rgba8();
        println!("{thumbnail_path:?}Load image: {:?}", start.elapsed());
        let (src_w, src_h) = img.dimensions();

        // 2. 计算目标尺寸
        let max_w = 256u32;
        let max_h = 150u32;
        let ratio = (max_w as f64 / src_w as f64)
            .min(max_h as f64 / src_h as f64)
            .min(1.0);
        let dst_w = NonZeroU32::new((src_w as f64 * ratio) as u32).unwrap();
        let dst_h = NonZeroU32::new((src_h as f64 * ratio) as u32).unwrap();

        // 3. 包装源图像 (修正：使用 .get() 将 NonZeroU32 转为 u32)
        let src_image = fr::images::Image::from_vec_u8(
            src_w, // 直接传入 u32，或者如果你保留了 NonZero 变量就用 src_w_nonzero.get()
            src_h,
            img.into_raw(),
            fr::PixelType::U8x4,
        )?;

        // 4. 创建目标缓冲区 (修正：传入 u32)
        let mut dst_image =
            fr::images::Image::new(dst_w.get(), dst_h.get(), src_image.pixel_type());

        // 5. 执行缩放
        let mut resizer = fr::Resizer::new();
        println!("{thumbnail_path:?}Create resizer: {:?}", start.elapsed());
        resizer.resize(&src_image, &mut dst_image, None)?;
        println!("{thumbnail_path:?}Resize image: {:?}", start.elapsed());

        // 6. 保存为 WebP
        // 优化点：直接使用图像缓冲区创建预览并保存，减少到 DynamicImage 的转换
        let raw_parts = dst_image.into_vec();
        image::save_buffer_with_format(
            &thumbnail_path,
            &raw_parts,
            dst_w.get(),
            dst_h.get(),
            image::ColorType::Rgba8,
            image::ImageFormat::WebP,
        )?;

        Ok(thumbnail_path.to_string_lossy().to_string())
    }
}
