use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Position {
    pub map_id: u32,
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
