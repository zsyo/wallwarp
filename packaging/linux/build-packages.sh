#!/usr/bin/env zsh
# =====================================================================
# WallWarp Linux 安装包本地构建脚本（WSL + Docker）
#
# 在 WSL 的 zsh 中直接构建 appimage / deb / rpm / pacman(pkg.tar.zst)
# 四种安装包，编译环境由 Docker 容器控制（与 CI 的 release.yml 链路一致），
# 免去本机安装完整构建环境，也免去为验证打包专门跑 GitHub Actions。
#
# 用法（NTFS 挂载盘无执行权限，建议以 zsh 方式运行）：
#   zsh packaging/linux/build-packages.sh [-a x64|arm64] [-f all|<格式组合>] [-v 版本]
#
#   -a  目标架构，默认 x64
#       arm64 在 x86 宿主上经 qemu 模拟编译，全量构建可能需要 1 小时以上
#   -f  包格式：all（默认）或逗号组合，如 deb,rpm / appimage / pacman
#   -v  应用内显示版本（WALLWARP_DISPLAY_VERSION，默认取 Cargo.toml 版本；
#       产物文件名恒用 Cargo.toml 版本，与 CI 的 tag 重命名逻辑无关）
#   -r  基础镜像源：auto（默认，探测 Docker Hub 连通性，不通自动切换
#       加速源）/ direct（强制直连）/ 指定加速源域名（如 docker.m.daocloud.io）
#
# 产物输出：仓库根 dist-linux/
# 编译缓存：~/.cache/wallwarp-build/<arch>/{cargo,target}（WSL ext4，
#           避免 /mnt 跨盘 IO 拖慢增量编译；cargo clean 不会清掉缓存，
#           需要释放空间时手动删除该目录即可）
# =====================================================================

emulate -L zsh
setopt err_exit no_unset pipefail

# ---------- 参数解析 ----------
ARCH=x64
FORMATS=all
DISPLAY_VERSION=""
REGISTRY="auto"

usage() {
  print -u2 "用法: zsh $0 [-a x64|arm64] [-f all|appimage,deb,rpm,pacman] [-v 版本] [-r auto|direct|<加速源域名>]"
}

while (( $# > 0 )); do
  case $1 in
    -a|--arch)    ARCH=$2; shift 2 ;;
    -f|--formats) FORMATS=$2; shift 2 ;;
    -v|--version) DISPLAY_VERSION=$2; shift 2 ;;
    -r|--registry) REGISTRY=$2; shift 2 ;;
    -h|--help)    usage; exit 0 ;;
    *)            print -u2 "未知参数: $1"; usage; exit 1 ;;
  esac
done

case $ARCH in
  x64|arm64) ;;
  *) print -u2 "无效架构: $ARCH（仅支持 x64 / arm64）"; exit 1 ;;
esac

if [[ $FORMATS != all ]]; then
  for f in ${(s:,:)FORMATS}; do
    [[ $f == (appimage|deb|rpm|pacman) ]] || {
      print -u2 "无效格式: $f（可选 appimage / deb / rpm / pacman / all）"; exit 1
    }
  done
fi

# ---------- 定位仓库根与运行环境 ----------
REPO_ROOT="${0:A:h}/../.."
cd "$REPO_ROOT"

command -v docker >/dev/null || { print -u2 "未找到 docker 命令"; exit 1 }
docker info >/dev/null 2>&1 || { print -u2 "Docker 守护进程未运行"; exit 1 }

# ---------- 架构映射（与 release.yml 矩阵一致）----------
case $ARCH in
  x64)
    TARGET=x86_64-unknown-linux-gnu
    RUSTFLAGS="-C target-cpu=x86-64-v3"
    DOCKER_ARCH=amd64
    PKG_ARCH=x86_64
    ;;
  arm64)
    TARGET=aarch64-unknown-linux-gnu
    RUSTFLAGS=""
    DOCKER_ARCH=arm64
    PKG_ARCH=aarch64
    ;;
