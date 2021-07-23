use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use futures::channel::mpsc::UnboundedSender;
use async_std::net::{TcpListener, TcpStream};
use async_tungstenite::tungstenite::protocol::Message;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, PeerData>>>;

use from_anarchy_lib::*;
pub type GameArc = Arc<Mutex<Game>>;

pub struct PeerData {
    pub tx: Tx,
    pub player_entity_id: Option<EntityId>
}

impl PeerData {
    pub fn new(tx: Tx) -> PeerData {
        PeerData {
            tx: tx,
            player_entity_id: None
        }
    }
}