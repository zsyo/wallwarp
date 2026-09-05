// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::local;
use crate::ui::main::MainMessage;
use crate::ui::{ActivePage, App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn page_selected(&mut self, page: ActivePage) -> Task<AppMessage> {
        // 当切换离开在线壁纸页面时，取消正在进行的请求
        if self.active_page == ActivePage::OnlineWallpapers && page != ActivePage::OnlineWallpapers
        {
            self.online_state.cancel_and_new_context();
        }
        self.active_page = page;

        match page {
            ActivePage::OnlineWallpapers => {
                // 滚动到顶部
                Task::done(MainMessage::ScrollToTop("online_wallpapers_scroll".to_string()).into())
            }
            ActivePage::LocalList => {
                // 本地列表状态跨页保留，避免每次进入都全量重扫+重建缩略图；
                // 仅当数据路径变化或尚未加载过时重新加载
                let need_reload = self.local_state.loaded_data_path.as_deref()
                    != Some(self.config.data.data_path.as_str());
                let reload_task = if need_reload {
                    Task::done(local::LocalMessage::LoadWallpapers.into())
                } else {
                    Task::none()
                };
                Task::batch(vec![
                    reload_task,
                    Task::done(
                        MainMessage::ScrollToTop("local_wallpapers_scroll".to_string()).into(),
                    ),
                ])
            }
            ActivePage::Settings => {
                // 当切换到设置页面时，重置设置相关的临时状态
                let (proxy_protocol, proxy_address, proxy_port) =
                    crate::ui::settings::SettingsState::parse_proxy_string(
                        &self.config.global.proxy,
                    );
                self.settings_state.proxy_protocol = proxy_protocol;
                self.settings_state.proxy_address = proxy_address;
                self.settings_state.proxy_port = proxy_port;
                if proxy_port == 0 {
                    self.settings_state.proxy_port = 1080;
                }

                // 重置API KEY设置状态
                self.settings_state.wallhaven_api_key = self.config.wallhaven.api_key.clone();

                // 重置壁纸模式状态
                self.settings_state.wallpaper_mode = self.config.wallpaper.mode;

                // 重置定时切换模式状态
                self.settings_state.auto_change_mode = self.config.wallpaper.auto_change_mode;

                // 重置定时切换周期状态
                self.settings_state.auto_change_interval =
                    self.config.wallpaper.auto_change_interval;

                // 重置自定义分钟数状态
                self.settings_state.custom_interval_minutes = self
                    .config
                    .wallpaper
                    .auto_change_interval
                    .get_minutes()
                    .unwrap_or(30);

                // 重置定时切换关键词状态
                self.settings_state.auto_change_query =
                    self.config.wallpaper.auto_change_query.clone();

                // 滚动到顶部
                Task::done(MainMessage::ScrollToTop("settings_scroll".to_string()).into())
            }
            ActivePage::WallpaperHistory => {
                // 会话内首次进入时从数据库加载历史
                let load_task = if !self.history_state.loaded {
                    Task::done(crate::ui::history::HistoryMessage::Load.into())
                } else {
                    Task::none()
                };
                Task::batch(vec![
                    load_task,
                    Task::done(MainMessage::ScrollToTop("history_scroll".to_string()).into()),
                ])
            }
            _ => Task::none(),
        }
    }
}
