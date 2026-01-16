// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Size, font, window};
use wallwarp::i18n::I18n;
use wallwarp::ui::App;
use wallwarp::utils::assets;
use wallwarp::utils::config::Config;
use wallwarp::utils::logger;

const LOGO_SIZE: u32 = 128;
const MIN_WINDOW_WIDTH: f32 = 1280.0;
const MIN_WINDOW_HEIGHT: f32 = 800.0;

fn main() -> iced::Result {
    // 初始化日志系统
    logger::init_logger();

    let i18n = I18n::new();
    let config = Config::new(&i18n.current_lang);

    let (rgba, width, height) = assets::get_logo(LOGO_SIZE);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    let settings = window::Settings {
        position: window::Position::Centered,
        size: Size::new(config.display.width as f32, config.display.height as f32),
        min_size: Some(Size::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)),
        icon: Some(icon),
        exit_on_close_request: false,
        ..window::Settings::default()
    };

    let init_data = std::cell::RefCell::new(Some((i18n, config)));
    iced::application(
        move || {
            let (i18n, config) = init_data.borrow_mut().take().expect("App can only be initialized once");
            let app = App::new_with_config(i18n, config);
            let load_font_task = font::load(assets::ICON_FONT).discard();
            let initial_tasks = app.get_initial_tasks();
            (app, load_font_task.chain(initial_tasks))
        },
        App::update,
        App::view,
    )
    .subscription(|app: &App| app.subscription())
    .window(settings)
    .title(|app: &App| app.title())
    .run()
}
