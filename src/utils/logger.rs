// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use chrono::Datelike;
use chrono::Timelike;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::writer::MakeWriter;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::Layered, layer::SubscriberExt, reload,
    util::SubscriberInitExt,
};

use crate::utils::config::LogLevel;

/// 文件输出层的类型擦除别名（层本体固定挂载，开关通过写入器动态切换）
type FileLayer = Box<dyn Layer<Registry> + Send + Sync>;

/// 文件层挂载后的组合 subscriber 类型（console 层的 inner subscriber）
type FileSubscriber = Layered<FileLayer, Registry>;

/// 控制台层日志等级过滤器句柄，设置页实时调整等级时 reload
static CONSOLE_FILTER: OnceLock<reload::Handle<EnvFilter, FileSubscriber>> = OnceLock::new();
/// 文件层日志等级过滤器句柄，设置页实时调整等级时 reload
static FILE_FILTER: OnceLock<reload::Handle<EnvFilter, Registry>> = OnceLock::new();
/// 文件异步写入器：Some 表示运行日志开启，None 表示关闭（写入被跳过）
static FILE_WRITER: OnceLock<Mutex<Option<NonBlocking>>> = OnceLock::new();
/// 文件写线程凭证；重新开启文件日志时替换，进程退出前 drop 以落盘缓冲
static FILE_GUARD: OnceLock<Mutex<Option<WorkerGuard>>> = OnceLock::new();
/// 文件日志当前是否处于开启状态
static FILE_ENABLED: AtomicBool = AtomicBool::new(false);

/// 自定义时间格式化器
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, writer: &mut Writer<'_>) -> std::fmt::Result {
        // 使用本地时间，格式为 2026-01-22 21:33:40.495
        let now = chrono::Local::now();
        write!(
            writer,
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
}

/// 按开关状态分发写入的 MakeWriter：运行日志关闭时丢弃写入内容
struct ToggleableWriter;

impl<'a> MakeWriter<'a> for ToggleableWriter {
    type Writer = FileWriter;

    fn make_writer(&'a self) -> Self::Writer {
        let inner = file_writer_cell()
            .lock()
            .unwrap()
            .as_ref()
            .map(|writer| writer.make_writer());
        FileWriter(inner)
    }
}

/// 封装文件写入器；开关关闭时为 None，写入被跳过
struct FileWriter(Option<NonBlocking>);

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match &mut self.0 {
            Some(writer) => writer.write(buf),
            None => Ok(buf.len()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.0 {
            Some(writer) => writer.flush(),
            None => Ok(()),
        }
    }
}

/// 初始化日志系统
///
/// 初始日志级别优先取环境变量 `RUST_LOG`（面向开发者调试），否则使用配置档位：
/// - `RUST_LOG=off/error/warn/info/debug/trace`
/// - 也可按模块设置，如 `RUST_LOG=wallwarp=debug,reqwest=error`
///
/// 日志始终输出到控制台；`enable_logging` 为 true 时额外写入 logs/latest.log，
/// 并自动将旧的 latest.log 重命名为时间戳文件完成轮转。
///
/// 运行时可通过 [`update_log_config`] 实时调整日志等级与文件层开关，无需重启；
/// 进程退出前应调用 [`flush`] 落盘文件日志缓冲。
pub fn init_logger(enable_logging: bool, log_level: LogLevel) {
    // 文件层：固定挂载，写入行为由 FILE_WRITER 是否装入决定（实时开关）
    let (file_reload, file_handle) = reload::Layer::new(level_filter(log_level));
    let file_layer: FileLayer = Box::new(
        fmt::layer()
            .with_target(false) // 不显示模块路径
            .with_thread_ids(false)
            .with_file(true) // 显示文件名
            .with_line_number(true) // 显示行号
            .with_timer(LocalTimer) // 使用自定义本地时间格式化器
            .with_writer(ToggleableWriter)
            .with_ansi(false) // 禁用 ANSI 颜色代码，避免文件中出现乱码
            .with_filter(file_reload),
    );

    // 控制台层
    let (console_reload, console_handle) = reload::Layer::new(startup_filter(log_level));
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTimer)
        .with_filter(console_reload);

    // 同时启用控制台和文件日志
    // 注意：文件层必须先挂载（直接位于 Registry 上），使其 reload 句柄类型保持简单；
    // 层的先后顺序不影响过滤行为（每层独立过滤）
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    if enable_logging {
        let (writer, guard) = build_file_writer();
        *file_writer_cell().lock().unwrap() = Some(writer);
        *file_guard_cell().lock().unwrap() = Some(guard);
    }
    FILE_ENABLED.store(enable_logging, Ordering::Release);

    if CONSOLE_FILTER.set(console_handle).is_err() || FILE_FILTER.set(file_handle).is_err() {
        panic!("logger 只能初始化一次");
    }
}