esac

PLATFORM_ARGS=()
if [[ $(uname -m) != $PKG_ARCH ]]; then
  PLATFORM_ARGS=(--platform "linux/$DOCKER_ARCH")
  [[ $ARCH == arm64 ]] && print "提示: x86 宿主上经 qemu 模拟 arm64，全量编译较慢"
fi

if [[ -z $DISPLAY_VERSION ]]; then
  # tr -d '\r'：仓库在 Windows 检出时 Cargo.toml 为 CRLF，避免 \r 混入版本号
  DISPLAY_VERSION=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/' | tr -d '\r')
fi

# ---------- 基础镜像源解析（Docker Hub 直连不通时自动切换加速源）----------
probe_http() {
  command -v curl >/dev/null || return 1
  curl -sI --max-time 5 "$1" >/dev/null 2>&1
}

RUST_IMAGE_PREFIX=""
case $REGISTRY in
  auto)
    if ! probe_http "https://auth.docker.io/token"; then
      print "Docker Hub 直连不通，探测镜像加速源..."
      for m in docker.m.daocloud.io docker.1ms.run docker.xuanyuan.me dockerproxy.net; do
        if probe_http "https://$m/v2/"; then
          print "使用加速源: $m"
          RUST_IMAGE_PREFIX="$m/"
          break
        fi
      done
      [[ -n $RUST_IMAGE_PREFIX ]] || {
        print -u2 "所有镜像源均不可达，请检查网络或用 -r <加速源域名> 手动指定"
        exit 1
      }
    fi
    ;;
  direct) ;;
  *) RUST_IMAGE_PREFIX="$REGISTRY/" ;;
esac

BUILD_CACHE="$HOME/.cache/wallwarp-build/$ARCH"
mkdir -p "$BUILD_CACHE/cargo" "$BUILD_CACHE/target" dist-linux

