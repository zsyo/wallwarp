// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 壁纸组快速切换（在线页筛选栏）

use crate::services::source::SourceKind;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::config::WallpaperMode;
use iced::Task;
use tracing::info;

impl App {
    /// 展开壁纸组选择器
    pub(in crate::ui::online) fn online_group_picker_expanded(&mut self) -> Task<AppMessage> {
        self.online_state.group_picker_expanded = !self.online_state.group_picker_expanded;
        Task::none()
    }

    /// 收起壁纸组选择器
    pub(in crate::ui::online) fn online_group_picker_dismiss(&mut self) -> Task<AppMessage> {
        self.online_state.group_picker_expanded = false;
        Task::none()
    }

    /// 应用壁纸组：写回图源/筛选快照/铺满模式，刷新在线页与配置
    pub(in crate::ui::online) fn apply_wallpaper_group(
        &mut self,
        group_id: i64,
    ) -> Task<AppMessage> {
        let Some(group) = self.wallpaper_groups.iter().find(|g| g.id == group_id) else {
            return Task::none();
        };
        let group = group.clone();
        let filters = group.parse_filters();

        info!("[壁纸组] [{}] 应用（图源: {}）", group.name, group.source);

        // 写回配置（筛选快照 → config.wallhaven；铺满模式 → config.wallpaper）
        self.config.global.online_source = group.source.clone();
        self.config.wallpaper.mode = WallpaperMode::parse(&group.mode);
        self.config.wallhaven.category = filters.category;
        self.config.wallhaven.purity = filters.purity;
        self.config.wallhaven.sorting = filters.sorting;
        self.config.wallhaven.color = filters.color;
        self.config.wallhaven.source_color = filters.source_color;
        self.config.wallhaven.pixabay_category = filters.pixabay_category;
        self.config.wallhaven.editors_choice = filters.editors_choice;
        self.config.wallhaven.top_range = filters.top_range;
        self.config.wallhaven.resolution_mode = filters.resolution_mode;
        self.config.wallhaven.atleast_resolution = filters.atleast_resolution;
        self.config.wallhaven.resolutions = filters.resolutions;
        self.config.wallhaven.ratios = filters.ratios;

        // 刷新在线页状态（source 与筛选字段），下次触发加载
        self.online_state.source = SourceKind::parse(&group.source);
        self.online_state.sync_filters_from_config(&self.config);
        self.online_state.has_loaded = false;
        self.online_state.current_page = 1;
        self.online_state.group_picker_expanded = false;

        // 磁盘写入经防抖合并，随后按新筛选重新加载列表
        Task::batch([
            self.request_config_save(),
            self.load_online_wallpapers(),
            self.show_notification(
                self.i18n
                    .t_with_args("online-wallpapers.group-applied", &[("name", group.name)]),
                NotificationType::Success,
            ),
        ])
    }
}
