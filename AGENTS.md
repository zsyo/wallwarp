# WallWarp 项目记忆文件

## 项目概述
- 项目名称: WallWarp
- 项目类型: 桌面壁纸管理软件
- 开发语言: Rust
- GUI框架: Iced (版本 0.14.0)
- 支持平台: Windows 10+ / macOS 10.15+ / Linux（X11），各平台均提供 x64 与 arm64 构建
- 项目目的: 创建一个功能齐全的壁纸管理应用程序

## 项目结构
- src/main.rs - 应用入口点（iced::daemon 多窗口运行时）
- src/lib.rs - 库入口点，声明所有模块
- src/platform/ - 平台抽象层（三平台实现按 target_os 编译期选择）
  - mod.rs - 公共接口：窗口几何/工作区查询（原生坐标）、菜单锚点 WindowAnchor、
    system_color_mode、supports_floating_ball；`use xxx as imp` 按平台分发
  - menu.rs - 托盘/悬浮球菜单跨平台封装（Menu/TrayIconHandle 句柄，
    Windows/macOS 主线程直连 muda；Linux 走 menu_linux.rs 的 GTK 线程）
  - menu_linux.rs - Linux 菜单运行时：muda 对象为 Rc（非 Send）且 appindicator
    托盘需 GTK 事件循环，故托盘与全部菜单在专用 GTK 线程创建，主线程经
    mpsc 命令通道操控（BuildMenu/AttachTray/SetText/SetEnabled/PopupBall）
  - windows.rs - Win32 实现（MonitorFromWindow/GetMonitorInfoW/SetWindowPos/DWM）
  - macos.rs - AppKit 实现（objc2：NSScreen visibleFrame / NSWindow setFrameOrigin）
  - linux.rs - X11 实现（x11rb：RandR 显示器 + _NET_WORKAREA 求交集；
    Wayland 会话 supports_floating_ball()=false）
- src/ui/ - 用户界面组件
  - common.rs - UI公共方法和常量（按钮创建、模态对话框、容器样式等）
  - message.rs - 定义UI消息类型
  - view.rs - 定义界面渲染逻辑（按 window::Id 分发主窗口/悬浮球视图）
  - style.rs - 定义界面样式
  - main/floating_ball/ - 桌面悬浮球（菜单管理 + 视图 + 窗口参数）
- src/services/ - 业务逻辑服务
  - mod.rs - 服务模块声明
- src/utils/ - 工具函数
  - helpers.rs - 辅助函数（app_root_dir 为各平台数据根目录）
  - mod.rs - 工具模块声明
- src/i18n/ - 国际化支持模块（mod.rs 加载、translate.rs 翻译）
- locales/ - 语言配置文件目录
  - en.ftl - 英语翻译文件
  - zh-cn.ftl - 中文翻译文件

## 多窗口（daemon）规范
- 应用使用 `iced::daemon` 运行时，主窗口与悬浮球窗口均为 iced 窗口
- **主窗口 Id 固定为 `App.main_window_id`**：所有窗口操作必须使用它，
  禁止使用 `window::oldest()`（多窗口下不再可靠）
- 窗口事件（Resized/CloseRequested/Focused/Moved）由 subscription 携带 window::Id
  上报，处理器必须先按 `main_window_id` / `floating_ball_id` 过滤再执行逻辑
- 悬浮球窗口透明背景通过专用 Theme 实现（palette.background = TRANSPARENT，
  在 `.style` 闭包中按 palette 识别），不是按窗口区分 style 闭包
- 悬浮球菜单复用 muda（`tray_icon::menu`）全局 MenuEvent 通道，菜单 id 以
  `ball_` 为前缀；弹出经 `platform::menu`：window::run 提取 WindowAnchor →
  消息回传 → update 中 popup_at（Windows/macOS 阻塞弹出；Linux 非阻塞，
  以 menu_open 守卫 + 延迟 `FloatingBallMenuClosed` 复位近似阻塞语义，
  悬停重新进入也会解除守卫）
