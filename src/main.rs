// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Size, font, window};
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
    let settings = window::Settings {
        position: window::Position::Centered,
        size: Size::new(config.display.width as f32, config.display.height as f32),
        min_size: Some(Size::new(1280.0, 800.0)),
        icon: Some(icon),
        exit_on_close_request: false, // 关键：不自动退出
        ..window::Settings::default()
    };

    let init_data = std::cell::RefCell::new(Some((i18n, config)));
    iced::application(
        move || {
            // 通过 borrow_mut() 获取可变引用并 take() 出数据
            let (i18n, config) = init_data.borrow_mut().take().expect("App can only be initialized once");

            // 加载图标字体
            let load_font_task = font::load(assets::ICON_FONT).discard();

            (App::new_with_config(i18n, config), load_font_task)
        },
        App::update,
        App::view,
    )
    .subscription(|app: &App| app.subscription())
    .window(settings)
    .title(|app: &App| app.title())
    .run()
}
