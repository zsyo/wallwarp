use iced::window::icon as iced_icon;
use image::GenericImageView;
use include_dir::{Dir, include_dir};
use tray_icon::Icon as TrayIcon;

// 编译时递归包含 assets 文件夹下所有内容
static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

// 通用函数：将资源文件转为 (RGBA字节, 宽度, 高度)
fn load_rgba_from_assets(path: &str, size: u32) -> (Vec<u8>, u32, u32) {
    let file = ASSETS_DIR
        .get_file(path)
        .expect(&format!("在 assets 中找不到文件: {}", path));

    let mut img =
        image::load_from_memory(file.contents()).expect(&format!("无法解码图片数据: {}", path));

    if size > 0 && (img.width() != size || img.height() != size) {
        img = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
    }

    let (width, height) = img.dimensions();
    let rgba = img.to_rgba8().into_raw();

    (rgba, width, height)
}

// 获取 Iced 窗口图标
pub fn get_iced_icon(path: &str) -> iced_icon::Icon {
    let (rgba, width, height) = load_rgba_from_assets(path, 0);
    iced_icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败")
}

// 获取 Tray-icon 托盘图标
pub fn get_tray_icon(path: &str) -> TrayIcon {
    let (rgba, width, height) = load_rgba_from_assets(path, 32);
    TrayIcon::from_rgba(rgba, width, height).expect("生成 Tray 图标失败")
}
