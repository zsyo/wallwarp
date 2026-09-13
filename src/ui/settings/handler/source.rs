// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 在线图源选择（设置页图源区块）

use crate::services::source::SourceKind;
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    /// 在线图源选择（状态持久化到配置）
    ///
    /// 在线页标记为未加载，下次进入时按新图源重新拉取列表
    pub(in crate::ui::settings) fn settings_online_source_selected(
        &mut self,
        kind: SourceKind,
    ) -> Task<AppMessage> {
        let old = self.online_state.source;
        info!("[设置] [在线图源] 修改: {} -> {}", old.id(), kind.id());
        self.online_state.source = kind;
        self.config.global.online_source = kind.id().to_string();
        // 图源已变化，在线页下次进入时按新图源重新加载
        self.online_state.has_loaded = false;
        self.online_state.current_page = 1;
        // 选择后关闭选择器，磁盘写入经防抖合并
        self.settings_state.online_source_picker_expanded = false;
        self.request_config_save()
    }

    pub(in crate::ui::settings) fn settings_online_source_picker_expanded(
        &mut self,
    ) -> Task<AppMessage> {
        // 切换图源选择器的展开/收起状态
        self.settings_state.online_source_picker_expanded =
            !self.settings_state.online_source_picker_expanded;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_online_source_picker_dismiss(
        &mut self,
    ) -> Task<AppMessage> {
        self.settings_state.online_source_picker_expanded = false;
        Task::none()
    }
}
