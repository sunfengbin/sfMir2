//! mir2-m2：主引擎（ECS World）
//!
//! MVP-0：骨架 —— 待实现：
//! - ECS World（Entity/Component/Resource）
//! - Systems 调度
//! - 地图管理 + 怪物 AI
//! - Lua 脚本加载

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-m2 (ECS engine) starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
