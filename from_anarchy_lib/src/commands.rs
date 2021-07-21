use crate::*;
use std::borrow::Cow;
use serde::{Serialize, Deserialize};

pub trait Command<'a>: Serialize + Deserialize<'a> + std::fmt::Debug {
    fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn from_bin(bin: &'a [u8]) -> bincode::Result<Self> {
        bincode::deserialize(bin)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    RegisterPlayer
}
impl Command<'_> for ServerCommand {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand<'a> {
    FirstSync(FirstSyncData<'a>)
}
impl Command<'_> for ClientCommand<'_> {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirstSyncData<'a> {
    pub player_entity_id: EntityId,
    pub map: Cow<'a, TileMap>,
    pub entities: Cow<'a, EntityMap>,
}