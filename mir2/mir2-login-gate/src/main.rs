//! mir2-login-gate：登录网关（TCP 入口）
//!
//! MVP-0：骨架 —— 待实现：
//! - TCP 监听
//! - 客户端连接管理
//! - 登录请求转发至 LoginSrv

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-login-gate starting (MVP-0 scaffold)");

    // TODO：实现登录网关 TCP 监听 + 转发到 LoginSrv
    // 占位 sleep，保持进程存活以便 cargo check
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
