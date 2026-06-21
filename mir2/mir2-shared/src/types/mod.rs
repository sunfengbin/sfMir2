use serde_derive::{Deserialize, Serialize};

pub type CharId = u64;
pub type GuildId = u64;
pub type EntityId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Job {
    Warrior,
    Mage,
    Taoist,
}

impl Default for Job {
    fn default() -> Self { Job::Warrior }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self { Gender::Male }
}
