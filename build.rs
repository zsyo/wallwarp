fn main() {
    // 显示版本：CI 构建时经 WALLWARP_DISPLAY_VERSION 注入 tag 版本（可含
    // 预发布段，如 1.5.1_beta.1）；Cargo.toml 恒为干净版本，本地开发构建
    // 未注入时回退 CARGO_PKG_VERSION。业务代码一律读 WALLWARP_VERSION。
    let display_version = std::env::var("WALLWARP_DISPLAY_VERSION")
        .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string());
    println!("cargo:rustc-env=WALLWARP_VERSION={display_version}");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        // 这里的路径是相对于项目根目录的
        res.set_icon("assets/logo.ico");
        // 设置名称
        res.set("ProductName", "WallWarp");
        // 版权信息
        res.set("LegalCopyright", "Copyright © 2026 zsyo");
        // 版本信息（数字版本恒取干净版本：tag 预发布段无法映射为 16 位整数）
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