- 悬浮球左键/右键均可弹出操作菜单（右键释放即触发，不参与拖动）
- 悬浮球空闲时自动贴边呈半圆（仅支持左右贴边）：窗口紧贴屏幕边缘且尺寸
  不变，视图用 `iced::widget::Float` 把完整大球（含大 logo）向边缘平移一半
  （Float overlay 的 with_layer 按窗口矩形裁剪），呈现"大球探出一半"图案且
  完全留在当前屏幕内。悬停时平移归零显示整圆（无需移动窗口）。
  注意：不能用 padding 平移——iced 的 padding.fit 会把推出容器边界的
  正向 padding 钳回，导致单侧贴边失效。吸附（方向+工作区）存于 SnapState。
  窗口移动在 window::run 闭包内经 `platform::window_geometry/work_area/
  move_window_to` 以**原生全屏坐标**完成（Windows/Linux 物理像素左上原点、
  macOS 点坐标左下原点，平台内自洽），不用逻辑坐标——多屏不同 DPI 下
  换算有歧义

## 跨平台规范
- **平台代码分文件**：所有 OS 专属调用必须收敛到 `src/platform/`（win/macos/linux
  各一文件 + menu.rs），UI/handler 层禁止直接 `use windows::` / objc / x11rb；
  新增平台能力时先在 platform/mod.rs 定义公共签名，再补三平台实现
- **编译隔离**：平台专属依赖必须放 `[target.'cfg(...)'.dependencies]`
  （windows/winreg、objc2 系、x11rb/gtk）；tray-icon 为全平台依赖（Linux 需
  `features = ["gtk"]` 启用 libappindicator 后端）
- **数据目录**：`helpers::app_root_dir()` 决定 config/data/cache/db/logs 根——
  Windows 为 exe 同级（便携式），macOS 为 `~/Library/Application Support/WallWarp`，
  Linux 为 `~/.config/wallwarp`；启动时 set_current_dir 到该根目录，
  其余代码继续使用相对路径
- **locales 资源**：`i18n::resolve_locales_dir()` 按候选序解析（exe 同级 →
  `../Resources/locales`（mac bundle）→ `../locales` → CWD），支持运行时热加载
- **开机自启动**：`src/utils/startup/` 按平台拆分——Windows 注册表 Run 键、
  macOS LaunchAgent plist（`top.aico.wallwarp`）、Linux XDG autostart desktop；
  Linux AppImage 场景必须用 `APPIMAGE` 环境变量取真实路径（挂载点易失）
- **剪贴板**：使用 arboard（原生实现），禁止用 cmd/clip、xclip 等外部命令
- **窗口显隐**：托盘隐藏/恢复用 iced API（`set_mode(Mode::Hidden/Windowed)` +
  `minimize(false)` + `gain_focus`），跨平台且保留最大化状态；
  Windows 无边框缩放经 platform::enable_resize_border 注入 WS_THICKFRAME，
  macOS 用原生 fullsize_content_view（红绿灯叠加，自绘标题栏左侧预留 78px），
  Linux 复用自绘边缘感应层 + `window::drag_resize`
- **已知平台差异**：macOS 壁纸铺满方式由系统决定（wallpaper crate set_mode 为空）；
  Wayland 不显示悬浮球；Linux 托盘无双击事件（appindicator 限制）；
  dmg 默认未签名（Gatekeeper 需右键打开）
- **网络请求**：reqwest 保持 `native-tls`（走系统 TLS 栈与证书库，SOCKS5/自签 CA
  等代理场景兼容性最好）+ `system-proxy`（未显式代理时自动读取系统代理设置）。
  教训：编辑 Cargo.toml 触发依赖重解析时锁版本可能被降级
  （reqwest 0.13.4 → 0.13.1 曾导致 SOCKS5 代理全部失效），升级后必须核对
  Cargo.lock 中关键包版本是否与改动前一致
