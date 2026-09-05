// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 主窗口参数：启动与 Wayland 重建窗口共用，保证两次创建的参数一致

use crate::utils::assets;
use crate::utils::config::{self, Config};
use iced::{Size, window};

/// 主窗口图标尺寸
const MAIN_ICON_SIZE: u32 = 128;

/// 构建主窗口的 [`window::Settings`]
///
/// `visible` 控制创建时是否可见（`--hidden` 自启动为 false）
pub fn main_window_settings(config: &Config, visible: bool) -> window::Settings {
    let (rgba, width, height) = assets::get_logo(MAIN_ICON_SIZE);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    #[allow(unused_mut)] // Windows 平台无 platform_specific 定制
    let mut settings = window::Settings {
        // 位置记忆：仅记录过位置时恢复；最大化状态下保存的越界值不生效
        position: if config.display.x != i32::MIN && config.display.y != i32::MIN {
            window::Position::Specific(iced::Point::new(
                config.display.x as f32,
                config.display.y as f32,
            ))
        } else {
            window::Position::Centered
        },
        size: Size::new(config.display.width as f32, config.display.height as f32),
        min_size: Some(Size::new(
            config::MIN_WINDOW_WIDTH as f32,
            config::MIN_WINDOW_HEIGHT as f32,
        )),
        icon: Some(icon),
        exit_on_close_request: false,
        visible,
        // macOS：原生红绿灯叠加自绘标题栏；其余平台无边框
        decorations: cfg!(target_os = "macos"),
        ..window::Settings::default()
    };

    #[cfg(target_os = "macos")]
    {
        // macOS：原生红绿灯叠加在自绘标题栏上，原生边缘缩放/全屏可用
        settings.platform_specific.title_hidden = true;
        settings.platform_specific.titlebar_transparent = true;
        settings.platform_specific.fullsize_content_view = true;
    }
    #[cfg(target_os = "linux")]
    {
        // 与 .desktop 文件名保持一致，便于窗口管理器关联应用图标
        settings.platform_specific.application_id = "wallwarp".to_string();
    }

    settings
}
