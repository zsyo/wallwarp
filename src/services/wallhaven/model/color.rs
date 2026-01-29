// Copyright (C) 2026 zsyo - GNU AGPL v3.0

// 颜色选项（官方接口支持的29种颜色 + Any）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorOption {
    Any,
    Color660000, // 深红
    Color990000, // 正红
    ColorCC0000, // 亮红
    ColorCC3333, // 浅红
    ColorEA4C88, // 粉红
    Color993399, // 紫红
    Color663399, // 深紫
    Color333399, // 蓝紫
    Color0066CC, // 宝蓝
    Color0099CC, // 天蓝
    Color66CCCC, // 青绿
    Color77CC33, // 草绿
    Color669900, // 翠绿
    Color336600, // 深绿
    Color666600, // 橄榄绿
    Color999900, // 黄绿
    ColorCCCC33, // 柠檬黄
    ColorFFFF00, // 亮黄
    ColorFFCC33, // 金黄
    ColorFF9900, // 橙黄
    ColorFF6600, // 橘红
    ColorCC6633, // 砖红
    Color996633, // 棕褐
    Color663300, // 深棕
    Color000000, // 纯黑
    Color999999, // 深灰
    ColorCCCCCC, // 中灰
    ColorFFFFFF, // 纯白
    Color424153, // 深灰蓝
}

impl ColorOption {
    pub fn all() -> [ColorOption; 30] {
        [
            ColorOption::Any,
            ColorOption::Color660000,
            ColorOption::Color990000,
            ColorOption::ColorCC0000,
            ColorOption::ColorCC3333,
            ColorOption::ColorEA4C88,
            ColorOption::Color993399,
            ColorOption::Color663399,
            ColorOption::Color333399,
            ColorOption::Color0066CC,
            ColorOption::Color0099CC,
            ColorOption::Color66CCCC,
            ColorOption::Color77CC33,
            ColorOption::Color669900,
            ColorOption::Color336600,
            ColorOption::Color666600,
            ColorOption::Color999900,
            ColorOption::ColorCCCC33,
            ColorOption::ColorFFFF00,
            ColorOption::ColorFFCC33,
            ColorOption::ColorFF9900,
            ColorOption::ColorFF6600,
            ColorOption::ColorCC6633,
            ColorOption::Color996633,
            ColorOption::Color663300,
            ColorOption::Color000000,
            ColorOption::Color999999,
            ColorOption::ColorCCCCCC,
            ColorOption::ColorFFFFFF,
            ColorOption::Color424153,
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            ColorOption::Any => "any",
            ColorOption::Color660000 => "660000",
            ColorOption::Color990000 => "990000",
            ColorOption::ColorCC0000 => "cc0000",
            ColorOption::ColorCC3333 => "cc3333",
            ColorOption::ColorEA4C88 => "ea4c88",
            ColorOption::Color993399 => "993399",
            ColorOption::Color663399 => "663399",
            ColorOption::Color333399 => "333399",
            ColorOption::Color0066CC => "0066cc",
            ColorOption::Color0099CC => "0099cc",
            ColorOption::Color66CCCC => "66cccc",
            ColorOption::Color77CC33 => "77cc33",
            ColorOption::Color669900 => "669900",
            ColorOption::Color336600 => "336600",
            ColorOption::Color666600 => "666600",
            ColorOption::Color999900 => "999900",
            ColorOption::ColorCCCC33 => "cccc33",
            ColorOption::ColorFFFF00 => "ffff00",
            ColorOption::ColorFFCC33 => "ffcc33",
            ColorOption::ColorFF9900 => "ff9900",
            ColorOption::ColorFF6600 => "ff6600",
            ColorOption::ColorCC6633 => "cc6633",
            ColorOption::Color996633 => "996633",
            ColorOption::Color663300 => "663300",
            ColorOption::Color000000 => "000000",
            ColorOption::Color999999 => "999999",
            ColorOption::ColorCCCCCC => "cccccc",
            ColorOption::ColorFFFFFF => "ffffff",
            ColorOption::Color424153 => "424153",
        }
    }
}

impl std::fmt::Display for ColorOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
