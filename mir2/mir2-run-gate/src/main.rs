//! mir2-run-gate：运行网关（所有游戏消息的枢纽）
//!
//! MVP-0：骨架 —— 待实现：
//! - 与 M2Server 的双向通信
//! - 玩家消息路由（移动/攻击/聊天/技能/背包）

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-run-gate starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
