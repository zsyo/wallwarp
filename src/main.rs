#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Size, window};
use wallwarp::i18n::I18n;
use wallwarp::ui::App;
use wallwarp::utils::assets;
use wallwarp::utils::config::Config;

fn main() -> iced::Result {
    let i18n = I18n::new();

    // 首先加载配置
    let config = Config::new(&i18n.current_lang);

    let (rgba, width, height) = assets::get_logo(128);
    let icon = window::icon::from_rgba(rgba, width, height).expect("生成 Iced 图标失败");

    // 根据配置创建窗口设置
    let mut window_settings = window::Settings {
        size: Size::new(config.display.width as f32, config.display.height as f32),
        min_size: Some(Size::new(960.0, 640.0)),
        icon: Some(icon),
        exit_on_close_request: false, // 关键：不自动退出
        ..window::Settings::default()
    };

    // 如果配置中有窗口位置，则设置位置
    if let (Some(pos_x), Some(pos_y)) = (config.display.x, config.display.y) {
        window_settings.position = window::Position::Specific([pos_x as f32, pos_y as f32].into());
    }

    let init_data = std::cell::RefCell::new(Some((i18n, config)));

    iced::application(
        move || {
            // 通过 borrow_mut() 获取可变引用并 take() 出数据
            let (i18n, config) = init_data.borrow_mut().take().expect("App can only be initialized once");

            (App::new_with_config(i18n, config), iced::Task::none())
        },
        App::update,
        App::view,
    )
    .subscription(|app: &App| app.subscription())
    .window(window_settings)
    .title(|app: &App| app.title())
    .run()
}
