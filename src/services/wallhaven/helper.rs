/// Copyright (C) 2026 zsyo - GNU AGPL v3.0
use super::{ColorOption, Sorting, TimeRange};

/// 生成下载文件名
pub fn generate_file_name(id: &str, file_type: &str) -> String {
    format!("wallhaven-{}.{}", id, file_type)
}

/// 解析分类位掩码
pub fn parse_category_bitmask(category: &str) -> u32 {
    let mut result = 0u32;
    for (i, c) in category.chars().enumerate() {
        if c == '1' && i < 3 {
            result |= 1 << (2 - i);
        }
    }
    result
}

/// 解析纯净度位掩码
pub fn parse_purity_bitmask(purity: &str) -> u32 {
    match purity {
        "100" | "sfw" => 0b100,
        "010" | "sketchy" => 0b010,
        "001" | "nsfw" => 0b001,
        "110" => 0b110, // sfw + sketchy
        "101" => 0b101, // sfw + nsfw
        "011" => 0b011, // sketchy + nsfw
        "111" => 0b111, // all
        _ => 0b100,
    }
}

/// 解析排序方式
pub fn parse_sorting(sorting: &str) -> Sorting {
    match sorting {
        "date_added" => Sorting::DateAdded,
        "views" => Sorting::Views,
        "favorites" => Sorting::Favorites,
        "toplist" => Sorting::TopList,
        "random" => Sorting::Random,
        "relevance" => Sorting::Relevance,
        "hot" => Sorting::Hot,
        _ => Sorting::DateAdded,
    }
}

/// 解析颜色选项
pub fn parse_color(color: &str) -> ColorOption {
    match color.to_lowercase().as_str() {
        "any" => ColorOption::Any,
        "660000" => ColorOption::Color660000,
        "990000" => ColorOption::Color990000,
        "cc0000" => ColorOption::ColorCC0000,
        "cc3333" => ColorOption::ColorCC3333,
        "ea4c88" => ColorOption::ColorEA4C88,
        "993399" => ColorOption::Color993399,
        "663399" => ColorOption::Color663399,
        "333399" => ColorOption::Color333399,
        "0066cc" => ColorOption::Color0066CC,
        "0099cc" => ColorOption::Color0099CC,
        "66cccc" => ColorOption::Color66CCCC,
        "77cc33" => ColorOption::Color77CC33,
        "669900" => ColorOption::Color669900,
        "336600" => ColorOption::Color336600,
        "666600" => ColorOption::Color666600,
        "999900" => ColorOption::Color999900,
        "cccc33" => ColorOption::ColorCCCC33,
        "ffff00" => ColorOption::ColorFFFF00,
        "ffcc33" => ColorOption::ColorFFCC33,
        "ff9900" => ColorOption::ColorFF9900,
        "ff6600" => ColorOption::ColorFF6600,
        "cc6633" => ColorOption::ColorCC6633,
        "996633" => ColorOption::Color996633,
        "663300" => ColorOption::Color663300,
        "000000" => ColorOption::Color000000,
        "999999" => ColorOption::Color999999,
        "cccccc" => ColorOption::ColorCCCCCC,
        "ffffff" => ColorOption::ColorFFFFFF,
        "424153" => ColorOption::Color424153,
        _ => ColorOption::Any,
    }
}

/// 解析时间范围
pub fn parse_time_range(time_range: &str) -> TimeRange {
    match time_range {
        "1d" => TimeRange::Day,
        "3d" => TimeRange::ThreeDays,
        "1w" => TimeRange::Week,
        "1M" => TimeRange::Month,
        "3M" => TimeRange::ThreeMonths,
        "6M" => TimeRange::SixMonths,
        "1y" => TimeRange::Year,
        _ => TimeRange::Month,
    }
}
