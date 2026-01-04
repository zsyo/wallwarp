use super::App;
use super::AppMessage;
use crate::utils::config::CloseAction;
use iced::{
    Alignment, Length,
    widget::{column, container, pick_list, text, toggler},
};

/// 渲染设置页面的UI组件
pub fn settings_view(app: &App) -> iced::widget::Column<'_, AppMessage> {
    let system_config_section = container(
        column!(
            text(app.i18n.t("settings.system-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::row!(
                text(app.i18n.t("settings.app-language")).width(Length::FillPortion(1)),
                pick_list(
                    &app.i18n.available_langs[..],
                    Some(app.i18n.current_lang.clone()),
                    AppMessage::LanguageSelected
                )
                .width(Length::Fixed(80.0))
            )
            .height(Length::Fixed(20.0))
            .width(Length::Fill)
            .spacing(10),
            iced::widget::row!(
                text(app.i18n.t("settings.auto-startup")).width(Length::FillPortion(1)),
                toggler(app.config.global.auto_startup).on_toggle(AppMessage::AutoStartupToggled)
            )
            .height(Length::Fixed(20.0))
            .width(Length::Fill)
            .spacing(10),
            iced::widget::row!(
                text(app.i18n.t("settings.close-action")).width(Length::FillPortion(1)),
                iced::widget::row![
                    iced::widget::radio(
                        app.i18n.t("close-action-options.ask"),
                        CloseAction::Ask,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    ),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.minimize-to-tray"),
                        CloseAction::MinimizeToTray,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    ),
                    iced::widget::radio(
                        app.i18n.t("close-action-options.close-app"),
                        CloseAction::CloseApp,
                        Some(app.config.global.close_action.clone()),
                        AppMessage::CloseActionSelected
                    )
                ]
                .spacing(10)
            )
            .width(Length::Fill)
            .spacing(10)
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    let temp_config_section = container(
        column!(
            text(app.i18n.t("settings.temp-config"))
                .size(16)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::row!(
                text(app.i18n.t("settings.temp-option")).width(Length::FillPortion(1)),
                text(app.i18n.t("settings.temp-value")).width(Length::FillPortion(1))
            )
            .width(Length::Fill)
            .spacing(10)
        )
        .padding(15)
        .spacing(10),
    )
    .width(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    column!(system_config_section, temp_config_section)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .padding(20)
        .spacing(10)
}
