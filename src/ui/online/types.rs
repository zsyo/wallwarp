// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;

/// 显示用的时间范围包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableTimeRange {
    pub value: wallhaven::TimeRange,
    pub display: &'static str,
}

impl std::fmt::Display for DisplayableTimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

/// 显示用的排序方式包装类型，用于 pick_list 显示翻译后的文本
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayableSorting {
    pub value: wallhaven::Sorting,
    pub display: &'static str,
}

impl std::fmt::Display for DisplayableSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}
