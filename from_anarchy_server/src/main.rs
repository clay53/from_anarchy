use std::{
    borrow::Cow,
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::prelude::*;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future, pin_mut,
};

use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::tungstenite::protocol::Message;

use from_anarchy_server::*;
use from_anarchy_lib::*;
use from_anarchy_lib::commands::*;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr, game_arc: GameArc) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = async_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, PeerData::new(tx));

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming
        .try_for_each(|msg| {
            let respond = |command: ClientCommand, peer_action: Option<Box<dyn Fn(&mut PeerData) -> ()>>| {
                let bin_command = command.to_bin();
                eprintln!("Sending a response that's {}MB", bin_command.len() as f64/1048576.0);
                let message = Message::binary(bin_command);
                let mut peers = peer_map.lock().unwrap();
                let peer = peers.get_mut(&addr).unwrap();
                if let Some(peer_action) = peer_action {
                    peer_action(peer);
                }
                let reciever = &peer.tx;
                reciever.unbounded_send(message).unwrap();
            };
            if msg.is_close() {
                println!("Socket Disconnected");
            } else if let Ok(command) = ServerCommand::from_bin(&msg.into_data()[..]) {
                match command {
                    ServerCommand::RegisterPlayer => {
                        println!("Registering player...");
                        let mut game = game_arc.lock().unwrap();
                        let id = game.new_player("Player".to_string());
                        respond(ClientCommand::FirstSync(FirstSyncData {
                            player_entity_id: id,
                            map: Cow::Borrowed(&game.get_map()),
                            entities: Cow::Borrowed(&game.get_entities()),
                        }), Some(Box::new(|peer: &mut PeerData| {
                            peer.player_entity_id = Some(id);
                        })));
                    }
                }
            } else {
                eprintln!("Failed to parse command!");
                // for (_, recp) in peers.iter() {
                //     recp.unbounded_send(Message::binary("")).unwrap();
                // }
            }

            future::ok(())
        });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

async fn server_tick(_game_arc: GameArc) {
    
}

async fn accept_connections(listener: TcpListener, peer_map: PeerMap, game_arc: GameArc) {
    println!("Accepting connections...");
    while let Ok((stream, addr)) = listener.accept().await {
        task::spawn(handle_connection(peer_map.clone(), stream, addr, game_arc.clone()));
    }
}

async fn run() -> Result<(), IoError> {
    let addr = "127.0.0.1:8080";

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let game_arc = GameArc::new(Mutex::new(Game::new()));

    let tick = server_tick(game_arc.clone());
    let wsfut = accept_connections(listener, peer_map, game_arc);
    futures::join!(tick, wsfut);

    Ok(())
}

fn main() -> Result<(), IoError> {
    task::block_on(run())
}