/// 运行时更新日志配置（设置页入口），立即生效无需重启
///
/// - `log_level`：同时作用于控制台层与文件层
/// - `enable_logging`：实时开启/关闭文件日志；由关闭转为开启时自动轮转旧日志文件
pub fn update_log_config(enable_logging: bool, log_level: LogLevel) {
    if let Some(handle) = CONSOLE_FILTER.get() {
        handle
            .reload(level_filter(log_level))
            .expect("console 日志过滤器 reload 失败");
    }
    if let Some(handle) = FILE_FILTER.get() {
        handle
            .reload(level_filter(log_level))
            .expect("file 日志过滤器 reload 失败");
    }

    // 仅在开关状态变化时重建/卸载写入器，调整等级不影响当前日志文件
    if FILE_ENABLED.swap(enable_logging, Ordering::Release) == enable_logging {
        return;
    }

    if enable_logging {
        // 先停掉旧写线程关闭文件句柄，否则 Windows 下轮转重命名会因文件被占用而失败
        *file_guard_cell().lock().unwrap() = None;
        let (writer, guard) = build_file_writer();
        *file_writer_cell().lock().unwrap() = Some(writer);
        *file_guard_cell().lock().unwrap() = Some(guard);
    } else {
        // 先卸下写入器停止写入，再 drop 写线程凭证落盘剩余缓冲
        *file_writer_cell().lock().unwrap() = None;
        *file_guard_cell().lock().unwrap() = None;
    }
}

/// 进程退出前调用：落盘文件日志缓冲并停止写线程
pub fn flush() {
    if let Some(cell) = FILE_GUARD.get() {
        *cell.lock().unwrap() = None;
    }
}

fn file_writer_cell() -> &'static Mutex<Option<NonBlocking>> {
    FILE_WRITER.get_or_init(|| Mutex::new(None))
}

fn file_guard_cell() -> &'static Mutex<Option<WorkerGuard>> {
    FILE_GUARD.get_or_init(|| Mutex::new(None))
}

/// 构建文件写入器：轮转旧日志文件、创建异步写入器与写线程凭证
fn build_file_writer() -> (NonBlocking, WorkerGuard) {
    let logs_dir = std::env::current_dir().unwrap_or_default().join("logs");
    std::fs::create_dir_all(&logs_dir).ok();
    let latest_log_path = logs_dir.join("latest.log");

    // 日志文件轮转：如果旧的 latest.log 存在，重命名为时间戳命名的日志文件
    if latest_log_path.exists() {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let archived_path = logs_dir.join(format!("{}.log", timestamp));

        // 尝试重命名旧的日志文件
        if let Err(e) = std::fs::rename(&latest_log_path, &archived_path) {
            eprintln!("[Logger] 重命名旧日志文件失败: {}", e);
        }
    }

    let file_appender = tracing_appender::rolling::never(&logs_dir, "latest.log");
    tracing_appender::non_blocking(file_appender)
}

/// 启动期过滤器：RUST_LOG 环境变量优先，否则使用配置档位
fn startup_filter(level: LogLevel) -> EnvFilter {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()));
    env_filter_extra(filter)
}

/// 运行期过滤器：直接使用指定档位（UI 明确选择后以 UI 为准）
fn level_filter(level: LogLevel) -> EnvFilter {
    env_filter_extra(EnvFilter::new(level.as_str()))
}

fn env_filter_extra(filter: EnvFilter) -> EnvFilter {
    filter
        .add_directive("iced_winit=warn".parse().unwrap())
        .add_directive("iced_wgpu=warn".parse().unwrap())
        .add_directive("wgpu_core=warn".parse().unwrap())
        .add_directive("wgpu_hal=warn".parse().unwrap())
        .add_directive("fontdb=error".parse().unwrap())
}
