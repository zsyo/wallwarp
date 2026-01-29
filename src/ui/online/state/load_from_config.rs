// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::OnlineState;
use crate::services::wallhaven::{AspectRatio, Purity, Resolution};
use crate::ui::async_tasks;
use crate::ui::online::ResolutionMode;
use crate::utils::config::Config;

impl OnlineState {
    /// 从配置文件加载筛选条件
    pub fn load_from_config(config: &Config) -> Self {
        let mut state = Self::default();

        // 加载分类（从字符串解析位掩码）
        state.categories = async_tasks::parse_category_bitmask(&config.wallhaven.category);

        // 加载纯净度（从字符串解析位掩码）
        state.purities = async_tasks::parse_purity_bitmask(&config.wallhaven.purity);

        // 如果 API Key 为空，移除 NSFW 选项
        if config.wallhaven.api_key.is_empty() {
            state.purities &= !Purity::NSFW.bit_value();
        }

        // 加载排序
        state.sorting = async_tasks::parse_sorting(&config.wallhaven.sorting);

        // 加载颜色
        state.color = async_tasks::parse_color(&config.wallhaven.color);

        // 加载时间范围
        state.time_range = async_tasks::parse_time_range(&config.wallhaven.top_range);

        // 加载分辨率模式
        state.resolution_mode = match config.wallhaven.resolution_mode.as_str() {
            "all" => ResolutionMode::All,
            "atleast" => ResolutionMode::AtLeast,
            "exactly" => ResolutionMode::Exactly,
            _ => ResolutionMode::All,
        };

        // 加载AtLeast分辨率
        state.atleast_resolution = if !config.wallhaven.atleast_resolution.is_empty() {
            match config.wallhaven.atleast_resolution.as_str() {
                "2560x1080" => Some(Resolution::R2560x1080),
                "2560x1440" => Some(Resolution::R2560x1440U),
                "3840x1600" => Some(Resolution::R3840x1600),
                "1280x720" => Some(Resolution::R1280x720),
                "1600x900" => Some(Resolution::R1600x900),
                "1920x1080" => Some(Resolution::R1920x1080),
                "3840x2160" => Some(Resolution::R3840x2160),
                "1280x800" => Some(Resolution::R1280x800),
                "1600x1000" => Some(Resolution::R1600x1000),
                "1920x1200" => Some(Resolution::R1920x1200),
                "2560x1600" => Some(Resolution::R2560x1600),
                "3840x2400" => Some(Resolution::R3840x2400),
                "1280x960" => Some(Resolution::R1280x960),
                "1600x1200" => Some(Resolution::R1600x1200_4_3),
                "1920x1440" => Some(Resolution::R1920x1440),
                "2560x1920" => Some(Resolution::R2560x1920),
                "3840x2880" => Some(Resolution::R3840x2880),
                "1280x1024" => Some(Resolution::R1280x1024),
                "1600x1280" => Some(Resolution::R1600x1280),
                "1920x1536" => Some(Resolution::R1920x1536),
                "2560x2048" => Some(Resolution::R2560x2048),
                "3840x3072" => Some(Resolution::R3840x3072),
                _ => None,
            }
        } else {
            None
        };

        // 加载Exactly分辨率列表
        state.selected_resolutions = if !config.wallhaven.resolutions.is_empty() {
            let valid_resolutions = [
                "2560x1080",
                "2560x1440",
                "3840x1600",
                "1280x720",
                "1600x900",
                "1920x1080",
                "3840x2160",
                "1280x800",
                "1600x1000",
                "1920x1200",
                "2560x1600",
                "3840x2400",
                "1280x960",
                "1600x1200",
                "1920x1440",
                "2560x1920",
                "3840x2880",
                "1280x1024",
                "1600x1280",
                "1920x1536",
                "2560x2048",
                "3840x3072",
            ];

            let res_list: Vec<Resolution> = config
                .wallhaven
                .resolutions
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    if valid_resolutions.contains(&s) {
                        match s {
                            "2560x1080" => Some(Resolution::R2560x1080),
                            "2560x1440" => Some(Resolution::R2560x1440U),
                            "3840x1600" => Some(Resolution::R3840x1600),
                            "1280x720" => Some(Resolution::R1280x720),
                            "1600x900" => Some(Resolution::R1600x900),
                            "1920x1080" => Some(Resolution::R1920x1080),
                            "3840x2160" => Some(Resolution::R3840x2160),
                            "1280x800" => Some(Resolution::R1280x800),
                            "1600x1000" => Some(Resolution::R1600x1000),
                            "1920x1200" => Some(Resolution::R1920x1200),
                            "2560x1600" => Some(Resolution::R2560x1600),
                            "3840x2400" => Some(Resolution::R3840x2400),
                            "1280x960" => Some(Resolution::R1280x960),
                            "1600x1200" => Some(Resolution::R1600x1200_4_3),
                            "1920x1440" => Some(Resolution::R1920x1440),
                            "2560x1920" => Some(Resolution::R2560x1920),
                            "3840x2880" => Some(Resolution::R3840x2880),
                            "1280x1024" => Some(Resolution::R1280x1024),
                            "1600x1280" => Some(Resolution::R1600x1280),
                            "1920x1536" => Some(Resolution::R1920x1536),
                            "2560x2048" => Some(Resolution::R2560x2048),
                            "3840x3072" => Some(Resolution::R3840x3072),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            res_list
        } else {
            Vec::new()
        };

        // 加载比例列表和额外选项
        state.ratio_landscape_selected = false;
        state.ratio_portrait_selected = false;
        state.ratio_all_selected = config.wallhaven.ratios == "all";
        state.selected_ratios = Vec::new();

        if !state.ratio_all_selected && !config.wallhaven.ratios.is_empty() {
            let landscape_included = [
                AspectRatio::R16x9,
                AspectRatio::R16x10,
                AspectRatio::R21x9,
                AspectRatio::R32x9,
                AspectRatio::R48x9,
            ];
            let portrait_included = [AspectRatio::R9x16, AspectRatio::R10x16, AspectRatio::R9x18];

            let parts: Vec<&str> = config.wallhaven.ratios.split(',').collect();

            for part in parts {
                let part = part.trim();
                match part {
                    "landscape" => {
                        state.ratio_landscape_selected = true;
                    }
                    "portrait" => {
                        state.ratio_portrait_selected = true;
                    }
                    "16x9" => state.selected_ratios.push(AspectRatio::R16x9),
                    "16x10" => state.selected_ratios.push(AspectRatio::R16x10),
                    "21x9" => state.selected_ratios.push(AspectRatio::R21x9),
                    "32x9" => state.selected_ratios.push(AspectRatio::R32x9),
                    "48x9" => state.selected_ratios.push(AspectRatio::R48x9),
                    "9x16" => state.selected_ratios.push(AspectRatio::R9x16),
                    "10x16" => state.selected_ratios.push(AspectRatio::R10x16),
                    "9x18" => state.selected_ratios.push(AspectRatio::R9x18),
                    "1x1" => state.selected_ratios.push(AspectRatio::R1x1),
                    "3x2" => state.selected_ratios.push(AspectRatio::R3x2),
                    "4x3" => state.selected_ratios.push(AspectRatio::R4x3),
                    "5x4" => state.selected_ratios.push(AspectRatio::R5x4),
                    _ => {}
                }
            }

            if state.ratio_landscape_selected {
                state.selected_ratios.retain(|r| !landscape_included.contains(r));
            }
            if state.ratio_portrait_selected {
                state.selected_ratios.retain(|r| !portrait_included.contains(r));
            }
        }

        state.has_loaded = false;

        state
    }
}
