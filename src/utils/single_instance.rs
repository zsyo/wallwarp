use interprocess::local_socket::traits::tokio::Listener as _;
use interprocess::local_socket::{GenericNamespaced, ListenerOptions, Name, prelude::*};
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{ASFW_ANY, AllowSetForegroundWindow};

pub const WAKE_UP: &str = "WAKE_UP";

pub struct SingleInstanceGuard;

impl SingleInstanceGuard {
    fn get_socket_name() -> io::Result<Name<'static>> {
        // 唯一 ID，无需包含 \\.\pipe\ 或 /tmp/，库会处理
        const SOCKET_ID: &str = "top.aico.wallwarp.sock";
        SOCKET_ID
            .to_ns_name::<GenericNamespaced>()
            .map(|n| n.into_owned())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    pub fn send_args_to_existing_instance(start_hidden: bool) -> bool {
        if let Ok(name) = Self::get_socket_name() {
            if let Ok(mut conn) = LocalSocketStream::connect(name) {
                if start_hidden {
                    // 如果本次是后台启动，我们发现已经有实例在运行了，
                    // 那么直接返回 true 让当前进程退出，【不发送】任何消息给旧实例。
                    return true;
                }

                #[cfg(windows)]
                unsafe {
                    // 告诉系统：允许另一个进程（我们的主实例）带走前台焦点
                    // ASFW_ANY 允许任何进程（通常指接下来获得焦点的进程）置顶
                    let _ = AllowSetForegroundWindow(ASFW_ANY);
                }

                let args: Vec<String> = std::env::args().collect();
                let msg = args.get(1).map(|s| s.as_str()).unwrap_or(WAKE_UP);
                let _ = conn.write_all(format!("{}\n", msg).as_bytes());
                let _ = conn.flush();
                return true;
            }
        }
        false
    }

    pub async fn listen() -> String {
        let name = match Self::get_socket_name() {
            Ok(name) => name,
            Err(e) => return format!("NAME_ERROR: {}", e),
        };

        let listener = match ListenerOptions::new().name(name).create_tokio() {
            Ok(listener) => listener,
            Err(e) => return format!("LISTEN_ERROR: {}", e),
        };

        loop {
            match listener.accept().await {
                Ok(conn) => {
                    let mut reader = BufReader::new(conn);
                    let mut buffer = String::new();
                    match reader.read_line(&mut buffer).await {
                        Ok(n) if n > 0 => {
                            // 只有读到了字节（非 EOF）才返回
                            let trimmed = buffer.trim();
                            if !trimmed.is_empty() {
                                return trimmed.to_string();
                            }
                        }
                        _ => {
                            // 读取失败或读到 EOF (n=0)，说明是静默连接或连接已断开
                            // 继续循环，等待下一个真正的连接
                            continue;
                        }
                    }
                }
                Err(e) => {
                    // 连接错误
                    return format!("ACCEPT ERROR: {}", e);
                }
            }
        }
    }
}
