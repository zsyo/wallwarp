use super::{App, AppMessage, ActivePage};
use iced::widget::{column, container, row, text, button};
use iced::{Alignment, Element, Length};

pub fn view_internal(app: &App) -> Element<'_, AppMessage> {
    let content = match app.active_page {
        ActivePage::OnlineWallpapers => {
            // TODO: 实现在线壁纸页面
            column![text("在线壁纸页面").size(24)]
        }
        ActivePage::LocalList => {
            // TODO: 实现本地壁纸列表页面
            column![text("本地壁纸列表页面").size(24)]
        }
        ActivePage::DownloadProgress => {
            // TODO: 实现下载进度页面
            column![text("下载进度页面").size(24)]
        }
        ActivePage::Settings => {
            super::settings::settings_view(app)
        }
    };

    let sidebar = container(
        column![
            text(app.i18n.t("app-title")).size(24).width(Length::Fill).align_x(Alignment::Center),
            button(text(app.i18n.t("online-wallpapers.title")).width(Length::Fill).align_x(Alignment::Center))
                .on_press(AppMessage::PageSelected(ActivePage::OnlineWallpapers))
                .padding(10),
            button(text(app.i18n.t("local-list.title")).width(Length::Fill).align_x(Alignment::Center))
                .on_press(AppMessage::PageSelected(ActivePage::LocalList))
                .padding(10),
            button(text(app.i18n.t("download-tasks.title")).width(Length::Fill).align_x(Alignment::Center))
                .on_press(AppMessage::PageSelected(ActivePage::DownloadProgress))
                .padding(10),
            button(text(app.i18n.t("settings")).width(Length::Fill).align_x(Alignment::Center))
                .on_press(AppMessage::PageSelected(ActivePage::Settings))
                .padding(10),
        ]
        .spacing(5)
        .padding(10)
        .align_x(Alignment::Start),
    )
    .width(Length::FillPortion(1))
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
        .padding(20);

    let layout = row![sidebar, main_content].width(Length::Fill).height(Length::Fill);

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}
