// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::OnlineState;
use crate::ui::online::ResolutionMode;
use crate::utils::config::Config;

impl OnlineState {
    /// 保存当前筛选条件到配置文件
    pub fn save_to_config(&self, config: &mut Config) {
        config.wallhaven.category = format!("{:03b}", self.categories);
        config.wallhaven.purity = format!("{:03b}", self.purities);
        config.wallhaven.sorting = self.sorting.to_string();
        config.wallhaven.color = self.color.value().to_string();
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

        config.wallhaven.ratios = ratios_vec.join(",");

        config.save_to_file();
    }
}
