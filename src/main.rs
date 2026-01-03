use iced::{Size, window};
use wallwarp::i18n::I18n;
use wallwarp::ui::App;
use wallwarp::ui::message::AppMessage;
use wallwarp::utils::config::Config;

fn main() -> iced::Result {
    let i18n = I18n::new();

    // 首先加载配置
    let config = Config::new(&i18n.current_lang);

    // 根据配置创建窗口设置
    let mut window_settings = window::Settings {
        size: Size::new(config.window_width as f32, config.window_height as f32),
        min_size: Some(Size::new(960.0, 640.0)),
        ..window::Settings::default()
    };

    // 如果配置中有窗口位置，则设置位置
    if let (Some(pos_x), Some(pos_y)) = (config.window_pos_x, config.window_pos_y) {
        window_settings.position = window::Position::Specific([pos_x as f32, pos_y as f32].into());
    }

    let init_data = std::cell::RefCell::new(Some((i18n, config)));

    iced::application(
        move || {
            // 通过 borrow_mut() 获取可变引用并 take() 出数据
            let (i18n, config) = init_data
                .borrow_mut()
                .take()
                .expect("App can only be initialized once");

            (App::new_with_config(i18n, config), iced::Task::none())
        },
        app_update,
        app_view,
    )
    .subscription(|app: &App| app.subscription())
    .window(window_settings)
    .title(|app: &App| app.title())
    .run()
}

fn app_update(app: &mut App, message: AppMessage) {
    app.update(message);
}

fn app_view(app: &App) -> iced::Element<'_, AppMessage> {
    app.view()
}
