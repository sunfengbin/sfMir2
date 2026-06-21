use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MonsterTag;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonsterId(pub u32);

impl Default for MonsterId {
    fn default() -> Self { MonsterId(0) }
}
