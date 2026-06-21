//! mir2-sel-gate：选服/选角色网关
//!
//! MVP-0：骨架 —— 待实现：
//! - 角色列表查询
//! - 创建/删除角色
//! - 角色进入游戏请求

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-sel-gate starting (MVP-0 scaffold)");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
