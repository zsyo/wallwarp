use iced::{Size, window};
use wallwarp::ui::App;
use wallwarp::ui::message::AppMessage;

fn main() -> iced::Result {
    iced::application(|| App::new(), app_update, app_view)
        .window(window::Settings {
            size: Size::new(960.0, 640.0),
            min_size: Some(Size::new(960.0, 640.0)),
            ..window::Settings::default()
        })
        .title(|app: &App| app.title())
        .run()
}

fn app_update(app: &mut App, message: AppMessage) {
    app.update(message);
}

fn app_view(app: &App) -> iced::Element<'_, AppMessage> {
    app.view()
}
