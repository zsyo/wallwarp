// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::OnlineState;
use crate::services::source::SourceKind;
use crate::utils::config::Config;

impl OnlineState {
    /// 从配置文件加载筛选条件
    pub fn load_from_config(config: &Config) -> Self {
        let mut state = Self {
            source: SourceKind::parse(&config.global.online_source),
            ..Self::default()
        };

        // 从 config.wallhaven 同步各筛选字段（与应用壁纸组共用）
        state.sync_filters_from_config(config);

        state.has_loaded = false;

        state
    }
}
