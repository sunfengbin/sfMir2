//! mir2-client：Bevy 游戏客户端
//!
//! MVP-0：骨架 —— 待实现：
//! - App Builder（窗口初始化 / 场景切换）
//! - 登录界面 / 选服 / 游戏场景
//! - 与网关的异步 TCP 通信

use bevy::prelude::*;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("mir2-client (Bevy) starting (MVP-0 scaffold)");

    // 空的 Bevy App（无窗口依赖，保持 cargo check 通过）
    App::new().update();
}
