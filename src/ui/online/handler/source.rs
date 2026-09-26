// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 在线图源切换（在线页顶栏图源选择器）

use crate::services::source::SourceKind;
use crate::services::wallhaven;
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    /// 切换在线图源（状态持久化到配置）
    ///
    /// 在线页标记为未加载，重新进入时按新图源拉取列表
    pub(in crate::ui::online) fn online_source_selected(
        &mut self,
        kind: SourceKind,
    ) -> Task<AppMessage> {
        let old = self.online_state.source;
        info!("[在线壁纸] [图源] 切换: {} -> {}", old.id(), kind.id());
        self.online_state.source = kind;
        self.config.global.online_source = kind.id().to_string();
        // 切到未配置 API Key 的 Wallhaven 时不携带 NSFW 位（与
        // sync_filters 的加载期收紧一致；Danbooru 免密钥，分级由
        // rating metatag 自行映射，不受此限制）
        if kind == SourceKind::Wallhaven && self.config.wallhaven.api_key.is_empty() {
            self.online_state.purities &= !wallhaven::Purity::NSFW.bit_value();
        }
        // 图源已变化，标记未加载并按新图源立即重新拉取列表
        self.online_state.has_loaded = false;
        self.online_state.current_page = 1;
        // 选择后关闭选择器，磁盘写入经防抖合并
        self.online_state.source_picker_expanded = false;
        Task::batch([self.request_config_save(), self.load_online_wallpapers()])
    }

    pub(in crate::ui::online) fn online_source_picker_expanded(&mut self) -> Task<AppMessage> {
        // 切换图源选择器的展开/收起状态
        self.online_state.source_picker_expanded = !self.online_state.source_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::online) fn online_source_picker_dismiss(&mut self) -> Task<AppMessage> {
        // 关闭图源选择器
        self.online_state.source_picker_expanded = false;
        Task::none()
    }
}
