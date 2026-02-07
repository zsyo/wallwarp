// Copyright (C) 2026 zsyo - GNU AGPL v3.0

// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Size, Task, font, window};
use tracing::{error, info};
use wallwarp::i18n::I18n;
use wallwarp::services::async_task::async_cleanup_cache;
use wallwarp::ui::main::MainMessage;
use wallwarp::ui::{App, AppMessage};
use wallwarp::utils::{assets, config, helpers, logger, single_instance::SingleInstanceGuard};

const LOGO_SIZE: u32 = 128;

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
        if !helpers::is_running_via_cargo() {
            // 生产模式：使用可执行文件所在目录作为工作目录
            if let Some(parent_dir) = exe_path.parent() {
                let _ = std::env::set_current_dir(parent_dir);
            }
        }
    }

    let i18n = I18n::new();
    let cfg = config::Config::new(&i18n.current_lang, &i18n.available_langs);
    let _log_guard = logger::init_logger(cfg.global.enable_logging);

    let (rgba, width, height) = assets::get_logo(LOGO_SIZE);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    let settings = window::Settings {
        position: window::Position::Centered,
        size: Size::new(cfg.display.width as f32, cfg.display.height as f32),
        min_size: Some(Size::new(
            config::MIN_WINDOW_WIDTH as f32,
            config::MIN_WINDOW_HEIGHT as f32,
        )),
        icon: Some(icon),
        exit_on_close_request: false,
        visible: !start_hidden, // 如果是隐藏模式，初始不显示窗口
        decorations: false,     // 隐藏默认标题栏，使用自定义标题栏
        ..window::Settings::default()
    };

    let system_ui_font = helpers::get_system_ui_font();
    info!("系统 UI 字体: {}", system_ui_font);

    let init_data = std::cell::RefCell::new(Some((i18n, cfg)));
    iced::application(
        move || {
            let (i18n, cfg) = init_data.borrow_mut().take().expect("App can only be initialized once");

            // 在 cfg 被移动之前先克隆一份用于清理任务
            let cleanup_config = cfg.clone();

            let app = App::new_with_config(i18n, cfg);

            // 创建启动任务
            let load_font_task = font::load(assets::ICON_FONT).discard();
            let enable_resize_task = app.enable_window_drag_resize();
            let listen_task = Task::perform(SingleInstanceGuard::listen(), |payload| {
                MainMessage::ExternalInstanceTriggered(payload).into()
            });

            // 创建缓存清理任务（在后台异步执行）
            let cleanup_task = Task::perform(
                async move {
                    // 延迟 2 秒后执行清理，避免影响启动性能
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    match async_cleanup_cache(cleanup_config).await {
                        Ok(_) => {
                            info!("[启动] 缓存清理任务完成");
                        }
                        Err(e) => {
                            error!("[启动] 缓存清理任务失败: {}", e);
                        }
                    }
                    AppMessage::None // 返回一个空消息
                },
                |msg| msg.into(),
            );

            (
                app,
                Task::batch(vec![load_font_task, enable_resize_task, listen_task, cleanup_task]),
            )
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
