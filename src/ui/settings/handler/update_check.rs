// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 检查更新（GitHub Releases）

use crate::services::update_checker::{self, UpdateCheckResult};
use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::{error, info};

impl App {
    /// 发起检查更新（进行中时忽略重复点击）
    pub(in crate::ui::settings) fn settings_check_update(&mut self) -> Task<AppMessage> {
        if self.settings_state.update_checking {
            return Task::none();
        }
        self.settings_state.update_checking = true;

        let proxy = self.config.resolved_proxy();
        let proxy_enabled = self.config.global.proxy_enabled;
        Task::perform(
            update_checker::check_for_update(proxy, proxy_enabled),
            |result| SettingsMessage::UpdateCheckFinished(result).into(),
        )
    }

    /// 检查更新完成：发现新版本时通知并打开发布页，否则提示结果
    pub(in crate::ui::settings) fn settings_update_check_finished(
        &mut self,
        result: Result<UpdateCheckResult, String>,
    ) -> Task<AppMessage> {
        self.settings_state.update_checking = false;

        match result {
            Ok(check) => {
                if check.has_update {
                    info!(
                        "[设置] [检查更新] 发现新版本: {}（当前 {}）",
                        check.latest_version, check.current_version
                    );
                    let message = self.i18n.t_with_args(
                        "settings.update-available",
                        &[("version", check.latest_version.clone())],
                    );
                    Task::batch([
                        self.show_notification(message, NotificationType::Info),
                        self.settings_open_url(check.release_url),
                    ])
                } else {
                    let message = self.i18n.t("settings.update-uptodate").to_string();
                    self.show_notification(message, NotificationType::Success)
                }
            }
            Err(e) => {
                error!("[设置] [检查更新] 失败: {}", e);
                let message = self
                    .i18n
                    .t_with_args("settings.update-check-failed", &[("error", e)]);
                self.show_notification(message, NotificationType::Error)
            }
        }
    }
}
