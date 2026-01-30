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
    Settings,
}
