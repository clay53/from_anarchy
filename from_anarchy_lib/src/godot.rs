use crate::*;
use crate::commands::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Server;

#[methods]
impl Server {
    fn new(_owner: &Node) -> Server {
        Server
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("HELLO WORLD!!!");
    }

    #[export]
    fn parse_command(&self, _owner: &Node, command_bin: Vec<u8>) {
        if let Ok(command) = ClientCommand::from_json_bin(&command_bin[..]) {
            
        } else {
            godot_error!("Failed to parse command from server! Dump: {:?}", command_bin);
        }
    }

    #[export]
    fn register_player(&self, _owner: &Node) -> Vec<u8> {
        ServerCommand::RegisterPlayer.to_json_bin()
    }
}

// Registers classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Server>();
}

godot_init!(init);