// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::window_utils;
use iced::Task;
use iced::wgpu::rwh::RawWindowHandle;
use iced::window;
use windows::Win32::Foundation::HWND;

impl App {
    pub(in crate::ui::main) fn show_window(&mut self) -> Task<AppMessage> {
        // 显示窗口并检测状态，如果最小化或不在前台则置顶
        window::oldest().and_then(move |window_id| {
            Task::batch(vec![
                // 先设置窗口为 Windowed 模式（从 Hidden 恢复）
                window::set_mode(window_id, window::Mode::Windowed),
                // 然后检测窗口状态并置顶
                window::run(window_id, move |mw| {
                    if let Ok(handle) = mw.window_handle() {
                        if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                            let hwnd = HWND(win32_handle.hwnd.get() as _);

                            // 使用 Windows API 检测窗口状态并置顶
                            let is_minimized = window_utils::is_window_minimized(hwnd);
                            let is_foreground = window_utils::is_window_foreground(hwnd);

                            // 如果窗口最小化或不在前台，则恢复并置顶
                            if is_minimized || !is_foreground {
                                tracing::info!(
                                    "[显示窗口] 窗口状态 - 最小化: {}, 前台: {}, 执行恢复和置顶",
                                    is_minimized,
                                    is_foreground
                                );
                                let success = window_utils::restore_and_bring_to_front(hwnd);
                                tracing::info!(" [显示窗口] 置顶操作结果: {}", success);
                            } else {
                                tracing::info!("[显示窗口] 窗口已在前台，无需置顶");
                            }
                        }
                    }
                })
                .map(|_| AppMessage::None),
            ])
        })
    }
}
