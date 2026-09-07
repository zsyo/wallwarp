#[derive(Debug, Clone)]
pub enum CloseConfirmationAction {
    MinimizeToTray,
    CloseApp,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    None, // 空消息，用于某些不需要实际操作的情况
    Main(crate::ui::main::MainMessage),
    Local(crate::ui::local::LocalMessage),
    Online(crate::ui::online::OnlineMessage),
    Download(crate::ui::download::DownloadMessage),
    Settings(crate::ui::settings::SettingsMessage),
    AutoChange(crate::ui::auto_change::AutoChangeMessage),
    History(crate::ui::history::HistoryMessage),
    Favorites(crate::ui::favorites::FavoritesMessage),
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivePage {
    OnlineWallpapers,
    LocalList,
    DownloadProgress,
    WallpaperHistory,
    Favorites,
    Settings,
}

/// 设置页内部分类导航项
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    /// 通用：语言、主题、自启、悬浮球、关闭行为、日志
    General,
    /// 壁纸：模式与定时切换（预留多显示器独立壁纸）
    Wallpaper,
    /// 图源：壁纸在线来源与 API 密钥（预留多图源管理）
    Sources,
    /// 快捷键：占位，功能开发中
    Hotkeys,
    /// 网络：代理设置
    Network,
    /// 数据：路径配置
    Data,
    /// 关于：版本信息（预留检查更新）
    About,
}
