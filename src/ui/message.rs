use super::ActivePage;

#[derive(Debug, Clone)]
pub enum AppMessage {
    LanguageSelected(String),
    WindowResized(u32, u32), // 窗口大小改变事件
    WindowMoved(i32, i32),   // 窗口位置改变事件
    PageSelected(ActivePage),
    DebounceTimer,
}
