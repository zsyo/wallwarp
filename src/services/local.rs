use fast_image_resize as fr;
use image;
use rayon::prelude::*;
use std::fs;
use std::io::Read;
use std::num::NonZeroU32;
use std::path::Path;
use xxhash_rust::xxh3::xxh3_128;

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp", "gif", "webp"];
const THREAD_POOL_SIZE: usize = 3;
const HASH_CHUNK_SIZE: u64 = 64 * 1024;
const THUMBNAIL_MAX_WIDTH: u32 = 256;
const THUMBNAIL_MAX_HEIGHT: u32 = 150;

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub thumbnail_path: String,
    pub file_size: u64,
    pub width: u32,
    pub height: u32,
}

impl Wallpaper {
    pub fn new(path: String, name: String, file_size: u64, width: u32, height: u32) -> Self {
        Self {
            path: path.clone(),
            name,
            thumbnail_path: path,
            file_size,
            width,
            height,
        }
    }

    pub fn with_thumbnail(path: String, name: String, thumbnail_path: String, file_size: u64, width: u32, height: u32) -> Self {
        Self {
            path,
            name,
            thumbnail_path,
            file_size,
            width,
            height,
        }
    }
}

pub struct LocalWallpaperService;

impl LocalWallpaperService {
    /// 设置壁纸
    pub fn set_wallpaper(image_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use std::path::Path;

        let path = Path::new(image_path);
        let absolute_path = if path.is_absolute() {
            image_path.to_string()
        } else {
            std::env::current_dir()?
                .join(path)
                .canonicalize()?
                .to_string_lossy()
                .to_string()
        };

        println!("[DEBUG] 设置壁纸路径: {}", absolute_path);

        // 使用 wallpaper 库设置壁纸
        match wallpaper::set_from_path(&absolute_path) {
            Ok(_) => {
                println!("[DEBUG] 壁纸设置成功");
                Ok(())
            }
            Err(e) => {
                println!("[DEBUG] 壁纸设置失败: {}", e);
                Err(format!("设置壁纸失败: {}", e).into())
            }
        }
    }
}

impl LocalWallpaperService {
    pub fn load_wallpapers_from_path(
        data_path: &str,
        cache_path: &str,
    ) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error + Send + Sync>> {
        let path = Path::new(data_path);
        let cache_dir = Path::new(cache_path);

        fs::create_dir_all(cache_dir).map_err(to_boxed_error)?;

        if !path.exists() {
            return Ok(Vec::new());
        }

        let wallpapers = Self::collect_wallpapers(path)?;

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(THREAD_POOL_SIZE)
            .build()
            .map_err(to_boxed_error)?;

        pool.install(|| {
            wallpapers
                .into_par_iter()
                .map(|wallpaper| {
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
                            wallpaper.file_size,
                            wallpaper.width,
                            wallpaper.height,
                        ))
                    })()
                })
                .collect()
        })
    }

    pub fn get_wallpaper_paths(
        data_path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let path = Path::new(data_path);

        if !path.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(path).map_err(to_boxed_error)?;
        let mut wallpaper_paths = Vec::new();

        for entry in entries {
            let entry = entry.map_err(to_boxed_error)?;
            let file_path = entry.path();

            if file_path.is_file() && Self::is_supported_image(&file_path) {
                wallpaper_paths.push(file_path.to_string_lossy().to_string());
            }
        }

        Ok(wallpaper_paths)
    }

    pub fn generate_thumbnail_for_path(
        wallpaper_path: &str,
        cache_path: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cache_dir = Path::new(cache_path);
        fs::create_dir_all(cache_dir).map_err(to_boxed_error)?;

        Self::generate_thumbnail(&Path::new(wallpaper_path), cache_dir)
    }

    fn calculate_file_hash(
        file_path: &Path,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        use std::io::Seek;

        let mut file = fs::File::open(file_path).map_err(to_boxed_error)?;

        let file_size = file
            .metadata()
            .map_err(to_boxed_error)?
            .len();

        if file_size <= HASH_CHUNK_SIZE * 2 {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(to_boxed_error)?;
            let hash = xxh3_128(&buffer);
            Ok(format!("{:x}", hash))
        } else {
            let mut buffer = Vec::with_capacity((HASH_CHUNK_SIZE * 2) as usize);

            file.seek(std::io::SeekFrom::Start(0)).map_err(to_boxed_error)?;
            let mut head_chunk = vec![0u8; HASH_CHUNK_SIZE as usize];
            file.read_exact(&mut head_chunk).map_err(to_boxed_error)?;
            buffer.extend_from_slice(&head_chunk);

            file.seek(std::io::SeekFrom::End(-(HASH_CHUNK_SIZE as i64)))
                .map_err(to_boxed_error)?;
            let mut tail_chunk = vec![0u8; HASH_CHUNK_SIZE as usize];
            file.read_exact(&mut tail_chunk).map_err(to_boxed_error)?;
            buffer.extend_from_slice(&tail_chunk);

            buffer.extend_from_slice(&file_size.to_be_bytes());

            let hash = xxh3_128(&buffer);
            Ok(format!("{:x}", hash))
        }
    }

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
        let img = image::open(file_path)?.into_rgba8();
        println!("{thumbnail_path:?}Load image: {:?}", start.elapsed());
        let (src_w, src_h) = img.dimensions();

        let ratio = (THUMBNAIL_MAX_WIDTH as f64 / src_w as f64)
            .min(THUMBNAIL_MAX_HEIGHT as f64 / src_h as f64)
            .min(1.0);
        let dst_w = NonZeroU32::new((src_w as f64 * ratio) as u32).unwrap();
        let dst_h = NonZeroU32::new((src_h as f64 * ratio) as u32).unwrap();

        let src_image = fr::images::Image::from_vec_u8(
            src_w,
            src_h,
            img.into_raw(),
            fr::PixelType::U8x4,
        )?;

        let mut dst_image =
            fr::images::Image::new(dst_w.get(), dst_h.get(), src_image.pixel_type());

        let mut resizer = fr::Resizer::new();
        println!("{thumbnail_path:?}Create resizer: {:?}", start.elapsed());
        resizer.resize(&src_image, &mut dst_image, None)?;
        println!("{thumbnail_path:?}Resize image: {:?}", start.elapsed());

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

    fn collect_wallpapers(
        path: &Path,
    ) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error + Send + Sync>> {
        let entries = fs::read_dir(path).map_err(to_boxed_error)?;
        let mut wallpapers = Vec::new();

        for entry in entries {
            let entry = entry.map_err(to_boxed_error)?;
            let file_path = entry.path();

            if file_path.is_file() && Self::is_supported_image(&file_path) {
                if let Some(file_name) = file_path.file_name() {
                    if let Some(name) = file_name.to_str() {
                        let file_size = fs::metadata(&file_path)
                            .map_err(to_boxed_error)?
                            .len();

                        let (width, height) = image::image_dimensions(&file_path)
                            .unwrap_or((0, 0));

                        wallpapers.push(Wallpaper::new(
                            file_path.to_string_lossy().to_string(),
                            name.to_string(),
                            file_size,
                            width,
                            height,
                        ));
                    }
                }
            }
        }

        Ok(wallpapers)
    }

    fn is_supported_image(file_path: &Path) -> bool {
        file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}

fn to_boxed_error<E: std::error::Error + Send + Sync + 'static>(
    err: E,
) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(err)
}
