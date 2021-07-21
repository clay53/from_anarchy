use crate::*;
use crate::commands::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Server;

#[methods]
impl Server {
    fn new(_owner: &Spatial) -> Server {
        Server
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        godot_print!("HELLO WORLD!!!");
    }

    #[export]
    fn parse_command(&self, _owner: &Spatial, command_bin: Vec<u8>) {
        if let Ok(command) = ClientCommand::from_bin(&command_bin[..]) {
            match command {
                ClientCommand::FirstSync(data) => {
                    godot_print!("My player's entity id is: {}", data.player_entity_id);
                }
            }
        } else {
            godot_error!("Failed to parse command from server! Dump: {:?}", command_bin);
        }
    }

    #[export]
    fn register_player(&self, _owner: &Spatial) -> Vec<u8> {
        ServerCommand::RegisterPlayer.to_bin()
    }
}

// Registers classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Server>();
}

godot_init!(init);