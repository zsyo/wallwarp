// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 内存日志层
//!
//! 独立于文件层与控制台的第三条 tracing 通道：把应用自身日志格式化后写入
//! 全局环形缓冲，供"运行日志"展示窗口经增量游标拉取。
//!
//! 与文件层/控制台层完全隔离：
//! - 等级由本模块的 [`SHOWN_LEVEL`] 独立开关控制，不影响两层的目录与文件写入
//! - 窗口未打开时为 off，事件在 `on_event()` 快速短路，正常零开销
//! - 仅收录 target 以 `wallwarp` 开头的事件（第三方库噪声与设置页收敛策略一致）
//! - 按等级/时间戳/消息体结构化存储，UI 侧按等级着色，不做二次解析
//! - 本层为旁挂观察者，不参与 tracing 的全局 callsite 兴趣与 enabled 判定
//!   （否则窗口未打开时会经 `Layer::register_callsite` 的 never 缓存与
//!   `Layered::enabled` 的 AND 语义连带禁掉文件层与控制台层）

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

use chrono::Datelike;
use chrono::Timelike;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

use crate::utils::config::LogLevel;

/// 环形缓冲容量上限（超出丢最旧）
const BUFFER_CAPACITY: usize = 5000;
/// off 档位（窗口未打开）
const LEVEL_OFF: u8 = 0;

/// 一条结构化日志行（等级/时间戳与消息体分开存放，UI 按等级着色）
#[derive(Debug, Clone)]
pub struct LogLine {
    /// 单调递增序号（游标增量拉取的依据）
    pub seq: u64,
    /// 事件等级
    pub level: Level,
    /// 本地时间戳（与文件层格式一致）
    pub time: String,
    /// 消息体（不含时间戳与等级前缀）
    pub message: String,
}

/// 全局日志环形缓冲
struct LogBuffer {
    lines: VecDeque<LogLine>,
    next_seq: u64,
}

/// 内存日志层当前展示等级档位（LEVEL_OFF = 关闭收集）
static SHOWN_LEVEL: AtomicU8 = AtomicU8::new(LEVEL_OFF);

static LOG_BUFFER: OnceLock<Mutex<LogBuffer>> = OnceLock::new();

fn buffer_cell() -> &'static Mutex<LogBuffer> {
    LOG_BUFFER.get_or_init(|| {
        Mutex::new(LogBuffer {
            lines: VecDeque::with_capacity(BUFFER_CAPACITY.min(256)),
            next_seq: 1,
        })
    })
}

/// LogLevel 到内部档位（数值越大越宽松；off 单独表示关闭）
fn level_to_flag(level: LogLevel) -> u8 {
    match level {
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
        LogLevel::Trace => 5,
    }
}

/// tracing 事件等级到内部档位
fn event_level_flag(level: &Level) -> u8 {
    match *level {
        Level::ERROR => 1,
        Level::WARN => 2,
        Level::INFO => 3,
        Level::DEBUG => 4,
        Level::TRACE => 5,
    }
}

/// 设置内存层展示等级；None 表示关闭收集并清空缓冲
///
/// 只作用于内存层：文件层与控制台层的等级与开关均不受影响
pub fn set_log_window_level(level: Option<LogLevel>) {
    let flag = level.map_or(LEVEL_OFF, level_to_flag);
    SHOWN_LEVEL.store(flag, Ordering::Release);
    if flag == LEVEL_OFF {
        clear_buffer();
    }
}

/// 读取序号大于 `seq` 的增量日志行（不重复、不跳过）
///
/// 缓冲被写满后最旧行会被淘汰，`seq` 过于滞后时返回的是仍存活的最新行；
/// UI 游标按返回值的最后一条推进，不出现重复渲染
pub fn read_since(seq: u64) -> Vec<LogLine> {
    let buffer = buffer_cell().lock().unwrap_or_else(|e| e.into_inner());
    buffer
        .lines
        .iter()
        .filter(|line| line.seq > seq)
        .cloned()
        .collect()
}

/// 清空环形缓冲（序号继续递增，旧游标立即失效为增量起点）
pub fn clear_buffer() {
    let mut buffer = buffer_cell().lock().unwrap_or_else(|e| e.into_inner());
    buffer.lines.clear();
}

/// 追加一行到环形缓冲
fn push_line(level: Level, time: String, message: String) {
    let mut buffer = buffer_cell().lock().unwrap_or_else(|e| e.into_inner());
    let seq = buffer.next_seq;
    buffer.next_seq += 1;
    if buffer.lines.len() >= BUFFER_CAPACITY {
        buffer.lines.pop_front();
    }
    buffer.lines.push_back(LogLine {
        seq,
        level,
        time,
        message,
    });
}

/// 本地时间戳行（与文件层格式一致：2026-09-20 12:55:54.984）
fn timestamp() -> String {
    let now = chrono::Local::now();
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_subsec_millis()
    )
}

/// 从事件中提取 message 字段（其余字段忽略，本项目消息体自带上下文前缀）
struct MessageVisitor(String);

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" && self.0.is_empty() {
            self.0 = format!("{:?}", value);
        }
    }
}

/// 内存日志层：事件经预过滤后格式化入环形缓冲
///
/// 本层是**旁挂观察者**，刻意不实现 `enabled()` 与 `register_callsite()`：
/// - `Layer::register_callsite` 的默认实现按 `enabled()` 结果返回
///   `always`/`never`，窗口未打开（`enabled()=false`）会把 callsite 注册为
///   `never`，而 `Layered::pick_interest` 见外层 never 会短路内层，文件层与
///   控制台层连注册机会都没有，该 callsite 被 tracing-core 永久缓存为禁用；
/// - `Layered` 的 `Subscriber::enabled` 是各层 AND，外层否决同样会连带
///   禁掉文件层与控制台层。
///
/// 过滤（开关/target/档位）全部收敛在 `on_event()` 内完成，不影响其它层
pub struct MemoryLayer;

impl<S: Subscriber> Layer<S> for MemoryLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // 旁挂过滤：开关非零 + 仅收录 wallwarp target + 事件等级不高于展示档位
        let shown = SHOWN_LEVEL.load(Ordering::Acquire);
        if shown == LEVEL_OFF {
            return;
        }
        if !event.metadata().target().starts_with("wallwarp") {
            return;
        }
        if event_level_flag(event.metadata().level()) > shown {
            return;
        }

        let mut visitor = MessageVisitor(String::new());
        event.record(&mut visitor);
        if visitor.0.is_empty() {
            return;
        }

        push_line(*event.metadata().level(), timestamp(), visitor.0);
    }

    /// 等级提示恒为 TRACE
    ///
    /// tracing 的全局 callsite 等级取所有层提示的并集，文件层/控制台层的
    /// EnvFilter 会把 current 压到配置档位；内存层等级动态变化又无法触发
    /// callsite 兴趣重建。恒定提示 TRACE 保证任何档位开启时事件都能分发到
    /// 本层，再由 on_event 做档位过滤。各层独立过滤，文件层与控制台层
    /// 的实际输出不受影响
    fn max_level_hint(&self) -> Option<tracing::level_filters::LevelFilter> {
        Some(tracing::level_filters::LevelFilter::TRACE)
    }
}