- **x11rb**（platform/linux.rs）：依赖必须带 `features = ["randr"]`（默认特性
  不含任何协议扩展）；trait 方法返回 `Result<Cookie>`，惯用链式
  `.ok()?.reply().ok()?`；`GetPropertyReply::value32()` 返回 `Option`；
  移动窗口用 `configure_window` + `ConfigureWindowAux::new().x().y()`
  （X11 无独立 MoveWindow 请求）；RandR `Monitor` 的 x/y 为 i16、
  width/height 为 u16，混算必须分别 `as f32`。
  版本保持 0.13：arboard 与 winit 均依赖 x11rb ^0.13，升级到 0.14 会造成
  依赖树双版本并存且无功能收益。
  验证方法：platform/linux.rs 无法在 Windows 本机编译，需临时 crate +
  `cargo check --target x86_64-unknown-linux-gnu`（x11rb 纯 Rust 无 C 依赖；
  含 gtk/tray 的部分只能靠 CI 验证）
- **打包**：`[package.metadata.packager]` 三平台共用（product_name/identifier/
  icons=assets/logo-*.png/resources=locales），CI 用 `cargo packager --release
  --formats X --target T` 指定；macOS 的 icns 由 cargo-packager 从 PNG 自动生成；
  Linux 构建需 libssl-dev + pkg-config（native-tls 依赖 OpenSSL）及
  libarchive-tools/zstd
- **版本与预发布**：Cargo.toml 恒为干净正式版本（如 1.5.1），预发布只经 tag
  表达，分隔符用 `_` 不用 `-`（`v1.5.1_beta.1`/`v1.5.1_rc2` 为预发布，
  `v1.5.1` 为正式；RPM Version 与 pacman pkgver 字段均禁止 `-`）。
  CI 构建时经环境变量 `WALLWARP_DISPLAY_VERSION=<tag 版本>` 注入，build.rs
  据此生成 `WALLWARP_VERSION`（未注入回退 CARGO_PKG_VERSION），业务代码一律
  读 `env!("WALLWARP_VERSION")` 显示版本（Windows PE 数字版本 FILEVERSION
  仍取干净版本）；全部产物文件名在 CI `Rename installers` 步骤统一重写：
  版本号替换为 tag 版本 + 架构前补平台名（windows/macos/linux，如
  `wallwarp_1.6.0_linux_x64.deb`、`wallwarp-1.6.0-1-linux_aarch64.rpm`、
  `wallwarp_1.6.0_macos_x64.dmg`、`wallwarp_1.6.0_macos_x64.app.tar.gz`）；
  上游默认名映射注意点：deb/AppImage 用 Debian/RPM 各自的架构名（amd64/
  x86_64/aarch64），dmg 用 product_name（WallWarp_ 前缀，x86_64→x64），
  app 归档名 WallWarp.app.tar.gz 无版本无架构
- **Linux deb/rpm/pacman**：deb 由 cargo-packager 生成；rpm 由 cargo-generate-rpm
  按 `[package.metadata.generate-rpm]`（布局与 deb 对齐）单独生成——cargo-packager
  不支持 rpm；pacman 由 CI 在 cargo-packager 的 pacman 数据 tar.gz 基础上装配
  （上游只产出 PKGBUILD+数据包，非标准包）：写入 .PKGINFO（pkgver 带 -1 后缀，
  非法字符防御性替换为 _，depend=gtk3/libayatana-appindicator/openssl/xdotool，
  xdotool 提供 muda 运行时动态链接的 libxdo）+ bsdtar 生成 .MTREE（文件+目录，
  md5/sha256 摘要）+ zstd 压缩（--owner=0 --group=0 归零属主，.PKGINFO 必须是
  tar 首条目），产物 `wallwarp-{ver}-1-{arch}.pkg.tar.zst` 可直接 `pacman -U` 安装

## 开发规范

