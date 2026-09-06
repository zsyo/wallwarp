// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::i18n::I18n;
use crate::ui::AppMessage;
use crate::ui::common;
use crate::ui::common::preview_toolbar_button;
use crate::ui::online::{OnlineMessage, OnlineState};
use crate::ui::style::{BUTTON_COLOR_BLUE, ThemeConfig};
use iced::Element;
use iced::widget::{container, opaque};

/// 创建图片预览模态窗口（复用公共预览模态，注入在线页特有内容）
pub fn create_modal<'a>(
    i18n: &'a I18n,
    online_state: &'a OnlineState,
    theme_config: &'a ThemeConfig,
) -> Element<'a, AppMessage> {
    let wallpaper_index = online_state.current_image_index;
    let theme_colors = theme_config.get_theme_colors();
    // 图片加载完成后"设为壁纸/保存到库"才可点击
    let has_image = online_state.modal_image_handle.is_some();

    // 加载占位层：带进度环（替代默认占位文案）
    let loading_layer = super::create_modal_loading_placeholder(i18n, online_state, theme_config);

    // 信息浮层：分辨率/纯净度/收藏数/复制原图链接
    let info_layer = online_state
        .wallpapers_data
        .get(wallpaper_index)
        .map(|wallpaper| super::create_modal_info(i18n, wallpaper, wallpaper_index, theme_config));

    // 保存到库按钮：按钮实际作用是将缓存的图片保存到壁纸库，
    // 提示与卡片上的下载按钮区分
    let download_button = preview_toolbar_button(
        theme_config,
        theme_colors.disabled_color,
        "\u{F30A}", // download
        BUTTON_COLOR_BLUE,
        i18n.t("online-wallpapers.tooltip-save-to-library")
            .to_string(),
        has_image.then_some(OnlineMessage::DownloadFromCache(wallpaper_index).into()),
    );

    let modal_content = common::create_preview_modal(
        i18n,
        theme_config,
        online_state.modal_image_handle.as_ref(),
        None,
        true, // 上一张始终可点（在线页支持循环浏览）
        true, // 下一张始终可点
        common::PreviewModalMessages {
            previous: OnlineMessage::PreviousImage.into(),
            next: OnlineMessage::NextImage.into(),
            set_wallpaper: has_image
                .then_some(OnlineMessage::SetAsWallpaperFromCache(wallpaper_index).into()),
            view_in_folder: None, // 在线页无"打开所在文件夹"操作
            close: OnlineMessage::CloseModal.into(),
        },
        common::PreviewModalTexts {
            loading: String::new(), // 加载层由 extras 注入，文案不生效
            previous: i18n.t("online-wallpapers.tooltip-prev"),
            next: i18n.t("online-wallpapers.tooltip-next"),
            set_wallpaper: i18n.t("online-wallpapers.tooltip-set-wallpaper"),
            view_in_folder: String::new(), // 按钮未显示，文案不生效
            close: i18n.t("online-wallpapers.tooltip-close"),
        },
        common::PreviewModalExtras {
            loading_layer: Some(loading_layer),
            info_layer,
            toolbar_buttons: vec![download_button],
        },
    );

    container(opaque(modal_content)).into()
}
