// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::online::ResolutionMode;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_filter_category_toggled(
        &mut self,
        category: wallhaven::Category,
    ) -> Task<AppMessage> {
        // 切换分类：使用位掩码而不是枚举索引值
        self.online_state.categories ^= category.bit_value();
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_sorting_changed(
        &mut self,
        sorting: wallhaven::Sorting,
    ) -> Task<AppMessage> {
        self.online_state.sorting = sorting;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_purity_toggled(
        &mut self,
        purity: wallhaven::Purity,
    ) -> Task<AppMessage> {
        // 切换纯净度：使用位掩码而不是枚举索引值
        self.online_state.purities ^= purity.bit_value();
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_search_text_changed(&mut self, text: String) -> Task<AppMessage> {
        self.online_state.search_text = text;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_changed(
        &mut self,
        resolution: wallhaven::Resolution,
    ) -> Task<AppMessage> {
        self.online_state.resolution = resolution;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_changed(&mut self, ratio: wallhaven::Ratio) -> Task<AppMessage> {
        self.online_state.ratio = ratio;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_color_changed(
        &mut self,
        color: wallhaven::ColorOption,
    ) -> Task<AppMessage> {
        self.online_state.color = color;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        // 选择颜色后自动关闭颜色选择器
        self.online_state.color_picker_expanded = false;
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_time_range_changed(
        &mut self,
        time_range: wallhaven::TimeRange,
    ) -> Task<AppMessage> {
        self.online_state.time_range = time_range;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_mode_changed(
        &mut self,
        mode: ResolutionMode,
    ) -> Task<AppMessage> {
        // 切换分辨率筛选模式
        self.online_state.resolution_mode = mode;
        // 切换模式时清空之前的选择
        self.online_state.selected_resolutions.clear();
        self.online_state.atleast_resolution = None;
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_toggled(
        &mut self,
        resolution: wallhaven::Resolution,
    ) -> Task<AppMessage> {
        // Exactly模式：切换分辨率选择状态
        if let Some(pos) = self
            .online_state
            .selected_resolutions
            .iter()
            .position(|&r| r == resolution)
        {
            // 如果已选中，则取消选中
            self.online_state.selected_resolutions.remove(pos);
        } else {
            // 如果未选中，则添加到选中列表
            self.online_state.selected_resolutions.push(resolution);
        }
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_resolution_atleast_selected(
        &mut self,
        resolution: wallhaven::Resolution,
    ) -> Task<AppMessage> {
        // AtLeast模式：选择分辨率（不自动关闭）
        self.online_state.atleast_resolution = Some(resolution);
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_toggled(
        &mut self,
        ratio: wallhaven::AspectRatio,
    ) -> Task<AppMessage> {
        // 切换比例选择状态（多选）
        if let Some(pos) = self.online_state.selected_ratios.iter().position(|&r| r == ratio) {
            // 如果已选中，则取消选中
            self.online_state.selected_ratios.remove(pos);
        } else {
            // 如果未选中，则添加到选中列表
            self.online_state.selected_ratios.push(ratio);
        }
        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_landscape_toggled(&mut self) -> Task<AppMessage> {
        // 切换"全部横屏"选项
        self.online_state.ratio_landscape_selected = !self.online_state.ratio_landscape_selected;

        // 如果选中"全部横屏"，清空宽屏和超宽屏分组下的选中项
        if self.online_state.ratio_landscape_selected {
            self.online_state.selected_ratios.retain(|r| {
                !matches!(
                    r,
                    wallhaven::AspectRatio::R16x9
                        | wallhaven::AspectRatio::R16x10
                        | wallhaven::AspectRatio::R21x9
                        | wallhaven::AspectRatio::R32x9
                        | wallhaven::AspectRatio::R48x9
                )
            });
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_portrait_toggled(&mut self) -> Task<AppMessage> {
        // 切换"全部竖屏"选项
        self.online_state.ratio_portrait_selected = !self.online_state.ratio_portrait_selected;

        // 如果选中"全部竖屏"，清空竖屏分组下的选中项
        if self.online_state.ratio_portrait_selected {
            self.online_state.selected_ratios.retain(|r| {
                !matches!(
                    r,
                    wallhaven::AspectRatio::R9x16 | wallhaven::AspectRatio::R10x16 | wallhaven::AspectRatio::R9x18
                )
            });
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }

    pub(in crate::ui::online) fn online_filter_ratio_all_toggled(&mut self) -> Task<AppMessage> {
        // 切换"全部"选项
        self.online_state.ratio_all_selected = !self.online_state.ratio_all_selected;

        // 如果选中"全部"，清空其他所有选项的选中状态
        if self.online_state.ratio_all_selected {
            self.online_state.ratio_landscape_selected = false;
            self.online_state.ratio_portrait_selected = false;
            self.online_state.selected_ratios.clear();
        }

        // 保存到配置文件
        self.online_state.save_to_config(&mut self.config);
        Task::none()
    }
}
