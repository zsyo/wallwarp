// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 日志展示窗口消息与处理器

use super::{LogWindowState, SCROLL_ID};
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::config::{LOG_WINDOW_MAX_LINES_MAX, LOG_WINDOW_MAX_LINES_MIN, LogLevel};
use crate::utils::log_window_layer;
use iced::Task;
use iced::widget::{operation, scrollable};

impl From<LogWindowMessage> for AppMessage {
    fn from(message: LogWindowMessage) -> Self {
        AppMessage::LogWindow(message)
    }
}

/// 日志展示窗口消息
#[derive(Debug, Clone)]
pub enum LogWindowMessage {
    /// 打开窗口（设置页入口；已打开则聚焦）
    Open,
    /// 主动关闭窗口
    Close,
    /// 原生关闭按钮触发的关窗事件（携带窗口 Id，先过滤归属）
    CloseRequested(iced::window::Id),
    /// 切换展示等级（只影响本窗口，不动文件/控制台日志）
    LevelSelected(LogLevel),
    /// 展开/收起等级下拉框
    LevelPickerExpanded,
    /// 点击遮罩关闭等级下拉框
    LevelPickerDismiss,
    /// 修改最大保留行数输入框内容（仅本地缓冲，点保存/回车才应用）
    MaxLinesChanged(String),
    /// 提交最大保留行数（保存按钮/回车；区间外或非数字时提示错误）
    MaxLinesSubmitted,
    /// 修改筛选关键字（仅影响显示，后台照常全量记录）
    FilterChanged(String),
    /// 自动滚动开关切换
    AutoScrollToggled(bool),
    /// 清空当前展示内容与内存缓冲
    ClearBuffer,
    /// 定时拉取内存层增量日志
    Tick,
}

impl App {
    /// 处理日志展示窗口消息
    pub(crate) fn handle_log_window_message(
        &mut self,
        message: LogWindowMessage,
    ) -> Task<AppMessage> {
        match message {
            LogWindowMessage::Open => self.log_window_open(),
            LogWindowMessage::Close => self.log_window_close(),
            LogWindowMessage::CloseRequested(id) => self.log_window_close_requested(id),
            LogWindowMessage::LevelSelected(level) => self.log_window_level_selected(level),
            LogWindowMessage::LevelPickerExpanded => {
                self.log_window_state.level_picker_expanded =
                    !self.log_window_state.level_picker_expanded;
                Task::none()
            }
            LogWindowMessage::LevelPickerDismiss => {
                self.log_window_state.level_picker_expanded = false;
                Task::none()
            }
            LogWindowMessage::MaxLinesChanged(input) => self.log_window_max_lines_changed(input),
            LogWindowMessage::MaxLinesSubmitted => self.log_window_max_lines_submitted(),
            LogWindowMessage::FilterChanged(keyword) => {
                self.log_window_state.filter_text = keyword;
                Task::none()
            }
            LogWindowMessage::AutoScrollToggled(enabled) => {
                self.log_window_state.auto_scroll = enabled;
                if enabled {
                    self.log_window_scroll_to_bottom()
                } else {
                    Task::none()
                }
            }
            LogWindowMessage::ClearBuffer => {
                log_window_layer::clear_buffer();
                self.log_window_state.lines.clear();
                Task::none()
            }
            LogWindowMessage::Tick => self.log_window_tick(),
        }
    }

    /// 打开日志窗口；已打开则聚焦
    fn log_window_open(&mut self) -> Task<AppMessage> {
        if let Some(id) = self.log_window_id {
            return iced::window::gain_focus::<AppMessage>(id);
        }

        // 展示缓冲重置，输入框同步当前配置值
        self.log_window_state = LogWindowState {
            max_lines_input: self.config.global.log_window_max_lines.to_string(),
            ..LogWindowState::default()
        };
        // 激活内存层收集，等级取窗口独立配置（不影响文件/控制台层）
        log_window_layer::set_log_window_level(Some(self.config.global.log_window_level));

        let (id, open) = iced::window::open(super::log_window_settings());
        self.log_window_id = Some(id);
        open.map(|_| AppMessage::None)
    }

