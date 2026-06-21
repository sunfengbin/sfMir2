//! mir2-db-server：数据库统一入口（MySQL + Redis 缓存）
//!
//! MVP-0：骨架 —— 待实现：
//! - 连接池
//! - Repository（账号/角色/背包/行会）
//! - 缓存策略

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-db-server starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
