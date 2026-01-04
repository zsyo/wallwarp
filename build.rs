fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        // 这里的路径是相对于项目根目录的
        res.set_icon("assets/logo.ico");
        res.compile().unwrap();
    }
}
