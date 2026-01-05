fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        // 这里的路径是相对于项目根目录的
        res.set_icon("assets/logo.ico");
        // 设置名称
        res.set("ProductName", "WallWarp");
        // 版权信息
        res.set("LegalCopyright", "Copyright © 2026 zsyo");
        // 版本信息
        let version_str = env!("CARGO_PKG_VERSION");
        let version_u64 = parse_version(version_str);
        res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, version_u64);
        res.set_version_info(winresource::VersionInfo::FILEVERSION, version_u64);
        res.set("FileVersion", version_str);
        res.set("ProductVersion", version_str);

        res.compile().unwrap();
    }
}

fn parse_version(version: &str) -> u64 {
    let mut parts = version.split('.').map(|s| s.parse::<u64>().unwrap_or(0));

    let major = parts.next().unwrap_or(0);
    let minor = parts.next().unwrap_or(0);
    let patch = parts.next().unwrap_or(0);
    let build = parts.next().unwrap_or(0); // 如果只有三位,最后一位补0

    // windows 版本号: 每个版本占16位
    (major << 48) | (minor << 32) | (patch << 16) | build
}
