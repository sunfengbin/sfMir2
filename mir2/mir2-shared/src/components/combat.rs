use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Health {
    pub current: u32,
    pub max: u32,
    pub regen_per_sec: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Mp {
    pub current: u32,
    pub max: u32,
    pub regen_per_sec: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CombatStats {
    pub dc_min: u32,
    pub dc_max: u32,
    pub mc_min: u32,
    pub mc_max: u32,
    pub sc_min: u32,
    pub sc_max: u32,
    pub ac: u32,
    pub mac: u32,
}
