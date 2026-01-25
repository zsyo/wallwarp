// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use chrono::Datelike;
use chrono::Timelike;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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

/// 初始化日志系统
///
/// 支持通过环境变量 `RUST_LOG` 控制日志级别：
/// - `RUST_LOG=off` - 完全关闭日志（最低开销）
/// - `RUST_LOG=error` - 只显示错误日志
/// - `RUST_LOG=warn` - 显示警告和错误日志
/// - `RUST_LOG=info` - 显示信息、警告和错误日志（默认）
/// - `RUST_LOG=debug` - 显示调试及以上的所有日志
/// - `RUST_LOG=trace` - 显示所有日志（最详细）
///
/// 也可以针对特定模块设置日志级别，例如：
/// - `RUST_LOG=wallwarp=debug` - 只显示本项目的调试日志
/// - `RUST_LOG=wallwarp=info,reqwest=error` - 项目显示info，reqwest只显示error
///
/// 日志会同时输出到控制台和文件（Logs/latest.log）
/// 程序启动时会自动将旧的 latest.log 重命名为时间戳文件
///
/// # 参数
/// - `enable_logging`: 是否启用日志文件写入，为 false 时只输出到控制台
///
/// # 返回
/// 返回一个 guard，必须在 main 函数中保持它直到程序结束，否则日志文件写入会停止
pub fn init_logger(enable_logging: bool) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    // 从环境变量读取日志级别，默认为 info
    let filter = env_filter_extra(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    let guard = if enable_logging {
        // 获取当前工作目录
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let logs_dir = current_dir.join("logs");

        // 确保 Logs 目录存在
        std::fs::create_dir_all(&logs_dir).unwrap_or_else(|e| {
            eprintln!("[Logger] 无法创建日志目录: {}", e);
        });

        // 日志文件轮转：如果旧的 latest.log 存在，重命名为时间戳命名的日志文件
        let latest_log_path = logs_dir.join("latest.log");
        if latest_log_path.exists() {
            let now = chrono::Local::now();
            let timestamp = format!(
                "{:04}-{:02}-{:02}_{:02}_{:02}_{:02}",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            );
            let archived_log_path = logs_dir.join(format!("{}.log", timestamp));

            // 尝试重命名旧的日志文件
            if let Err(e) = std::fs::rename(&latest_log_path, &archived_log_path) {
                eprintln!("[Logger] 重命名旧日志文件失败: {}", e);
            }
        }

        // 配置日志文件输出
        let file_appender = tracing_appender::rolling::never(&logs_dir, "latest.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        // 配置控制台日志输出格式
        let console_layer = fmt::layer()
            .with_target(false) // 不显示模块路径
            .with_thread_ids(false)
            .with_file(true) // 显示文件名
            .with_line_number(true) // 显示行号
            .with_timer(LocalTimer) // 使用自定义本地时间格式化器
            .with_filter(filter.clone());

        // 配置文件日志输出格式
        let file_layer = fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(LocalTimer)
            .with_writer(non_blocking)
            .with_ansi(false) // 禁用 ANSI 颜色代码，避免文件中出现乱码
            .with_filter(filter);

        // 同时启用控制台和文件日志
        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .init();

        Some(guard)
    } else {
        // 只启用控制台日志
        let console_layer = fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(LocalTimer)
            .with_filter(filter);

        tracing_subscriber::registry().with(console_layer).init();

        None
    };

    guard
}

/// 设置日志级别（运行时动态设置）
///
/// # 参数
/// - `level`: 日志级别字符串 ("off", "error", "warn", "info", "debug", "trace")
pub fn set_log_level(level: &str) {
    let filter = env_filter_extra(EnvFilter::new(level));
    tracing::subscriber::set_global_default(
        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(LocalTimer)
            .finish(),
    )
    .expect("Failed to set global default subscriber");
}

fn env_filter_extra(filter: EnvFilter) -> EnvFilter {
    filter
        .add_directive("iced_winit=warn".parse().unwrap())
        .add_directive("iced_wgpu=warn".parse().unwrap())
        .add_directive("wgpu_core=warn".parse().unwrap())
        .add_directive("wgpu_hal=warn".parse().unwrap())
        .add_directive("fontdb=error".parse().unwrap())
}
