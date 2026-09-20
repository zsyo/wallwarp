// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::OnlineState;
use crate::services::source::{
    ColorOption, PixabayCategory, Sorting, SourceColor, SourceKind, TimeRange,
};
use crate::services::wallhaven::{AspectRatio, Purity, Resolution, helper};
use crate::ui::online::ResolutionMode;
use crate::utils::config::Config;

impl OnlineState {
    /// 从 config.wallhaven 同步在线筛选字段
    ///
    /// 进入页面加载（load_from_config）与应用壁纸组（apply_group）共用：
    /// 应用组先把快照写入 config，再经本方法刷新筛选状态
    pub fn sync_filters_from_config(&mut self, config: &Config) {
        // 分类和纯净度（从字符串解析位掩码）
        self.categories = helper::parse_category_bitmask(&config.wallhaven.category);
        self.purities = helper::parse_purity_bitmask(&config.wallhaven.purity);

        // Wallhaven 未配置 API Key 时移除 NSFW 选项（Danbooru 免密钥、
        // 分级由其 rating metatag 自行映射，不依赖该 key）
        if self.source == SourceKind::Wallhaven && config.wallhaven.api_key.is_empty() {
            self.purities &= !Purity::NSFW.bit_value();
        }

        self.sorting = Sorting::parse(&config.wallhaven.sorting);
        self.color = ColorOption::parse(&config.wallhaven.color);
        self.source_color = SourceColor::parse(&config.wallhaven.source_color);
        self.pixabay_category = PixabayCategory::parse(&config.wallhaven.pixabay_category);
        self.editors_choice = config.wallhaven.editors_choice;
        self.time_range = TimeRange::parse(&config.wallhaven.top_range);

        // 分辨率模式与选中项
        self.resolution_mode = match config.wallhaven.resolution_mode.as_str() {
            "atleast" => ResolutionMode::AtLeast,
            "exactly" => ResolutionMode::Exactly,
            _ => ResolutionMode::All,
        };
        self.atleast_resolution = Self::parse_resolution(&config.wallhaven.atleast_resolution);
        self.selected_resolutions = config
            .wallhaven
            .resolutions
            .split(',')
            .filter_map(|s| Self::parse_resolution(s.trim()))
            .collect();

        // 比例列表与"全部横屏/竖屏"快捷项
        self.ratio_landscape_selected = false;
        self.ratio_portrait_selected = false;
        self.ratio_all_selected = config.wallhaven.ratios == "all";
        self.selected_ratios = Vec::new();

        if !self.ratio_all_selected && !config.wallhaven.ratios.is_empty() {
            let landscape_included = [
                AspectRatio::R16x9,
                AspectRatio::R16x10,
                AspectRatio::R21x9,
                AspectRatio::R32x9,
                AspectRatio::R48x9,
            ];
            let portrait_included = [AspectRatio::R9x16, AspectRatio::R10x16, AspectRatio::R9x18];

            for part in config.wallhaven.ratios.split(',').map(str::trim) {
                match part {
                    "landscape" => self.ratio_landscape_selected = true,
                    "portrait" => self.ratio_portrait_selected = true,
                    _ => {
                        if let Some(ratio) = Self::parse_ratio(part) {
                            self.selected_ratios.push(ratio);
                        }
                    }
                }
            }

            if self.ratio_landscape_selected {
                self.selected_ratios
                    .retain(|r| !landscape_included.contains(r));
            }
            if self.ratio_portrait_selected {
                self.selected_ratios
                    .retain(|r| !portrait_included.contains(r));
            }
        }
    }

    /// 解析分辨率字符串（合法取值表与 UI 选项一致）
    fn parse_resolution(s: &str) -> Option<Resolution> {
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
    }

    /// 解析比例字符串
    fn parse_ratio(s: &str) -> Option<AspectRatio> {
        match s {
            "16x9" => Some(AspectRatio::R16x9),
            "16x10" => Some(AspectRatio::R16x10),
            "21x9" => Some(AspectRatio::R21x9),
            "32x9" => Some(AspectRatio::R32x9),
            "48x9" => Some(AspectRatio::R48x9),
            "9x16" => Some(AspectRatio::R9x16),
            "10x16" => Some(AspectRatio::R10x16),
            "9x18" => Some(AspectRatio::R9x18),
            "1x1" => Some(AspectRatio::R1x1),
            "3x2" => Some(AspectRatio::R3x2),
            "4x3" => Some(AspectRatio::R4x3),
            "5x4" => Some(AspectRatio::R5x4),
            _ => None,
        }
    }
}
