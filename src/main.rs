// Copyright (C) 2026 zsyo - GNU AGPL v3.0

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Task, font, theme, window};
use tracing::{error, info};
use wallwarp::i18n::I18n;
use wallwarp::services::async_task::async_cleanup_cache;
use wallwarp::ui::main::{MainMessage, main_window_settings, window_settings};
use wallwarp::ui::{App, AppMessage};
use wallwarp::utils::{assets, config, helpers, logger, single_instance::SingleInstanceGuard};

fn main() -> iced::Result {
    // 解析命令行参数，设置工作目录（用于开机自启动）
    let args: Vec<String> = std::env::args().collect();
    let start_hidden = args.iter().any(|arg| arg == "--hidden");

    // 如果 send_args 返回 true，说明已有实例，当前进程直接退出
    if SingleInstanceGuard::send_args_to_existing_instance(start_hidden) {
        println!("[启动] 检测到已有实例运行，已发送唤醒信号。");
        return Ok(());
    }

    if !helpers::is_running_via_cargo() {
        // 生产模式：切换工作目录到应用数据根目录
        // （Windows 便携式：exe 同级；macOS/Linux：平台标准数据目录，
        //   此后 config.toml/data/cache/db/logs 等相对路径全部落在根目录内）
        let root = helpers::app_root_dir();
        if let Err(e) = std::fs::create_dir_all(&root) {
            eprintln!("[启动] 创建数据目录失败 {}: {}", root.display(), e);
        }
        let _ = std::env::set_current_dir(&root);
    }

    let i18n = I18n::new();
    let cfg = config::Config::new(&i18n.current_lang, &i18n.lang_codes());
    let _log_guard = logger::init_logger(cfg.global.enable_logging);

    let system_ui_font = helpers::get_system_ui_font();
    info!("系统 UI 字体: {}", system_ui_font);

    let init_data = std::cell::RefCell::new(Some((i18n, cfg)));
    iced::daemon(
        move || {
            let (i18n, cfg) = init_data
                .borrow_mut()
                .take()
                .expect("App can only be initialized once");

            // 在 cfg 被移动之前先克隆一份用于清理任务
            let cleanup_config = cfg.clone();

            let mut app = App::new_with_config(i18n, cfg);

            // daemon 默认不开窗：主窗口在此显式打开并记录 Id
            let (main_id, open_main_task) =
                window::open(main_window_settings(&app.config, !start_hidden));
            app.main_window_id = main_id;

            let mut tasks: Vec<Task<AppMessage>> = vec![open_main_task.map(|_| AppMessage::None)];

            // 按配置打开悬浮球窗口（Wayland 会话不支持窗口定位/置顶，自动跳过）
            if app.config.global.show_floating_ball && wallwarp::platform::supports_floating_ball()
            {
                let (ball_id, open_ball_task) = window::open(window_settings(&app.config.global));
                app.floating_ball_id = Some(ball_id);
                tasks.push(open_ball_task.map(|_| AppMessage::None));
                info!("[启动] [悬浮球] 已按配置打开: {:?}", ball_id);
            }

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
                |msg| msg,
            );

            tasks.extend(vec![
                load_font_task,
                enable_resize_task,
                listen_task,
                cleanup_task,
            ]);

            (app, Task::batch(tasks))
        },
        App::update,
        App::view,
    )
    .subscription(|app: &App| app.subscription())
    .title(|app: &App, _id: window::Id| app.title())
    // 悬浮球窗口使用“背景透明”的专用主题（style 闭包中识别），实现圆形透出
    .theme(|app: &App, id: window::Id| {
        if Some(id) == app.floating_ball_id {
            let mut palette = iced::theme::Theme::Dark.palette();
            palette.background = iced::Color::TRANSPARENT;
            iced::theme::Theme::custom("wallwarp-floating-ball".to_string(), palette)
        } else {
            // 主窗口：跟随应用自身主题（深色/浅色）
            match app.theme_config.get_theme() {
                wallwarp::ui::style::Theme::Dark => iced::theme::Theme::Dark,
                _ => iced::theme::Theme::Light,
            }
        }
    })
    .style(|_app: &App, theme: &iced::Theme| {
        let mut style = theme::Base::base(theme);
        if theme.palette().background == iced::Color::TRANSPARENT {
            style.background_color = iced::Color::TRANSPARENT;
        }
        style
    })
    .default_font(iced::Font {
        family: font::Family::Name(system_ui_font),
        ..iced::Font::DEFAULT
    })
    .font(iced_aw::ICED_AW_FONT_BYTES)
    .run()
}
