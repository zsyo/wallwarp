// Copyright (C) 2026 zsyo - GNU AGPL v3.0

// 时间范围选项（仅用于 toplist 排序）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRange {
    Day,
    ThreeDays,
    Week,
    Month,
    ThreeMonths,
    SixMonths,
    Year,
}

impl TimeRange {
    pub fn all() -> [TimeRange; 7] {
        [
            TimeRange::Day,
            TimeRange::ThreeDays,
            TimeRange::Week,
            TimeRange::Month,
            TimeRange::ThreeMonths,
            TimeRange::SixMonths,
            TimeRange::Year,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            TimeRange::Day => "1d",
            TimeRange::ThreeDays => "3d",
            TimeRange::Week => "1w",
            TimeRange::Month => "1M",
            TimeRange::ThreeMonths => "3M",
            TimeRange::SixMonths => "6M",
            TimeRange::Year => "1y",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TimeRange::Day => "online-wallpapers.time-last-day",
            TimeRange::ThreeDays => "online-wallpapers.time-last-three-days",
            TimeRange::Week => "online-wallpapers.time-last-week",
            TimeRange::Month => "online-wallpapers.time-last-month",
            TimeRange::ThreeMonths => "online-wallpapers.time-last-three-months",
            TimeRange::SixMonths => "online-wallpapers.time-last-six-months",
            TimeRange::Year => "online-wallpapers.time-last-year",
        }
    }
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
