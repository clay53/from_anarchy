use serde::{Serialize, Deserialize};
pub trait Command<'a>: Serialize + Deserialize<'a> + std::fmt::Debug {
    fn to_json_bin(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn from_json_bin(bin: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bin)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerCommand {
    RegisterPlayer
}
impl Command<'_> for ServerCommand {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    FirstSync(FirstSyncData)
}
impl Command<'_> for ClientCommand {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FirstSyncData {
    // pub player_entity_id: u64
}