### 1. 代码风格
- 使用 Rust 标准代码风格 (rustfmt)
- 使用 4 个空格进行缩进
- 遵循 Rust 命名约定 (snake_case 用于函数和变量，PascalCase 用于类型和 trait)
- 所有公共 API 应包含文档注释

### 2. 项目架构模式
- 采用 Model-View-ViewModel (MVVM) 模式
- State 模块: 管理应用程序状态
- UI 模块: 处理用户界面和消息传递
- Services 模块: 实现业务逻辑
- Utils 模块: 提供通用工具函数
- I18n 模块: 处理多语言支持

### 3. 多语言支持规范
- 所有UI文本必须通过 I18n 实例的 t() 方法获取
- 含占位符的词条：FTL 中变量必须写成 `{$name}`（带 `$`，`{name}` 是消息引用不会被插值），
  代码用 `t_with_args(key, &[("name", value.to_string())])` 传参，禁止用 `.replace()` 手动替换
- 新文本同步添加到现有的翻译文件中
- 翻译文件存放在 locales/ 目录中
- 词条查找顺序：当前语言 → 默认语言(zh-cn) → 返回键名并打 warning 日志（相同键只告警一次）
- 语言列表支持运行时刷新：`I18n::refresh_languages()` 会重扫 locales 目录增量加载新语言文件；
  设置页语言下拉框展开时（`settings_language_picker_expanded`）自动触发，用户新增 .ftl 文件无需重启
- i18n 模块结构：`src/i18n/mod.rs`（目录扫描/加载/语言列表）、`src/i18n/translate.rs`（翻译查找/插值/回退/告警）

### 4. Iced 框架规范
- 遵循 Iced 0.14.0 的响应式编程模型
- 使用 Message 枚举处理 UI 事件
- View 函数返回 Element 类型
- 界面布局：左侧菜单区域包含AppName和菜单项，菜单项作为一个整体垂直居中显示
- 界面布局：左右面板之间使用分隔线分隔，提升界面美观度

### 4.1 Iced DropDown 组件使用规范
- **依赖引入**: 使用项目本地化的 `crate::ui::common::drop_down::{self, DropDown}` 组件（API 与 iced_aw 完全一致）
  - **本地化原因**: iced_aw 0.14 的 overlay 定位存在回归（见 iced_aw issue #334/#300）：
    越界处理从"贴边钳制"改为"整体翻转"，且用 scrollable 可见区域的宽高与窗口绝对坐标比较，
    导致滚动区域内靠右/靠下的下拉框被错误翻转位置。iced_aw 0.13.0 无此问题。
    本地版本在 `src/ui/common/drop_down/overlay.rs` 中修正了边界计算（计入 viewport 的 x/y 偏移，越界时贴边钳制）。
  - **升级提示**: 若未来升级 iced_aw 且上游已修复该问题，可评估切回 `iced_aw::drop_down` 并删除本地模块
- **组件结构**: DropDown 由三部分组成：underlay（触发按钮）、overlay（下拉内容）、expanded（展开状态）
- **基本用法**:
  ```rust
  use crate::ui::common::drop_down::{self, DropDown};
  
  // 创建触发按钮（underlay）
  let underlay = button(text("选择")).on_press(Message::Expand);
  
  // 创建下拉内容（overlay）
  let overlay = column![
      button("选项1").on_press(Message::Select(Option1)),
      button("选项2").on_press(Message::Select(Option2)),
  ];
  
  // 组合 DropDown 组件
  let dropdown = DropDown::new(underlay, overlay, state.expanded)
      .width(Length::Fill)  // 关键：必须设置 width(Length::Fill) 以确保下拉内容正确显示在按钮下方
      .on_dismiss(Message::Dismiss)
      .alignment(drop_down::Alignment::Bottom);
  ```
