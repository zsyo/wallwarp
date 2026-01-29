// Copyright (C) 2026 zsyo - GNU AGPL v3.0

// 比例选项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ratio {
    Any,
    Portrait,
    Landscape,
    Square,
}

impl Ratio {
    pub fn all() -> [Ratio; 4] {
        [Ratio::Any, Ratio::Portrait, Ratio::Landscape, Ratio::Square]
    }

    pub fn value(&self) -> &str {
        match self {
            Ratio::Any => "any",
            Ratio::Portrait => "portrait",
            Ratio::Landscape => "landscape",
            Ratio::Square => "square",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Ratio::Any => "online-wallpapers.ratio-any",
            Ratio::Portrait => "online-wallpapers.ratio-portrait",
            Ratio::Landscape => "online-wallpapers.ratio-landscape",
            Ratio::Square => "online-wallpapers.ratio-square",
        }
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
