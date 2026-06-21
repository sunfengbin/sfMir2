//! mir2-game-center：统一运维/GM 管理界面
//!
//! MVP-0：骨架 —— 待实现：
//! - egui 界面（服务状态、在线玩家、公告、GM 指令）
//! - 与各服务的通信（读取状态、发送指令）

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-game-center starting (MVP-0 scaffold — egui placeholder)");

    // 占位：不实际启动 egui（需要图形环境），仅保证能编译通过
    Ok(())
}
