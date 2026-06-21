//! mir2-log-server：集中日志与审计
//!
//! MVP-0：骨架 —— 待实现：
//! - GM 操作审计
//! - 登录日志
//! - 交易日志
//! - 错误/异常事件

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-log-server starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
