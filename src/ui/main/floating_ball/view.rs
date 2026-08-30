// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 悬浮球视图：48px 圆形容器 + 居中 logo，窗口背景透明
//!
//! 贴边形态：窗口紧贴屏幕左/右边缘且尺寸不变，用 `Float` 把完整的大球
//! （含 32px 大 logo）向边缘平移一半——超出窗口表面的部分被渲染图层
//! 裁剪，屏幕上呈现的正是“大球探出一半”的半圆图案（含半个大 logo），
//! 且完全留在当前屏幕内，多屏接缝处不会出现在另一块屏上

use super::{BALL_ICON_SIZE, BALL_SIZE};
use crate::ui::main::MainMessage;
use crate::ui::main::SnapEdge;
use crate::ui::{App, AppMessage};
use iced::border::{Border, Radius};
use iced::widget::{container, float::Float, image, mouse_area};
use iced::{Alignment, Color, Element, Length, Vector};

/// 渲染悬浮球窗口内容
pub fn floating_ball_view(app: &App) -> Element<'_, AppMessage> {
    let theme_colors = app.theme_colors;
    let hovered = app.floating_ball_state.is_hovered();
    let snap_edge = app.floating_ball_state.snap().edge;

    // 悬停时提升背景不透明度并加描边，形成视觉反馈
    let bg = if hovered {
        iced::Color {
            a: 0.92,
            ..theme_colors.background
        }
    } else {
        iced::Color {
            a: 0.78,
            ..theme_colors.background
        }
    };
    let border_color = if hovered {
        theme_colors.tooltip_border_color
    } else {
        Color::TRANSPARENT
    };

    let content: Element<'_, AppMessage> = match snap_edge {
        Some(edge) if !hovered => {
            // 完整大球向贴边平移一半：窗口外部分被渲染图层裁剪，
            // 呈现“大球 + 大 logo 探出一半”的半圆贴边效果
            let dx = match edge {
                SnapEdge::Left => -BALL_SIZE / 2.0,
                SnapEdge::Right => BALL_SIZE / 2.0,
            };
            let ball: Element<'_, AppMessage> = full_ball_view(app, bg, border_color).into();
            let floating = Float::new(ball).translate(move |_, _| Vector::new(dx, 0.0));
            container(floating)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        _ => full_ball_view(app, bg, border_color).into(),
    };

    mouse_area(content)
        .on_press(MainMessage::FloatingBallPressed.into())
        .on_move(|pos| MainMessage::FloatingBallCursorMoved(pos).into())
        .on_release(MainMessage::FloatingBallReleased.into())
        // 右键同样弹出菜单（右键不支持拖动，释放即触发）
        .on_right_release(MainMessage::FloatingBallRightReleased.into())
        .on_enter(MainMessage::FloatingBallHovered(true).into())
        .on_exit(MainMessage::FloatingBallHovered(false).into())
        .into()
}

/// 展开形态：完整的 48px 圆球 + 居中 logo
fn full_ball_view<'a>(
    app: &'a App,
    bg: Color,
    border_color: Color,
) -> impl Into<Element<'a, AppMessage>> {
    container(
        image(app.logo_handle.clone())
            .width(Length::Fixed(BALL_ICON_SIZE))
            .height(Length::Fixed(BALL_ICON_SIZE)),
    )
    .width(Length::Fixed(BALL_SIZE))
    .height(Length::Fixed(BALL_SIZE))
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(bg)),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: Radius::from(BALL_SIZE / 2.0),
        },
        ..Default::default()
    })
}