# cargo-packager 打 appimage 时会从 GitHub 下载 linuxdeploy/AppRun 工具链，
# 容器内经 rootlesskit 网络访问 GitHub 易挂起；在 WSL 侧（GitHub 直连正常）
# 预下载到挂载卷内，并把容器的 XDG_CACHE_HOME 指向卷内，cargo-packager
# 检测到文件已存在即跳过下载
APPIMAGE_TOOLS="$BUILD_CACHE/cargo/cache/.cargo-packager/AppImage"
if [[ $FORMATS == all || $FORMATS == *appimage* ]] \
  && [[ ! -f $APPIMAGE_TOOLS/linuxdeploy-$PKG_ARCH.AppImage ]]; then
  mkdir -p "$APPIMAGE_TOOLS"
  print "==> 预下载 AppImage 工具链（linuxdeploy/AppRun，仅首次）"
  curl -fL --retry 3 -o "$APPIMAGE_TOOLS/AppRun-$PKG_ARCH" \
    "https://github.com/tauri-apps/binary-releases/releases/download/apprun-old/AppRun-$PKG_ARCH"
  curl -fL --retry 3 -o "$APPIMAGE_TOOLS/linuxdeploy-$PKG_ARCH.AppImage" \
    "https://github.com/tauri-apps/binary-releases/releases/download/linuxdeploy/linuxdeploy-$PKG_ARCH.AppImage"
  curl -fL --retry 3 -o "$APPIMAGE_TOOLS/linuxdeploy-plugin-appimage.AppImage" \
    "https://github.com/linuxdeploy/linuxdeploy-plugin-appimage/releases/download/continuous/linuxdeploy-plugin-appimage-$PKG_ARCH.AppImage"
  chmod 755 "$APPIMAGE_TOOLS"/*
fi

# 容器运行时 CARGO_HOME 指向挂载卷，cargo 源配置需写入卷内
# （rsproxy 加速；海外环境删除此文件即恢复官方源）
if [[ ! -f $BUILD_CACHE/cargo/config.toml ]]; then
  cat > "$BUILD_CACHE/cargo/config.toml" <<'CARGOCFG'
[source.crates-io]
replace-with = "rsproxy-sparse"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[net]
git-fetch-with-cli = true
CARGOCFG
fi

IMAGE="wallwarp-builder:$ARCH"

# ---------- 构建编译镜像（层缓存命中时秒级完成）----------
print "==> 构建编译镜像 $IMAGE（首次需拉取 rust 基础镜像并编译打包工具）"
docker build "${PLATFORM_ARGS[@]}" --build-arg RUST_IMAGE="${RUST_IMAGE_PREFIX}library/rust:latest" \
  -t "$IMAGE" - <<'DOCKERFILE'
ARG RUST_IMAGE=rust:latest
FROM ${RUST_IMAGE}

# apt 换中科大镜像源（国内网络直连 deb.debian.org 限速严重；
# 海外环境删除本行的 sed 部分即可换回官方源）
RUN sed -i 's|deb.debian.org|mirrors.ustc.edu.cn|g' \
      /etc/apt/sources.list.d/debian.sources /etc/apt/sources.list 2>/dev/null; \
    apt-get update && apt-get install -y --no-install-recommends \
        libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
        libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev \
        libx11-dev libxcb1-dev libxrandr-dev libxi-dev cmake \
        libssl-dev pkg-config libxdo-dev \
        libarchive-tools zstd \
    && rm -rf /var/lib/apt/lists/*

# 打包工具（cargo-packager 生成 appimage/deb/pacman 数据，
# cargo-generate-rpm 生成 rpm），编译进镜像层一次成型。
# crates 下载走 rsproxy 加速（原因同上，海外环境删除 printf 部分）
RUN mkdir -p /usr/local/cargo \
 && printf '[source.crates-io]\nreplace-with = "rsproxy-sparse"\n\n[source.rsproxy-sparse]\nregistry = "sparse+https://rsproxy.cn/index/"\n' \
      > /usr/local/cargo/config.toml \
 && cargo install --locked cargo-packager \
 && cargo install --locked cargo-generate-rpm
DOCKERFILE

# ---------- 容器内构建 ----------
# 不指定 --user：rootless docker 下容器 root 映射到宿主当前用户，
# 产物落盘属主即 WSL 用户（指定 --user 反而会映射到 subuid 段导致
# 挂载目录不可写）；CARGO_HOME 与 target 均绑定到 ext4 缓存目录，
# 避免 NTFS 跨盘 IO；APPIMAGE_EXTRACT_AND_RUN=1：容器内无 FUSE，
# linuxdeploy 走解压模式运行
print "==> 容器内编译（target=$TARGET formats=$FORMATS version=$DISPLAY_VERSION）"
docker run --rm -i \
  "${PLATFORM_ARGS[@]}" \
  -v "$REPO_ROOT:/work" \
  -w /work \
  -v "$BUILD_CACHE/cargo:/cargo-home" \
  -v "$BUILD_CACHE/target:/work/target" \
  -e CARGO_HOME=/cargo-home \
  -e "RUSTFLAGS=$RUSTFLAGS" \
  -e "WALLWARP_DISPLAY_VERSION=$DISPLAY_VERSION" \
  -e APPIMAGE_EXTRACT_AND_RUN=1 \
  -e TARGET="$TARGET" \
  -e FORMATS="$FORMATS" \
  -e PKG_ARCH="$PKG_ARCH" \
  "$IMAGE" bash -s <<'INSIDE'
set -euo pipefail

version="$WALLWARP_DISPLAY_VERSION"
target="$TARGET"
out_dir="target/$target/release"
dist="dist-linux"
mkdir -p "$dist"

echo "==> cargo build --release --target $target"
cargo build --release --target "$target"

# cargo-packager 负责的三种格式（rpm 不走 cargo-packager）
packager_formats=""
[[ $FORMATS == all || $FORMATS == *appimage* ]] && packager_formats+="appimage,"
[[ $FORMATS == all || $FORMATS == *deb* ]] && packager_formats+="deb,"
[[ $FORMATS == all || $FORMATS == *pacman* ]] && packager_formats+="pacman,"
packager_formats=${packager_formats%,}

if [[ -n $packager_formats ]]; then
  echo "==> cargo packager --formats $packager_formats"
  cargo packager --release --formats "$packager_formats" --target "$target"
fi

if [[ $FORMATS == all || $FORMATS == *rpm* ]]; then
  echo "==> cargo generate-rpm"
  # 布局见 Cargo.toml [package.metadata.generate-rpm]；
  # 二进制路径按 --target 自动改写为 target/<triple>/release/wallwarp
  cargo generate-rpm --target "$target"
  cp target/"$target"/generate-rpm/*.rpm "$dist"/
fi

[[ $FORMATS == all || $FORMATS == *deb* ]] && cp "$out_dir"/*.deb "$dist"/
[[ $FORMATS == all || $FORMATS == *appimage* ]] && cp "$out_dir"/*.AppImage "$dist"/

# pacman 装配（与 release.yml 的 Package Pacman 步骤一致）：
# cargo-packager 只产出 PKGBUILD + 数据 tar.gz，在此装配为标准包
if [[ $FORMATS == all || $FORMATS == *pacman* ]]; then
  echo "==> 装配 pacman 包"
  # pkgver 只允许字母数字与 . _ +（'-' 保留为 pkgrel 分隔符），防御性清洗
  pkgver="${version//[^[:alnum:].+_]/_}"

  shopt -s nullglob
  data_tars=("$out_dir"/wallwarp_*_*.tar.gz)
  shopt -u nullglob
  if (( ${#data_tars[@]} != 1 )); then
    echo "期望恰好 1 个 pacman 数据包，实际 ${#data_tars[@]} 个" >&2
    exit 1
  fi

  mkdir -p pacman-pkg
  tar -xzf "${data_tars[0]}" -C pacman-pkg
  installed_size="$(du -sb pacman-pkg | cut -f1)"

  cat > pacman-pkg/.PKGINFO <<EOF
pkgname = wallwarp
pkgbase = wallwarp
pkgver = ${pkgver}-1
pkgdesc = WallWarp 桌面壁纸管理软件
url = https://github.com/zsyo/wallwarp
builddate = $(date +%s)
packager = zsyo <zephyr@aico.top>
size = ${installed_size}
arch = ${PKG_ARCH}
license = AGPL-3.0
depend = gtk3
depend = libayatana-appindicator
depend = openssl
depend = xdotool
xdata = pkgtype=pkg
EOF

  # .MTREE 与 makepkg 相同方式生成（文件+目录，bsdtar mtree 格式，含摘要）
  (cd pacman-pkg && find .PKGINFO usr | LC_ALL=C sort | \
    bsdtar --uid 0 --uname root --gid 0 --gname root -czf .MTREE \
    --format=mtree \
    --options='!all,use-set,type,uid,gid,mode,time,size,md5,sha256,link' -T -)

  pkg_file="$dist/wallwarp-${pkgver}-1-linux_${PKG_ARCH}.pkg.tar.zst"
  # .PKGINFO 必须是首个条目；包内属主归零（makepkg/fakeroot 语义）
  tar --zstd --owner=0 --group=0 --numeric-owner -cf "$pkg_file" \
    -C pacman-pkg .PKGINFO .MTREE usr
  rm -f "${data_tars[0]}" "$out_dir/PKGBUILD"
  rm -rf pacman-pkg
  echo "[pacman] 已生成 $pkg_file"
fi
INSIDE

print ""
print "==> 构建完成，产物目录: $REPO_ROOT/dist-linux"
ls -la dist-linux
