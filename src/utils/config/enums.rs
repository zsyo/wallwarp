// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 配置枚举定义（含"纯标记枚举"的 as_str/parse/Display 样板宏）

use serde::{Deserialize, Serialize};

/// 为"纯标记枚举"生成 as_str/parse/Display 样板
///
/// - `as_str`：配置文件中的序列化取值；
/// - `parse`：未知取值回退到默认变体（解析不会失败）；
/// - `Display`：展示文本（与变体名不必一致，如 Crop 展示为 Fill）。
macro_rules! impl_config_enum {
    ($name:ident { default: $default:ident; $( $variant:ident => $as_str:literal, $display:literal; )* }) => {
        impl $name {
            /// 配置文件中的序列化取值
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $as_str, )*
                }
            }

            /// 解析配置取值；未知取值回退到默认变体，不会失败
            pub fn parse(s: &str) -> Self {
                match s {
                    $( $as_str => $name::$variant, )*
                    _ => $name::$default,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let text = match self {
                    $( $name::$variant => $display, )*
                };
                write!(f, "{text}")
            }
        }
    };
}
impl_config_enum!(WallpaperAutoChangeMode {
default: Online;
Online => "online", "Online";
Local => "local", "Local";
});

/// 定时切换的图片来源
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperAutoChangeMode {
    // 默认在线模式：首次安装时本地壁纸库为空，在线模式保证
    // 定时切换/托盘"切换下一张"开箱即用
    #[default]
    Online,
    Local,
}

/// 壁纸可读性处理选项（参考 Variety：避免浅色壁纸下桌面图标看不清）
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperReadability {
    /// 不处理，直接使用原图
    #[default]
    None,
    /// 暗化：叠加半透明黑遮罩
    Dim,
    /// 模糊：高斯模糊
    Blur,
    /// 暗化 + 模糊
    DimBlur,
}
impl_config_enum!(WallpaperReadability {
default: None;
None => "none", "None";
Dim => "dim", "Dim";
Blur => "blur", "Blur";
DimBlur => "dim_blur", "Dim + Blur";
});

/// 定时切换的周期取值（预定义档位 + 自定义分钟数）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WallpaperAutoChangeInterval {
    #[default]
    Off,
    Minutes(u32),
    Custom(u32),
}

impl Serialize for WallpaperAutoChangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for WallpaperAutoChangeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(WallpaperAutoChangeInterval::parse(&s).unwrap_or_default())
    }
}

impl WallpaperAutoChangeInterval {
    pub fn as_str(&self) -> String {
        match self {
            WallpaperAutoChangeInterval::Off => "off".to_string(),
            WallpaperAutoChangeInterval::Minutes(10) => "10m".to_string(),
            WallpaperAutoChangeInterval::Minutes(30) => "30m".to_string(),
            WallpaperAutoChangeInterval::Minutes(60) => "1h".to_string(),
            WallpaperAutoChangeInterval::Minutes(120) => "2h".to_string(),
            WallpaperAutoChangeInterval::Custom(minutes) => format!("custom:{}m", minutes),
            WallpaperAutoChangeInterval::Minutes(_) => {
                // 其他未预定义的 Minutes 值，序列化为 custom: 格式
                if let Some(minutes) = self.get_minutes() {
                    format!("custom:{}m", minutes)
                } else {
                    "off".to_string()
                }
            }
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "off" => Some(WallpaperAutoChangeInterval::Off),
            "10m" => Some(WallpaperAutoChangeInterval::Minutes(10)),
            "30m" => Some(WallpaperAutoChangeInterval::Minutes(30)),
            "1h" => Some(WallpaperAutoChangeInterval::Minutes(60)),
            "2h" => Some(WallpaperAutoChangeInterval::Minutes(120)),
            s if s.starts_with("custom:") && s.ends_with('m') => {
                // 解析 custom:XXm 格式
                let inner = &s[7..s.len() - 1]; // 去掉 "custom:" 和 "m"
                if let Ok(minutes) = inner.parse::<u32>() {
                    if (1..=1440).contains(&minutes) {
                        Some(WallpaperAutoChangeInterval::Custom(minutes))
                    } else {
                        Some(WallpaperAutoChangeInterval::Off)
                    }
                } else {
                    Some(WallpaperAutoChangeInterval::Off)
                }
            }
            _ => Some(WallpaperAutoChangeInterval::Off),
        }
    }

    pub fn get_minutes(&self) -> Option<u32> {
        match self {
            WallpaperAutoChangeInterval::Off => None,
            WallpaperAutoChangeInterval::Minutes(m) => Some(*m),
            WallpaperAutoChangeInterval::Custom(m) => Some(*m),
        }
    }
}

impl std::fmt::Display for WallpaperAutoChangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperAutoChangeInterval::Off => write!(f, "Off"),
            WallpaperAutoChangeInterval::Minutes(10) => write!(f, "10 min"),
            WallpaperAutoChangeInterval::Minutes(30) => write!(f, "30 min"),
            WallpaperAutoChangeInterval::Minutes(60) => write!(f, "1 hour"),
            WallpaperAutoChangeInterval::Minutes(120) => write!(f, "2 hours"),
            WallpaperAutoChangeInterval::Minutes(m) => write!(f, "{} min", m),
            WallpaperAutoChangeInterval::Custom(m) => write!(f, "{} min", m),
        }
    }
}

/// 壁纸铺满模式
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperMode {
    #[default]
    Crop,
    Fit,
    Stretch,
    Tile,
    Center,
    Span,
}

impl_config_enum!(WallpaperMode {
default: Crop;
Crop => "crop", "Fill";
Fit => "fit", "Fit";
Stretch => "stretch", "Stretch";
Tile => "tile", "Tile";
Center => "center", "Center";
Span => "span", "Span";
});

/// 日志等级配置
#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        }
    }

    /// 按严重程度从高到低排列的全部档位，供设置页下拉框遍历
    pub fn all() -> [LogLevel; 5] {
        [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ]
    }
}

/// 主题配置
#[derive(Clone, Serialize, Deserialize, Copy, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Dark,
    Light,
    Auto,
}

impl_config_enum!(Theme {
default: Dark;
Dark => "dark", "Dark";
Light => "light", "Light";
Auto => "auto", "Auto";
});

/// 窗口关闭按钮行为
#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    #[default]
    Ask,
    MinimizeToTray,
    CloseApp,
}

impl_config_enum!(CloseAction {
default: Ask;
Ask => "ask", "Ask";
MinimizeToTray => "minimize_to_tray", "MinimizeToTray";
CloseApp => "close_app", "CloseApp";
});
