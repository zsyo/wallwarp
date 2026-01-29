// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::wgpu::rwh::RawWindowHandle;
use iced::window;
use windows::Win32::Foundation::HWND;

impl App {
    /// 处理延迟保存
    pub(in crate::ui::main) fn drag_resize_window(&mut self, direction: window::Direction) -> Task<AppMessage> {
        window::oldest().and_then(move |id: iced::window::Id| window::drag_resize(id, direction))
    }

    pub fn enable_window_drag_resize(&self) -> Task<AppMessage> {
        window::oldest().and_then(move |id| {
            window::run(id, move |mw| {
                if let Ok(handle) = mw.window_handle() {
                    if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                        let hwnd = HWND(win32_handle.hwnd.get() as _);

                        unsafe {
                            use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
                            use windows::Win32::UI::Controls::MARGINS;
                            use windows::Win32::UI::WindowsAndMessaging::{
                                GWL_STYLE, GetWindowLongPtrW, SetWindowLongPtrW, WS_SIZEBOX, WS_THICKFRAME,
                            };

                            // 获取当前窗口样式
                            let style = GetWindowLongPtrW(hwnd, GWL_STYLE);

                            // 添加 WS_THICKFRAME 和 WS_SIZEBOX 样式以启用窗口边缘调整大小和边框
                            let new_style = style | WS_THICKFRAME.0 as isize | WS_SIZEBOX.0 as isize;

                            // 设置新的窗口样式
                            let _ = SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);

                            // 启用窗口阴影效果
                            // 将边距设置为 -1，表示整个窗口都有阴影
                            let margins = MARGINS {
                                cxLeftWidth: -1,
                                cxRightWidth: -1,
                                cyTopHeight: -1,
                                cyBottomHeight: -1,
                            };
                            let _ = DwmExtendFrameIntoClientArea(hwnd, &margins);
                        }
                    }
                }
            })
            .map(|_| AppMessage::None)
        })
    }
}
