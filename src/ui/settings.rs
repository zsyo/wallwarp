use super::message::AppMessage;
use super::App;
use iced::{Alignment, Length, widget::{column, pick_list, text}};

/// 渲染设置页面的UI组件
pub fn settings_view(app: &App) -> iced::widget::Column<'_, AppMessage> {
    column!(
        text(app.i18n.t("settings.title")).size(24),
        pick_list(
            &app.i18n.available_langs[..],
            Some(app.i18n.current_lang.clone()),
            AppMessage::LanguageSelected
        )
    )
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .padding(20)
}