    /// 主动关闭窗口
    fn log_window_close(&mut self) -> Task<AppMessage> {
        let Some(id) = self.log_window_id.take() else {
            return Task::none();
        };
        self.log_window_deactivate();
        iced::window::close::<AppMessage>(id)
    }

    /// 原生关闭按钮：确认归属后关窗并清理状态
    fn log_window_close_requested(&mut self, id: iced::window::Id) -> Task<AppMessage> {
        if Some(id) != self.log_window_id {
            return Task::none();
        }
        self.log_window_id = None;
        self.log_window_deactivate();
        iced::window::close::<AppMessage>(id)
    }

    /// 关闭后的公共清理：内存层停止收集并清空缓冲
    fn log_window_deactivate(&mut self) {
        log_window_layer::set_log_window_level(None);
        self.log_window_state.lines.clear();
        self.log_window_state.cursor_seq = 0;
    }

    /// 切换窗口展示等级（只 reload 内存层开关，文件/控制台配置不动）
    fn log_window_level_selected(&mut self, level: LogLevel) -> Task<AppMessage> {
        self.config.set_log_window_level(level);
        self.log_window_state.level_picker_expanded = false;
        log_window_layer::set_log_window_level(Some(level));
        self.request_config_save()
    }

    /// 最大保留行数输入变化：仅更新本地缓冲，不写配置
    ///
    /// 生效时机统一由 [`Self::log_window_max_lines_submitted`]（保存按钮/回车）控制，
    /// 避免删除/输入中间态把半个数字写进配置
    fn log_window_max_lines_changed(&mut self, input: String) -> Task<AppMessage> {
        self.log_window_state.max_lines_input = input;
        Task::none()
    }

    /// 提交最大保留行数：区间外或非数字提示错误，合法才写配置
    ///
    /// 有意不做即时裁剪：调小时当前已展示的超限内容保留，等新日志到达时
    /// 再按上限淘汰；调大也不回填更早的历史，均只影响后续新增的上限
    fn log_window_max_lines_submitted(&mut self) -> Task<AppMessage> {
        let invalid_message = self.i18n.t("log-window.max-lines-invalid");

        let Ok(value) = self.log_window_state.max_lines_input.trim().parse::<u32>() else {
            return self.show_notification(invalid_message, NotificationType::Error);
        };
        if !(LOG_WINDOW_MAX_LINES_MIN..=LOG_WINDOW_MAX_LINES_MAX).contains(&value) {
            return self.show_notification(invalid_message, NotificationType::Error);
        }

        self.config.set_log_window_max_lines(value);
        self.request_config_save()
    }

    /// 定时拉取内存层增量日志；开启自动滚动时滚动到底
    fn log_window_tick(&mut self) -> Task<AppMessage> {
        let new_lines = log_window_layer::read_since(self.log_window_state.cursor_seq);
        let has_new = !new_lines.is_empty();
        if let Some(last) = new_lines.last() {
            self.log_window_state.cursor_seq = last.seq;
        }

        // 追加并裁剪到配置的最大行数
        let max_lines = self.config.global.log_window_max_lines as usize;
        {
            let state = &mut self.log_window_state;
            for line in new_lines {
                state.lines.push_back(line);
            }
            while state.lines.len() > max_lines {
                state.lines.pop_front();
            }
        }

        if has_new && self.log_window_state.auto_scroll {
            self.log_window_scroll_to_bottom()
        } else {
            Task::none()
        }
    }

    /// 滚动日志区域到底部
    fn log_window_scroll_to_bottom(&self) -> Task<AppMessage> {
        operation::scroll_to(
            SCROLL_ID.to_string(),
            scrollable::AbsoluteOffset {
                x: None,
                y: Some(f32::MAX),
            },
        )
    }
}
