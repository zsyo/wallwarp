// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 剪贴板写入

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;

impl App {
    /// 异步复制文本到剪贴板（arboard 原生实现，三平台通用）
    ///
    /// 成功/失败均以 toast 通知反馈
    pub(in crate::ui) fn copy_text_to_clipboard(
        &mut self,
        text: String,
        success_message: String,
        failed_message_prefix: String,
    ) -> Task<AppMessage> {
        Task::perform(
            async move {
                tokio::task::spawn_blocking(move || {
                    arboard::Clipboard::new()
                        .and_then(|mut clipboard| clipboard.set_text(&text))
                        .map_err(|e| e.to_string())
                })
                .await
                .map_err(|e| format!("任务中断: {e}"))?
            },
            move |result| match result {
                Ok(()) => {
                    MainMessage::ShowNotification(success_message, NotificationType::Success)
                        .into()
                }
                Err(e) => MainMessage::ShowNotification(
                    format!("{}: {}", failed_message_prefix, e),
                    NotificationType::Error,
                )
                .into(),
            },
        )
    }
}
