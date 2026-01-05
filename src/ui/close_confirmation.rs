use iced::{
    Alignment, Length,
    widget::{button, column, container, row, text, toggler},
};

use super::{App, AppMessage, CloseConfirmationAction};

// 渲染关闭确认对话框（模态）
pub fn close_confirmation_view(app: &App) -> iced::Element<'_, AppMessage> {
    if !app.show_close_confirmation {
        return iced::widget::Space::new().into();
    }

    // 创建对话框内容
    let dialog_content = column![
        text(app.i18n.t("close-confirmation.title"))
            .size(16)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        text(app.i18n.t("close-confirmation.message"))
            .size(14)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        row![
            button(text(app.i18n.t("close-confirmation.minimize-to-tray")).size(14))
                .on_press(AppMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::MinimizeToTray,
                    app.remember_close_setting
                ))
                .style(|_theme: &iced::Theme, _status| {
                    let base = iced::widget::button::text(_theme, _status);
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            0, 123, 255,
                        ))), // 蓝色
                        text_color: iced::Color::WHITE,
                        ..base
                    }
                }),
            button(text(app.i18n.t("close-confirmation.exit")).size(14))
                .on_press(AppMessage::CloseConfirmationResponse(
                    CloseConfirmationAction::CloseApp,
                    app.remember_close_setting
                ))
                .style(|_theme: &iced::Theme, _status| {
                    let base = iced::widget::button::text(_theme, _status);
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            220, 53, 69,
                        ))), // 红色
                        text_color: iced::Color::WHITE,
                        ..base
                    }
                }),
            button(text(app.i18n.t("close-confirmation.cancel")).size(14))
                .on_press(AppMessage::CloseConfirmationCancelled)
                .style(|_theme: &iced::Theme, _status| {
                    let base = iced::widget::button::text(_theme, _status);
                    iced::widget::button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb8(
                            108, 117, 125,
                        ))), // 灰色
                        text_color: iced::Color::WHITE,
                        ..base
                    }
                }),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            toggler(app.remember_close_setting).on_toggle(AppMessage::ToggleRememberSetting),
            text(app.i18n.t("close-confirmation.remember-setting")).size(12),
        ]
        .align_y(Alignment::Center)
        .spacing(5)
    ]
    .padding(20)
    .spacing(15)
    .align_x(Alignment::Center);

    // 将对话框包装在容器中，设置样式（白色背景，边框）
    let modal_dialog = container(dialog_content)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .max_width(500) // 限制对话框最大宽度
        .padding(10)
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::border::Border {
                radius: iced::border::Radius::from(8.0),
                width: 1.0,
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
            },
            ..Default::default()
        });

    // 创建完整的模态内容：使用容器包含半透明背景和居中的对话框
    let modal_content = container(
        // 使用stack将遮罩层和居中对话框叠加
        iced::widget::stack(vec![
            // 半透明背景遮罩
            container(iced::widget::Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(iced::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5, // 半透明背景，实现模态效果
                    })),
                    ..Default::default()
                })
                .into(),
            // 居中的对话框
            container(modal_dialog)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        ])
    )
    .width(Length::Fill)
    .height(Length::Fill);

    // 返回使用opaque包装的模态内容
    iced::widget::opaque(modal_content)
        .into()
}
