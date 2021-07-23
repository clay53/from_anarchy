use crate::*;
use crate::commands::*;
use gdnative::prelude::*;
use gdnative::api::ResourceLoader;
use std::collections::HashMap;

pub type GTileMap = HashMap<TilePos, Ref<Spatial, Shared>>;
pub type GEntityMap = HashMap<EntityId, Ref<Spatial, Shared>>;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct Server {
    player_entity_id: Option<EntityId>,
    tiles: Ref<Node, Shared>,
    entities: Ref<Node, Shared>,
    map: GTileMap,
    entity_map: GEntityMap
}

#[methods]
impl Server {
    fn new(_owner: &Spatial) -> Server {
        Server {
            player_entity_id: None,
            tiles: unsafe {
                ResourceLoader::godot_singleton().load("res://Tiles.tscn", "PackedScene", false).unwrap()
                .try_cast::<PackedScene>().unwrap()
                .assume_safe().instance(0).unwrap()
            },
            entities: unsafe {
                ResourceLoader::godot_singleton().load("res://Entities.tscn", "PackedScene", false).unwrap()
                .try_cast::<PackedScene>().unwrap()
                .assume_safe().instance(0).unwrap()
            },
            map: GTileMap::new(),
            entity_map: GEntityMap::new()
        }
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        godot_print!("HELLO WORLD!!!");
        godot_print!("{:?}", self.tiles);
    }

    #[export]
    fn parse_command(&mut self, owner: &Spatial, command_bin: Vec<u8>) {
        if let Ok(command) = ClientCommand::from_bin(&command_bin[..]) {
            match command {
                ClientCommand::FirstSync(data) => self.first_sync(owner, data)
            }
        } else {
            godot_error!("Failed to parse command from server! Dump: {:?}", command_bin);
        }
    }

    #[export]
    fn register_player(&self, _owner: &Spatial) -> Vec<u8> {
        ServerCommand::RegisterPlayer.to_bin()
    }

    fn first_sync(&mut self, owner: &Spatial, data: FirstSyncData) {
        self.player_entity_id = Some(data.player_entity_id);
        godot_print!("My player's entity id is: {}", data.player_entity_id);
        for (x, yzslice) in data.map.iter().enumerate() {
            for (y, zslice) in yzslice.iter().enumerate() {
                for (z, tile) in zslice.iter().enumerate() {
                    match tile {
                        Tile::Air => {},
                        Tile::Dirt => {
                            let dirt: TRef<Spatial, Shared> = unsafe {
                                self.tiles.assume_safe()
                                .get_child(0).unwrap()
                                .assume_safe().duplicate(0).unwrap()
                                .assume_safe().cast().unwrap()
                            };
                            let pos = TilePos::new(x, y, z);
                            dirt.set_translation(pos.to_f32());
                            self.map.insert(pos, dirt.claim());
                            if y == 29 {
                                owner.add_child(dirt, false);
                            }
                        }
                    }
                }
            }
        }
        for (entityid, entity) in data.entities.inner() {
            if entityid == &data.player_entity_id {
                godot_print!("{:?}", entity);
                let player: TRef<Spatial, Shared> = unsafe {
                    self.entities.assume_safe()
                    .get_child(0).unwrap()
                    .assume_safe().duplicate(0).unwrap()
                    .assume_safe().cast().unwrap()
                };
                let Entity::Player(player_data) = entity;
                player.set_translation(player_data.transform.position.as_vector3d());
                self.entity_map.insert(*entityid, player.claim());
                owner.add_child(player, false);
            }
        }
    }
}

// Registers classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Server>();
}

godot_init!(init);