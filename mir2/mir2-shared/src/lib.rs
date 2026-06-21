//! mir2-shared：服务端与客户端共享的核心数据与逻辑
//!
//! - components：ECS 组件（纯数据）
//! - constants：配置表常量（物品/技能/怪物）
//! - calc：数值计算（伤害/经验/掉落）
//! - protocol：协议指令 ID、错误码
//! - proto：Protobuf 消息定义与自动生成代码
//! - types：基础类型（CharId / Job / Gender）
//! - config：运行时通用配置结构
//! - util：通用小工具（随机数、编码）

pub mod components;
pub mod constants;
pub mod calc;
pub mod protocol;
pub mod proto;
pub mod types;
pub mod config;
pub mod util;

pub use components::*;
pub use types::{CharId, GuildId, EntityId, Job, Gender};
pub use protocol::{OpCode, ErrorCode};
