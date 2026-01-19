use tracing_subscriber::{EnvFilter, fmt};

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
    fmt()
        .with_env_filter(filter)
        .with_target(true) // 显示模块路径
        .with_thread_ids(false)
        .with_file(true) // 显示文件名
        .with_line_number(true) // 显示行号
        .compact() // 使用紧凑格式，减少输出
        .init();
}

/// 设置日志级别（运行时动态设置）
///
/// # 参数
/// - `level`: 日志级别字符串 ("off", "error", "warn", "info", "debug", "trace")
pub fn set_log_level(level: &str) {
    let filter = env_filter_extra(EnvFilter::new(level));
    tracing::subscriber::set_global_default(fmt().with_env_filter(filter).compact().finish()).expect("Failed to set global default subscriber");
}

fn env_filter_extra(filter: EnvFilter) -> EnvFilter {
    filter
        .add_directive("iced_winit=warn".parse().unwrap())
        .add_directive("iced_wgpu=warn".parse().unwrap())
        .add_directive("wgpu_core=warn".parse().unwrap())
        .add_directive("wgpu_hal=warn".parse().unwrap())
        .add_directive("fontdb=error".parse().unwrap())
}
