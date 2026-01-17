use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::common;
use crate::ui::AppMessage;
use crate::ui::style::*;
use crate::i18n::I18n;
use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

/// 创建图片预览模态窗口
pub fn create_modal<'a>(
    i18n: &'a I18n,
    online_state: &'a OnlineState,
) -> Element<'a, AppMessage> {
    let wallpaper_index = online_state.current_image_index;

    // 创建背景加载文字
    let loading_text = create_modal_loading_placeholder(i18n);

    // 创建图片层（加载完成后显示）
    let image_layer: Element<_> = if let Some(ref handle) = online_state.modal_image_handle {
        let modal_image = iced::widget::image(handle.clone())
            .content_fit(iced::ContentFit::Contain)
            .width(Length::Fill)
            .height(Length::Fill);
        modal_image.into()
    } else {
        container(iced::widget::Space::new()).width(Length::Fill).height(Length::Fill).into()
    };

    let modal_image_content = iced::widget::stack(vec![loading_text, image_layer]);

    // 创建底部工具栏按钮
    let prev_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F12E}", BUTTON_COLOR_BLUE, AppMessage::Online(OnlineMessage::PreviousImage)),
        i18n.t("online-wallpapers.tooltip-prev"),
    );

    let next_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F137}", BUTTON_COLOR_BLUE, AppMessage::Online(OnlineMessage::NextImage)),
        i18n.t("online-wallpapers.tooltip-next"),
    );

    let download_button = common::create_button_with_tooltip(
        common::create_icon_button(
            "\u{F30A}",
            BUTTON_COLOR_GREEN,
            AppMessage::Online(OnlineMessage::DownloadWallpaper(wallpaper_index)),
        ),
        i18n.t("online-wallpapers.tooltip-download"),
    );

    let close_button = common::create_button_with_tooltip(
        common::create_icon_button("\u{F659}", BUTTON_COLOR_RED, AppMessage::Online(OnlineMessage::CloseModal)),
        i18n.t("online-wallpapers.tooltip-close"),
    );

    // 底部工具栏
    let toolbar = container(
        row![
            container(iced::widget::Space::new()).width(Length::Fill),
            prev_button,
            next_button,
            download_button,
            close_button,
            container(iced::widget::Space::new()).width(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(50.0),
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(iced::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.7,
        })),
        ..Default::default()
    });

    let modal_content = container(
        column![container(modal_image_content).width(Length::Fill).height(Length::Fill).padding(20), toolbar,]
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(COLOR_MODAL_BG)),
        ..Default::default()
    });

    container(iced::widget::opaque(modal_content)).into()
}

/// 创建模态窗口加载占位符
fn create_modal_loading_placeholder<'a>(i18n: &'a I18n) -> Element<'a, AppMessage> {
    let loading_text = text(i18n.t("online-wallpapers.image-loading"))
        .size(24)
        .style(|_theme: &iced::Theme| text::Style {
            color: Some(COLOR_OVERLAY_TEXT),
        });

    container(loading_text)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}