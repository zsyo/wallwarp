// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 悬浮球交互处理：按下/移动/释放、菜单弹出、位置持久化、开关窗口、
//! 空闲贴边半圆与悬停展开
//!
//! 坐标体系说明：贴边相关的几何采集与窗口移动都经 `crate::platform`
//! 在 window::run 闭包内以**原生全屏坐标**完成（Windows/Linux 物理
//! 像素左上原点、macOS 点坐标左下原点，平台内自洽）。
//! 仅支持左右贴边：贴边时窗口紧贴屏幕边缘且尺寸不变，
//! 视图把球向边缘偏移一半（窗口外部分被渲染表面裁剪）呈现半圆图案，
//! 因此多屏接缝处不会有任何部分出现在另一块屏上。

use crate::platform::{self, WindowAnchor};
use crate::ui::main::MainMessage;
use crate::ui::main::floating_ball;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;
use tracing::info;

impl App {
    /// 悬浮球鼠标按下：重置交互状态
    pub(crate) fn floating_ball_pressed(&mut self) -> Task<AppMessage> {
        self.floating_ball_state.press();
        Task::none()
    }

    /// 悬浮球内光标移动：超过拖动阈值时发起系统级窗口拖动
    pub(crate) fn floating_ball_cursor_moved(&mut self, pos: iced::Point) -> Task<AppMessage> {
        if self.floating_ball_state.cursor_moved(pos) {
            self.floating_ball_state.set_dragging();
            // 进入自由拖动，脱离贴边状态（视图恢复完整圆形）
            self.floating_ball_state.clear_snap();
            if let Some(ball_id) = self.floating_ball_id {
                return window::drag::<AppMessage>(ball_id);
            }
        }
        Task::none()
    }

    /// 悬浮球鼠标释放：未拖动则视为点击，弹出菜单
    pub(crate) fn floating_ball_released(&mut self) -> Task<AppMessage> {
        let clicked = self.floating_ball_state.release();
        if clicked {
            return self.show_floating_ball_menu();
        }
        Task::none()
    }

    /// 悬浮球右键释放：弹出菜单（右键不参与拖动，无需按压检测）
    pub(crate) fn floating_ball_right_released(&mut self) -> Task<AppMessage> {
        if self.floating_ball_id.is_some() {
            return self.show_floating_ball_menu();
        }
        Task::none()
    }

    /// 悬浮球悬停状态变化：仅影响视图形态（悬停整圆/贴边半圆）；
    /// 鼠标离开后延迟调度贴边（拖动后的自由位置需要吸附）
    pub(crate) fn floating_ball_hovered(&mut self, hovered: bool) -> Task<AppMessage> {
        self.floating_ball_state.set_hovered(hovered);
        if hovered {
            // 重新进入球体：解除菜单打开守卫（Linux 非阻塞弹出的复位路径之一）
            self.floating_ball_state.set_menu_open(false);
        }
        if !hovered
            && self.floating_ball_id.is_some()
            && !self.floating_ball_state.is_menu_open()
            && !self.floating_ball_state.is_pressed()
        {
            // 鼠标离开：延迟 200ms 贴边，期间重新进入会因 hovered 检查而跳过
            return Task::perform(
                tokio::time::sleep(tokio::time::Duration::from_millis(200)),
                |_| MainMessage::FloatingBallSnapToEdge.into(),
            );
        }
        Task::none()
    }

    /// 贴边调度：吸附到窗口中心最近的左右屏幕边缘并呈半圆形态
    ///
    /// 几何采集与窗口移动在 window::run 闭包内经 platform 抽象完成，
    /// 吸附上下文经消息回传后存入状态
    pub(crate) fn floating_ball_snap_to_edge(&mut self) -> Task<AppMessage> {
        let Some(ball_id) = self.floating_ball_id else {
            return Task::none();
        };
        // 前置检查（调度与执行在同一更新周期内，无竞态窗口）
        if self.floating_ball_state.is_hovered()
            || self.floating_ball_state.is_menu_open()
            || self.floating_ball_state.is_pressed()
        {
            return Task::none();
        }
        Self::tuck_ball(ball_id)
    }

