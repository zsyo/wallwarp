// Copyright (C) 2026 zsyo - GNU AGPL v3.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Size, Task, font, window};
use tracing::info;
use wallwarp::i18n::I18n;
use wallwarp::ui::{App, AppMessage};
use wallwarp::utils::{assets, config::Config, logger, single_instance::SingleInstanceGuard};

const LOGO_SIZE: u32 = 128;
const MIN_WINDOW_WIDTH: f32 = 1280.0;
const MIN_WINDOW_HEIGHT: f32 = 800.0;

fn main() -> iced::Result {
    // 解析命令行参数，设置工作目录（用于开机自启动）
    let args: Vec<String> = std::env::args().collect();
    let start_hidden = args.iter().any(|arg| arg == "--hidden");

    // 如果 send_args 返回 true，说明已有实例，当前进程直接退出
    if SingleInstanceGuard::send_args_to_existing_instance(start_hidden) {
        println!("[启动] 检测到已有实例运行，已发送唤醒信号。");
        return Ok(());
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if !is_running_via_cargo() {
            // 生产模式：使用可执行文件所在目录作为工作目录
            if let Some(parent_dir) = exe_path.parent() {
                let _ = std::env::set_current_dir(parent_dir);
            }
        }
    }

    let i18n = I18n::new();
    let config = Config::new(&i18n.current_lang);
    let _log_guard = logger::init_logger(config.global.enable_logging);

    let (rgba, width, height) = assets::get_logo(LOGO_SIZE);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    let settings = window::Settings {
        position: window::Position::Centered,
        size: Size::new(config.display.width as f32, config.display.height as f32),
        min_size: Some(Size::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)),
        icon: Some(icon),
        exit_on_close_request: false,
        visible: !start_hidden, // 如果是隐藏模式，初始不显示窗口
        decorations: false,     // 隐藏默认标题栏，使用自定义标题栏
        ..window::Settings::default()
    };

    let system_ui_font = get_system_ui_font();
    info!("系统 UI 字体: {}", system_ui_font);

    let init_data = std::cell::RefCell::new(Some((i18n, config)));
    iced::application(
        move || {
            let (i18n, config) = init_data.borrow_mut().take().expect("App can only be initialized once");
            let app = App::new_with_config(i18n, config);
            let load_font_task = font::load(assets::ICON_FONT).discard();
            let enable_resize_task = app.enable_window_resize();
            let listen_task = Task::perform(SingleInstanceGuard::listen(), AppMessage::ExternalInstanceTriggered);
            (app, Task::batch(vec![load_font_task, enable_resize_task, listen_task]))
        },
        App::update,
        App::view,
    )
    .subscription(|app: &App| app.subscription())
    .window(settings)
    .title(|app: &App| app.title())
    .default_font(iced::Font {
        family: font::Family::Name(system_ui_font),
        ..iced::Font::DEFAULT
    })
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .run()
}

/// 根据平台返回最通用的系统 UI 字体名称
fn get_system_ui_font() -> &'static str {
    if cfg!(target_os = "windows") {
        "Microsoft YaHei" // 微软雅黑
    } else if cfg!(target_os = "macos") {
        "Helvetica Neue" // 或使用 ".AppleSystemUIFont"
    } else {
        "Noto Sans CJK SC" // Linux 常用中文字体
    }
}

/// 检测运行环境
fn is_running_via_cargo() -> bool {
    // 只要是 cargo 启动的，这个环境变量一定存在
    std::env::var("CARGO").is_ok()
}