- **关键注意事项**:
  - **宽度设置**: 必须调用 `.width(Length::Fill)`，否则下拉内容会显示在屏幕左侧而不是按钮下方
  - **对齐方式**: 使用 `.alignment(drop_down::Alignment::Bottom)` 让下拉内容显示在按钮下方
  - **状态管理**: 需要在 State 中维护 `expanded` 布尔值来控制展开/收起状态
  - **消息处理**: 
    - 展开消息：切换 expanded 状态
    - 选择消息：处理选项选择并自动关闭（设置 expanded = false）
    - 关闭消息：设置 expanded = false
  - **overlay 内容**: 可以使用 `scrollable` 包裹以支持滚动
- **容器宽度设置规范**（实践经验）:
  - **何时需要指定容器固定宽度**: 
    - 当 overlay 内容使用 `row` 或 `column` 等布局容器，且内部元素使用了 `Length::Fill` 或 `Length::Fixed` 时
    - 当 overlay 内容的宽度需要精确控制时（如表格、网格布局）
    - 当 overlay 内容包含多个水平排列的分组时
  - **何时不需要指定容器固定宽度**:
    - 当 overlay 内容是简单的垂直列表，且每个元素宽度自适应时
    - 当 overlay 内容使用固定宽度的元素（如按钮、文本）且不需要精确布局时
  - **实践总结**:
    1. **颜色选择器**（不需要指定容器宽度）:
       - 使用 `column` + `row` 网格布局
       - 每个颜色按钮使用 `Length::Fixed(64.0)` 固定宽度
       - 容器不设置宽度，让内容自适应
       - DropDown 设置 `.width(Length::Fill)` 后，弹窗宽度由内容决定（约400像素）
    2. **分辨率选择器**（需要指定容器宽度）:
       - 使用 `row` 水平排列5个分组列
       - 每个分组列使用 `Length::Fixed(100.0)` 固定宽度
       - 容器不设置宽度，但内部元素使用了固定宽度
       - DropDown 设置 `.width(Length::Fill)` 后，弹窗宽度由内容决定（约500像素）
    3. **通用原则**:
       - 如果 overlay 内容使用了 `Length::Fill`，则必须在容器上设置固定宽度
       - 如果 overlay 内容只使用 `Length::Fixed`，则容器宽度可自适应
       - 始终在 DropDown 上设置 `.width(Length::Fill)` 以确保正确对齐
  - **推荐做法**:
    - 对于复杂的布局（表格、网格、多列），建议在容器上设置合适的固定宽度
    - 对于简单的列表，可以不设置容器宽度，让内容自适应
    - 无论哪种情况，都必须在 DropDown 上设置 `.width(Length::Fill)`
  - **防止鼠标穿透**:
    - **重要性**: 必须在返回容器时使用 `iced::widget::opaque()` 包裹容器，防止点击弹窗空白区域时鼠标事件穿透到下层内容
    - **实现方式**: 
      ```rust
      use iced::widget::opaque;
      
      let picker_content = opaque(container(grid).padding(12).style(...));
      
      picker_content.into()
      ```
    - **作用**: `opaque` 会让容器区域拦截所有鼠标事件，避免用户点击弹窗的空白区域时触发下层内容的操作
    - **示例**: 颜色选择器和分辨率选择器的容器都应该用 `opaque` 包裹
- **示例场景**:
  - 颜色选择器：使用5×6网格布局，包含29种颜色+1个Any选项
  - 分辨率选择器：使用5列水平布局，每列包含多个分辨率选项
  - 筛选器：支持下拉选择各种筛选条件
  - 菜单：实现多级菜单导航

### 5. 文件组织
- 每个模块在 src 目录下有对应子目录
- 模块内部功能按职责分离到不同文件
- 相关功能的测试文件与源文件位于同一目录

