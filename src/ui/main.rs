use super::{ActivePage, App, AppMessage};
use crate::utils::assets;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

pub fn view_internal(app: &App) -> Element<'_, AppMessage> {
    let content: Element<'_, AppMessage> = match app.active_page {
        ActivePage::OnlineWallpapers => {
            // TODO: 实现在线壁纸页面
            column![text("在线壁纸页面").size(24)].into()
        }
        ActivePage::LocalList => {
            // 使用local模块实现的本地壁纸列表页面，传递当前窗口宽度以实现响应式布局
            super::local::local_view(
                &app.i18n,
                &app.config,
                app.current_window_width,
                &app.local_state,
            )
        }
        ActivePage::DownloadProgress => {
            // TODO: 实现下载进度页面
            column![text("下载进度页面").size(24)].into()
        }
        ActivePage::Settings => super::settings::settings_view(app),
    };

    let (img, width, height) = assets::get_logo(128);
    let sidebar = container(
        column![
            text(app.i18n.t("app-name"))
                .size(24)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                .width(Length::Fixed(128.0))
                .height(Length::Fixed(128.0)),
            container(iced::widget::Space::new()).height(Length::Fixed(20.0)),
            button(
                text(app.i18n.t("online-wallpapers.title"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press_maybe(if app.active_page != ActivePage::OnlineWallpapers {
                Some(AppMessage::PageSelected(ActivePage::OnlineWallpapers))
            } else {
                None
            })
            .padding(10),
            button(
                text(app.i18n.t("local-list.title"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press_maybe(if app.active_page != ActivePage::LocalList {
                Some(AppMessage::PageSelected(ActivePage::LocalList))
            } else {
                None
            })
            .padding(10),
            button(
                text(app.i18n.t("download-tasks.title"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press_maybe(if app.active_page != ActivePage::DownloadProgress {
                Some(AppMessage::PageSelected(ActivePage::DownloadProgress))
            } else {
                None
            })
            .padding(10),
            button(
                text(app.i18n.t("settings"))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
            )
            .on_press_maybe(if app.active_page != ActivePage::Settings {
                Some(AppMessage::PageSelected(ActivePage::Settings))
            } else {
                None
            })
            .padding(10),
        ]
        .spacing(5)
        .padding(10)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(180.0))
    .height(Length::Fill)
    .style(|theme: &iced::Theme| iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: 1.0,
            radius: iced::border::Radius::from(5.0),
        },
        ..Default::default()
    });

    let main_content = container(content)
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .padding(20)
        .style(|theme: &iced::Theme| iced::widget::container::Style {
            border: iced::border::Border {
                color: theme.extended_palette().primary.strong.color,
                width: 1.0,
                radius: iced::border::Radius::from(5.0),
            },
            ..Default::default()
        });

    let layout = row![sidebar, main_content]
        .spacing(20)
        .width(Length::Fill)
        .height(Length::Fill);

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}
