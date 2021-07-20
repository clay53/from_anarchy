use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    RegisterPlayer
}

impl Command {
    pub fn to_json_bin(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    pub fn from_json_bin(bin: &[u8]) -> serde_json::Result<Command> {
        serde_json::from_slice(bin)
    }
}