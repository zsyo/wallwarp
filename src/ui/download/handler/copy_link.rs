// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn copy_download_link(&mut self, id: usize) -> Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let url = task.task.url.clone();
            let success_message = self.i18n.t("download-tasks.copy-link-success").to_string();
            let failed_message = self.i18n.t("download-tasks.copy-link-failed").to_string();

            // 异步复制到剪贴板（arboard 原生实现，三平台通用）
            return Task::perform(
                async move {
                    tokio::task::spawn_blocking(move || {
                        arboard::Clipboard::new()
                            .and_then(|mut clipboard| clipboard.set_text(&url))
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
                        format!("{}: {}", failed_message, e),
                        NotificationType::Error,
                    )
                    .into(),
                },
            );
        }
        Task::none()
    }
}