### 6. 代码文件规范
- **文件长度限制**: 代码文件应该尽量不超过300行；稍微超过但在50行以内且逻辑已经比较紧凑的，也在接受范围之内
- **职责单一原则**: 每个文件应保持职能单一，便于后续使用AI工具读取和迭代内容
- **拆分时机**: 当文件超过300行且逻辑很容易拆分时，应尽量拆分为多个小文件；承担了多个职责时，也应考虑拆分
- **模块化设计**: 通过合理的模块化和子模块划分，将复杂功能分散到多个文件中
- **可维护性**: 保持小文件结构有助于代码理解、测试和维护

### 7. 配置文件规范
- 配置文件位于程序同级目录，文件名为 config.ini
- 程序启动时自动从项目根目录读取配置文件
- 如果配置文件不存在，启动时自动创建默认配置文件
- 配置更改时自动同步到配置文件
- 支持语言切换、窗口大小和位置等配置项的持久化存储

### 8. 错误处理
- 使用 Result 类型进行错误处理
- 提供有意义的错误信息
- 对外部依赖操作进行适当的错误处理

### 9. 图像处理
- 支持主流图像格式 (jpg, jpeg, png, bmp, gif, webp)
- 在应用壁纸前验证图像路径有效性
- 对图像尺寸进行格式化处理

### 10. Iced 模态窗口实现规范
- 使用 `iced::widget::stack` 组件将模态内容叠加在底层内容之上
- 使用 `iced::widget::opaque` 包装整个模态内容，阻止鼠标事件穿透到底层
- 模态内容应包含半透明背景遮罩和居中的对话框
- 通过 `container.center_x(Length::Fill).center_y(Length::Fill)` 实现对话框居中
- 模态窗口的显示/隐藏状态应通过应用状态管理
- 在应用的主 view 函数中使用 stack 将底层内容和模态窗口叠加
- 确保模态窗口内容能够正确响应用户交互，同时底层内容不可操作

