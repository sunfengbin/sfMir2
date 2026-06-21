//! mir2-login-srv：账号认证服务
//!
//! MVP-0：骨架 —— 待实现：
//! - 账号校验（DBServer 查询）
//! - Session Token 生成
//! - 登录日志

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-login-srv starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
