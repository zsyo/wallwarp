use super::AppMessage;
use super::{ActivePage, App};
use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, row, text},
};

use super::settings;

impl App {
    pub fn view_internal(&self) -> Element<'_, AppMessage> {
        // 左侧菜单
        let menu = column!()
            .push(
                container(text(self.i18n.t("app-name")).size(24))
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .push(container(iced::widget::Space::new()).height(Length::Fixed(150.0)))
            .push(
                column!(
                    button(
                        container(text(self.i18n.t("online-wallpapers")))
                            .width(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                    )
                    .on_press(AppMessage::PageSelected(ActivePage::OnlineWallpapers))
                    .width(Length::Fill)
                    .style(match self.active_page {
                        ActivePage::OnlineWallpapers => iced::widget::button::primary,
                        _ => iced::widget::button::secondary,
                    }),
                    button(
                        container(text(self.i18n.t("local-list")))
                            .width(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                    )
                    .on_press(AppMessage::PageSelected(ActivePage::LocalList))
                    .width(Length::Fill)
                    .style(match self.active_page {
                        ActivePage::LocalList => iced::widget::button::primary,
                        _ => iced::widget::button::secondary,
                    }),
                    button(
                        container(text(self.i18n.t("download-tasks")))
                            .width(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                    )
                    .on_press(AppMessage::PageSelected(ActivePage::DownloadProgress))
                    .width(Length::Fill)
                    .style(match self.active_page {
                        ActivePage::DownloadProgress => iced::widget::button::primary,
                        _ => iced::widget::button::secondary,
                    }),
                    button(
                        container(text(self.i18n.t("settings")))
                            .width(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                    )
                    .on_press(AppMessage::PageSelected(ActivePage::Settings))
                    .width(Length::Fill)
                    .style(match self.active_page {
                        ActivePage::Settings => iced::widget::button::primary,
                        _ => iced::widget::button::secondary,
                    })
                )
                .spacing(10),
            )
            .padding(10)
            .spacing(10);

        // 右侧内容区域
        let content = match self.active_page {
            ActivePage::OnlineWallpapers => {
                column!(text(self.i18n.t("online-wallpapers.title")).size(24))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding(20)
            }
            ActivePage::LocalList => column!(text(self.i18n.t("local-list.title")).size(24))
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .padding(20),
            ActivePage::DownloadProgress => {
                column!(text(self.i18n.t("download-tasks.title")).size(24))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .padding(20)
            }
            ActivePage::Settings => settings::settings_view(self).into(),
        };

        // 组合左右两部分，包含分隔线
        let layout = row!(
            container(menu).width(180),
            // 分隔线
            container(iced::widget::Space::new())
                .width(2)
                .height(Length::Fill)
                .style(|theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(
                        theme.extended_palette().primary.strong.color
                    )),
                    ..Default::default()
                }),
            container(content).width(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}