### 11. 代码优化规范
- **避免代码重复**: 当相同或相似的代码逻辑在多处出现时，必须提取为公共方法或函数
- **提取辅助方法**: 对于重复使用的复杂逻辑（如动态图解码器初始化、路径处理、通知显示等），应提取为私有辅助方法
- **合并相似函数**: 功能相同仅参数不同的函数应合并为一个通用函数，通过参数区分不同场景
- **简化消息处理**: update 方法中的消息处理逻辑应保持简洁，复杂逻辑应委托给辅助方法
- **路径处理统一**: 所有相对路径到绝对路径的转换应使用统一的辅助方法处理
- **通知显示统一**: 所有通知的显示和自动隐藏应通过统一的辅助方法处理
- **代码审查要求**: 在提交代码前，必须检查是否存在重复代码，确保遵循 DRY (Don't Repeat Yourself) 原则
- **示例优化场景**:
  - 动态图解码器初始化逻辑在多处重复 → 提取为 `init_animated_decoder()` 方法
  - 图片索引查找逻辑在 NextImage/PreviousImage 中重复 → 提取为 `find_next_valid_image_index()` 方法
  - 路径选择对话框逻辑重复 → 合并为通用的 `select_folder_async()` 函数
  - 通知显示逻辑重复 → 提取为 `show_notification()` 方法

### 12. 公共方法提取规范
- **作用域控制**: 优先将仅适用于当前源码文件的公共方法提取到当前源码文件中，以控制作用域
- **复用性判断**: 如果公共方法大概率能被多处使用（如按钮创建、模态对话框、容器样式等），则提取到 `src/ui/common.rs` 中以便其他源码文件复用
- **常量定义优先级**:
  - 如果 `common.rs` 中已有公共定义的全局常量（如颜色值、尺寸常量等），则优先使用 `common.rs` 中的常量
  - 如果常量仅当前源码文件中使用，则定义在当前源码文件中以控制作用域
- **命名规范**: 提取的公共方法应以清晰的功能命名，如 `create_colored_button`、`create_confirmation_dialog` 等
- **文档注释**: 所有公共方法都应包含清晰的文档注释，说明参数、返回值和用途
- **示例场景**:
  - 按钮创建逻辑在多处使用且样式统一 → 提取到 `common.rs` 中的 `create_colored_button()` 方法
  - 模态确认对话框在多个页面使用 → 提取到 `common.rs` 中的 `create_confirmation_dialog()` 方法
  - 特定于某个页面的布局逻辑 → 提取到该页面的源码文件中作为私有方法
  - 颜色常量（如 BUTTON_COLOR_BLUE）在多处使用 → 定义在 `common.rs` 中
  - 仅某个页面使用的尺寸常量 → 定义在该页面的源码文件中

### 13. 样式常量定义规范
- **集中管理**: UI 开发中的样式相关常量（如颜色、尺寸、间距、字体大小等）应统一定义到 `src/ui/style/` 模块中（colors.rs / theme_colors.rs / dimensions.rs / shadows.rs / color_utils.rs）
- **复用检查**: 添加新常量前，应先检查 `style/` 模块中是否已有相同含义的常量，避免重复定义
- **命名规范**: 样式常量使用 UPPER_SNAKE_CASE 命名，命名应清晰表达其用途（如 `BUTTON_COLOR_BLUE`、`SECTION_PADDING` 等）
- **示例场景**:
  - 新增按钮颜色常量 → 先检查 `style.rs` 是否已有相同颜色定义
  - 新增间距常量 → 检查 `style.rs` 中是否已有相同用途的间距定义
  - 新增主题颜色 → 应在 `style.rs` 中统一定义，便于后续主题切换

### 14. 日志输出规范
- **日志标识要求**: 所有日志输出必须包含明确的上下文标识，便于追踪和调试
- **标识内容优先级**:
  - 对于特定资源的操作（如壁纸、文件、API请求），必须在日志中包含资源ID或资源路径
  - 对于批量操作或搜索操作，应包含操作参数的简明标识（如搜索参数组合标签）
  - 避免使用无意义的通用日志，如"请求成功"、"解析成功"等，必须带上操作对象
- **日志格式规范**:
  - 使用统一的日志前缀格式：`[模块名] [标识] 消息内容`
  - 示例：`[Wallhaven API] [ID:abc123] 请求失败: timeout`
  - 示例：`[Wallhaven API] [page1_catGeneral_sortToplist] 解析成功，获取到 24 张壁纸`
- **减少冗余日志**:
  - 避免输出过长的数据内容（如完整响应体），使用摘要或截断方式
  - 仅在调试必要时输出详细数据，正常流程使用简明日志
  - 错误日志必须包含足够的上下文信息以便定位问题
- **示例场景**:
  - API请求日志：`[Wallhaven API] [ID:xyz789] 响应状态: 200 OK`
  - 搜索操作日志：`[Wallhaven API] [page1_catAnime] 请求URL: https://...`
  - 错误日志：`[Wallhaven API] [ID:abc123] JSON解析失败: unexpected token`

### 15. 依赖管理规范
- **新依赖优先最新稳定版**: 新增依赖库时，一律使用其最新的稳定版本（不含预发布版），以规避旧版本已知安全漏洞、及时获得新特性与缺陷修复，也避免版本差距积压导致后续升级迁移成本越拖越大
- **平台专属依赖同理**: 新增 `[target.'cfg(...)'.dependencies]` 平台依赖（如 windows/objc2/x11rb）同样遵循最新稳定版原则，并在对应平台上完成编译验证（无法本地编译的目标用交叉 `cargo check` 或 CI 验证）
- **例外——版本受生态锁定时**: 若最新版与依赖树中其他库的版本要求冲突（如 x11rb 被 arboard/winit 锁定 `^0.13`），则选择与生态一致的版本、保持依赖树单一副本，并在 Cargo.toml 注释中说明锁定原因，待上游迁移后自然统一，不做强行超前
- **升级后核对锁版本**: 编辑 Cargo.toml 会触发依赖重解析，锁版本可能被意外降级（案例：reqwest 0.13.4 → 0.13.1 导致 SOCKS5 代理全部失效）。每次改动依赖后必须核对 Cargo.lock 中关键包版本：新增的包确认是预期版本，既有包未被降级
- **记录升级理由与差异**: 引入或升级重要依赖时在 Cargo.toml 注释中写明选型理由（如 reqwest 用 native-tls 走系统 TLS 栈的说明），便于后续维护者判断能否安全升级

### 16. 主题与配色规范
- **唯一强调色**: 全应用统一使用现代蓝——浅色主题 `#3B82F6`，深色主题 `#5C9DFF`。需要强调色的场景（按钮、选中态、指示条、焦点边框、链接）一律使用 `ThemeColors::primary` / `primary_hover` / `primary_active`，禁止散落新的蓝色硬编码值
- **主题色与固定色的分界**: 随明暗主题变化的颜色定义在 `style/theme_colors.rs` 的 `ThemeColors::light()/dark()`；与主题无关的固定色（Wallhaven 29 色板、纯净度语义色、按钮语义色）定义在 `style/colors.rs`。新增颜色时先判断属于哪一类
- **交互态必须完整**: 可交互控件（按钮、菜单项、下拉选项、表格行、输入框）的样式闭包应接收 `status` 参数并区分 Hovered / Pressed / Disabled——通用悬停填充用 `hover_fill`，彩色按钮悬停用 `darken(color, 0.08)`（style/color_utils.rs 提供 darken/lighten/with_alpha/tint）
- **统一样式函数优先**: text_input 一律用 `common::styled_text_input(theme_colors)`（含焦点强调色描边与禁用弱化）、进度条用 `common::styled_progress_bar`、下拉面板/选项/单元格用 `drop_down::styles::*`、壁纸卡片用 `common::wallpaper_card_button_style` / `wallpaper_image_container_style`；新增同类控件时扩展这些函数而非另写内联样式
- **圆角刻度**: 圆角统一使用 `RADIUS_SM`(4) / `RADIUS_MD`(8) / `RADIUS_LG`(12) / `RADIUS_FULL`(20)，不要出现新的魔法数字
- **选中态视觉语言**: 列表/下拉的选中项 = 强调色 12% 淡染底 + 强调色文字；网格单元格选中项 = 淡染底 + 强调色 1.5px 描边。不要用实心蓝底白字

### 17. 图标码点规范
- **图标来源**: 全部图标使用 `assets/icons.ttf`（Bootstrap Icons），通过 `Font::with_name("bootstrap-icons")` + Unicode 码点渲染；界面禁止再使用 emoji 图标或 unicode 符号充当图标（▲▼⇅⏷ 等）
- **码点必须验证**: 新增图标前必须用字体 cmap 验证码点（源码注释可能失真，历史案例：`\u{F341}` 注释写 folder-fill 实际渲染 eye）。验证方式：用 Python struct 解析 icons.ttf 的 cmap+post 表得到 glyph 名→码点映射，或对照 bootstrap-icons 官方码点表
- **常用已验证码点**: house-door=`F423`、folder=`F3D7`、folder2-open=`F3D8`、download=`F30A`、gear=`F3E5`、chevron-down=`F282`、chevron-right=`F285`、chevron-up=`F286`、chevron-expand=`F283`、x-lg=`F659`、x-circle-fill=`F622`、trash3=`F78B`、arrow-repeat=`F130`、arrow-counterclockwise=`F117`、copy=`F759`、check2-circle=`F270`、exclamation-triangle-fill=`F33A`、info-circle-fill=`F430`、image-fill=`F429`、image-alt=`F428`、pause-fill=`F4C3`、play-fill=`F4F4`、dash=`F2EA`、dash-lg=`F63B`
- **注释即契约**: 图标码点旁的注释必须写渲染出的真实 glyph 名（如 `// x-circle-fill (取消)`），发现注释与实际渲染不符时以字体为准修正
