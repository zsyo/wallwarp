// Copyright (C) 2026 zsyo - GNU GPL v3.0

use super::OnlineState;
use crate::ui::online::ResolutionMode;
use crate::utils::config::Config;

impl OnlineState {
    /// 将当前筛选条件同步到内存中的配置
    ///
    /// 只更新内存中的 config，磁盘写入统一由 App 层的
    /// request_config_save()（300ms 防抖）合并执行，调用方负责触发
    pub fn save_to_config(&self, config: &mut Config) {
        config.global.online_source = self.source.id().to_string();
        config.wallhaven.category = format!("{:03b}", self.categories);
        config.wallhaven.purity = format!("{:03b}", self.purities);
        config.wallhaven.sorting = self.sorting.to_string();
        config.wallhaven.color = self.color.value().to_string();
        config.wallhaven.source_color = self.source_color.value().to_string();
        config.wallhaven.site_category = self
            .site_category
            .api_value(self.source)
            .unwrap_or_default()
            .to_string();
        config.wallhaven.top_range = self.time_range.value().to_string();

        config.wallhaven.resolution_mode = match self.resolution_mode {
            ResolutionMode::All => "all".to_string(),
            ResolutionMode::AtLeast => "atleast".to_string(),
            ResolutionMode::Exactly => "exactly".to_string(),
        };

        config.wallhaven.atleast_resolution = if let Some(res) = self.atleast_resolution {
            res.value().to_string()
        } else {
            String::new()
        };

        config.wallhaven.resolutions = if !self.selected_resolutions.is_empty() {
            let res_list: Vec<String> = self
                .selected_resolutions
                .iter()
                .map(|r| r.value().to_string())
                .collect();
            res_list.join(",")
        } else {
            String::new()
        };

        config.wallhaven.ratios = self.ratios_to_config_value();
    }

    /// 将比例选择状态组装为 config 取值（快捷项 + 精确项，逗号分隔）
    ///
    /// 配置同步（save_to_config）与壁纸组筛选快照共用
    pub fn ratios_to_config_value(&self) -> String {
        let mut ratios_vec = Vec::new();

        if self.ratio_landscape_selected {
            ratios_vec.push("landscape".to_string());
        }

        if self.ratio_portrait_selected {
            ratios_vec.push("portrait".to_string());
        }

        for ratio in &self.selected_ratios {
            ratios_vec.push(ratio.value().to_string());
        }

        ratios_vec.join(",")
    }
}
