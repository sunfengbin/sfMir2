use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerTag;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Level(pub u32);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Gold(pub u64);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerName(pub String);
