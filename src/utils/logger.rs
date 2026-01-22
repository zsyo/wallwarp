use tracing_subscriber::{EnvFilter, fmt as tracing_fmt};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;
use chrono::Datelike;
use chrono::Timelike;

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
pub fn init_logger() {
    // 从环境变量读取日志级别，默认为 info
    let filter = env_filter_extra(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    // 配置日志输出格式
    tracing_fmt()
        .with_env_filter(filter)
        .with_target(false) // 不显示模块路径
        .with_thread_ids(false)
        .with_file(true) // 显示文件名
        .with_line_number(true) // 显示行号
        .with_timer(LocalTimer) // 使用自定义本地时间格式化器
        .init();
}

/// 设置日志级别（运行时动态设置）
///
/// # 参数
/// - `level`: 日志级别字符串 ("off", "error", "warn", "info", "debug", "trace")
pub fn set_log_level(level: &str) {
    let filter = env_filter_extra(EnvFilter::new(level));
    tracing::subscriber::set_global_default(
        tracing_fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_timer(LocalTimer)
            .finish()
    ).expect("Failed to set global default subscriber");
}

fn env_filter_extra(filter: EnvFilter) -> EnvFilter {
    filter
        .add_directive("iced_winit=warn".parse().unwrap())
        .add_directive("iced_wgpu=warn".parse().unwrap())
        .add_directive("wgpu_core=warn".parse().unwrap())
        .add_directive("wgpu_hal=warn".parse().unwrap())
        .add_directive("fontdb=error".parse().unwrap())
}