    /// 贴边：窗口紧贴最近的左/右屏幕边缘（尺寸不变），视图呈半圆
    fn tuck_ball(ball_id: window::Id) -> Task<AppMessage> {
        window::run::<floating_ball::SnapState>(ball_id, move |mw| {
            let mut result = floating_ball::SnapState::default();
            let Some(geom) = platform::window_geometry(mw) else {
                return result;
            };
            let Some(work) = platform::work_area(mw) else {
                return result;
            };
            let size = geom.size;
            // 垂直方向保持在屏幕内
            let clamp_y = geom.y.clamp(work.y, work.y + work.height - size);

            // 以窗口中心判断贴近左还是右边缘
            use floating_ball::SnapEdge;
            let center_x = geom.x + size / 2.0;
            let edge = if center_x - work.x <= work.x + work.width - center_x {
                SnapEdge::Left
            } else {
                SnapEdge::Right
            };

            let tx = match edge {
                SnapEdge::Left => work.x,
                SnapEdge::Right => work.x + work.width - size,
            };
            info!(
                "[悬浮球] [贴边] 吸附至 {:?}, 目标位置 ({}, {})",
                edge, tx, clamp_y
            );
            platform::move_window_to(mw, tx, clamp_y);
            result.edge = Some(edge);
            result.work = work;
            result
        })
        .map(|snap| MainMessage::FloatingBallSnapped(snap).into())
    }

    /// 悬浮球鼠标释放：未拖动则视为点击，弹出菜单
    pub(crate) fn show_floating_ball_menu(&mut self) -> Task<AppMessage> {
        let Some(ball_id) = self.floating_ball_id else {
            return Task::none();
        };

        // Menu 内部非 Send，无法移入 window::run 闭包，
        // 因此这里仅提取可跨线程传递的窗口锚点，弹出动作在
        // 主线程消息处理器中完成
        window::run::<WindowAnchor>(ball_id, platform::window_anchor)
            .map(|anchor| MainMessage::FloatingBallMenuReady(anchor).into())
    }

