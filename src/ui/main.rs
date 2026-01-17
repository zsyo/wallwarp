use super::{ActivePage, App, AppMessage};
use crate::ui::style::{
    APP_NAME_SIZE, BORDER_RADIUS, BORDER_WIDTH, BUTTON_PADDING, BUTTON_SPACING, LAYOUT_PADDING, LOGO_DISPLAY_SIZE, LOGO_SIZE, LOGO_SPACING, OUTER_PADDING,
    ROW_SPACING_LARGE, SIDEBAR_PADDING, SIDEBAR_WIDTH,
};
use crate::utils::assets;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

pub fn view_internal(app: &App) -> Element<'_, AppMessage> {
    let functional_area_width = (app.current_window_width as f32 - SIDEBAR_WIDTH - ROW_SPACING_LARGE - OUTER_PADDING).max(1.0);

    let content: Element<'_, AppMessage> = match app.active_page {
        ActivePage::OnlineWallpapers => super::online::online_view(&app.i18n, functional_area_width as u32, &app.online_state, &app.config),
        ActivePage::LocalList => super::local::local_view(&app.i18n, &app.config, functional_area_width as u32, &app.local_state),
        ActivePage::DownloadProgress => super::download::download_view(&app.i18n, functional_area_width as u32, &app.download_state),
        ActivePage::Settings => super::settings::settings_view(app),
    };

    let (img, width, height) = assets::get_logo(LOGO_SIZE);
    let sidebar = container(
        column![
            text(app.i18n.t("app-name")).size(APP_NAME_SIZE).width(Length::Fill).align_x(Alignment::Center),
            iced::widget::image(iced::widget::image::Handle::from_rgba(width, height, img))
                .width(Length::Fixed(LOGO_DISPLAY_SIZE))
                .height(Length::Fixed(LOGO_DISPLAY_SIZE)),
            container(iced::widget::Space::new()).height(Length::Fixed(LOGO_SPACING)),
            create_menu_button(app.i18n.t("online-wallpapers.title"), app.active_page, ActivePage::OnlineWallpapers),
            create_menu_button(app.i18n.t("local-list.title"), app.active_page, ActivePage::LocalList),
            create_menu_button(app.i18n.t("download-tasks.title"), app.active_page, ActivePage::DownloadProgress),
            create_menu_button(app.i18n.t("settings"), app.active_page, ActivePage::Settings),
        ]
        .spacing(BUTTON_SPACING)
        .padding(SIDEBAR_PADDING)
        .align_x(Alignment::Center),
    )
    .width(Length::Fixed(SIDEBAR_WIDTH))
    .height(Length::Fill)
    .style(create_bordered_container_style);

    let main_content = container(content)
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .padding(0)
        .style(create_bordered_container_style);

    let layout = row![sidebar, main_content].spacing(ROW_SPACING_LARGE).width(Length::Fill).height(Length::Fill);

    container(layout).width(Length::Fill).height(Length::Fill).padding(LAYOUT_PADDING).into()
}

fn create_menu_button<'a>(label: String, current_page: ActivePage, target_page: ActivePage) -> button::Button<'a, AppMessage> {
    button(text(label).width(Length::Fill).align_x(Alignment::Center))
        .on_press_maybe(if current_page != target_page {
            Some(AppMessage::PageSelected(target_page))
        } else {
            None
        })
        .padding(BUTTON_PADDING)
}

fn create_bordered_container_style(theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        border: iced::border::Border {
            color: theme.extended_palette().primary.strong.color,
            width: BORDER_WIDTH,
            radius: iced::border::Radius::from(BORDER_RADIUS),
        },
        ..Default::default()
    }
}