    /// 收到球窗口锚点：弹出菜单（主线程）
    ///
    /// Windows/macOS：muda 弹出为阻塞调用，返回即代表菜单已关闭；
    /// Linux：GTK 线程非阻塞弹出，用"菜单打开守卫 + 延迟复位"近似阻塞语义
    pub(crate) fn floating_ball_menu_ready(&mut self, anchor: WindowAnchor) -> Task<AppMessage> {
        if anchor == WindowAnchor::Unsupported {
            tracing::warn!("[悬浮球] [菜单] 无法获取窗口锚点，弹出菜单失败");
            return Task::none();
        }

        #[cfg(not(target_os = "linux"))]
        {
            // 菜单打开期间（阻塞式弹出）禁止贴边隐藏
            self.floating_ball_state.set_menu_open(true);
            let shown = self.floating_ball.show_popup_at(anchor);
            self.floating_ball_state.set_menu_open(false);
            if !shown {
                tracing::warn!("[悬浮球] [菜单] 弹出菜单失败");
            }
            // 菜单期间鼠标可能已移出球且 on_exit 已被消费，关闭后需补充贴边调度
            if !self.floating_ball_state.is_hovered() {
                return Task::perform(
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)),
                    |_| MainMessage::FloatingBallSnapToEdge.into(),
                );
            }
            Task::none()
        }
        #[cfg(target_os = "linux")]
        {
            let _ = self.floating_ball.show_popup_at(anchor);
            // 复位延迟须大于菜单典型交互时长；期间鼠标重新进入球体也会解除守卫
            Task::perform(
                tokio::time::sleep(tokio::time::Duration::from_millis(2500)),
                |_| MainMessage::FloatingBallMenuClosed.into(),
            )
        }
    }

    /// 菜单打开守卫解除：恢复贴边调度（Linux 非阻塞弹出路径）
    pub(crate) fn floating_ball_menu_closed(&mut self) -> Task<AppMessage> {
        self.floating_ball_state.set_menu_open(false);
        if !self.floating_ball_state.is_hovered() {
            return Task::perform(
                tokio::time::sleep(tokio::time::Duration::from_millis(200)),
                |_| MainMessage::FloatingBallSnapToEdge.into(),
            );
        }
        Task::none()
    }

    /// 贴边吸附上下文回存
    pub(crate) fn floating_ball_snapped(
        &mut self,
        snap: floating_ball::SnapState,
    ) -> Task<AppMessage> {
        if snap.edge.is_some() {
            self.floating_ball_state.set_snap(snap);
        }
        Task::none()
    }

    /// 窗口位置改变：主窗口记忆位置（防抖持久化）；悬浮球窗口更新配置并防抖持久化
    pub(crate) fn window_moved(&mut self, id: window::Id, pos: iced::Point) -> Task<AppMessage> {
        // 主窗口：位置记忆（最大化状态下 Windows 会报出还原矩形之外的位置，跳过）
        if id == self.main_window_id {
            if self.main_state.is_maximized {
                return Task::none();
            }
            self.config.display.x = pos.x as i32;
            self.config.display.y = pos.y as i32;
            return self.request_config_save();
        }

        if Some(id) != self.floating_ball_id {
            return Task::none();
        }

        self.config.global.floating_ball_x = pos.x as i32;
        self.config.global.floating_ball_y = pos.y as i32;

        // 防抖：拖动过程中 Moved 事件密集，500ms 内合并为一次写盘
        if !self.floating_ball_save_pending {
            self.floating_ball_save_pending = true;
            return Task::perform(
                tokio::time::sleep(tokio::time::Duration::from_millis(500)),
                |_| MainMessage::FloatingBallSavePosition.into(),
            );
        }
        Task::none()
    }

    /// 防抖到期：把悬浮球位置写入配置文件
    pub(crate) fn floating_ball_save_position(&mut self) -> Task<AppMessage> {
        self.floating_ball_save_pending = false;
        let (x, y) = (
            self.config.global.floating_ball_x,
            self.config.global.floating_ball_y,
        );
        info!("[悬浮球] [位置] 持久化: ({}, {})", x, y);
        self.config.save_to_file();
        Task::none()
    }

    /// 打开悬浮球窗口并记录 Id
    pub(crate) fn open_floating_ball_window(&mut self) -> Task<AppMessage> {
        if self.floating_ball_id.is_some() {
            return Task::none();
        }
        if !platform::supports_floating_ball() {
            tracing::warn!("[悬浮球] [窗口] 当前平台/会话不支持悬浮球（Wayland 下窗口定位受限）");
            return Task::none();
        }
        let (ball_id, task) = window::open(floating_ball::window_settings(&self.config.global));
        self.floating_ball_id = Some(ball_id);
        info!("[悬浮球] [窗口] 已打开: {:?}", ball_id);
        Task::batch(vec![
            task.map(|_| AppMessage::None),
            Self::disable_ball_dwm_frame(ball_id),
            // 打开后鼠标不在球上，自动贴边呈半圆
            Task::perform(
                tokio::time::sleep(tokio::time::Duration::from_millis(400)),
                |_| MainMessage::FloatingBallSnapToEdge.into(),
            ),
        ])
    }

    /// 移除悬浮球窗口的系统边框（仅 Windows 有此修饰问题）
    pub(crate) fn disable_ball_dwm_frame(ball_id: window::Id) -> Task<AppMessage> {
        window::run(ball_id, |mw| platform::remove_dwm_frame(mw)).map(|_| AppMessage::None)
    }

    /// 关闭悬浮球窗口并清空 Id
    pub(crate) fn close_floating_ball_window(&mut self) -> Task<AppMessage> {
        if let Some(ball_id) = self.floating_ball_id.take() {
            info!("[悬浮球] [窗口] 已关闭: {:?}", ball_id);
            return window::close::<AppMessage>(ball_id);
        }
        Task::none()
    }

    /// 菜单"关闭悬浮球"：关窗口并同步到配置
    pub(crate) fn floating_ball_close(&mut self) -> Task<AppMessage> {
        self.config.global.show_floating_ball = false;
        self.config.save_to_file();
        self.close_floating_ball_window()
    }
}
