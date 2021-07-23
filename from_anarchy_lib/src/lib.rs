#[cfg(feature = "godot")]
pub mod godot;

#[cfg(feature = "commands")]
pub mod commands;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use euclid::{Vector3D, UnknownUnit};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Tile {
    Air,
    Dirt,
}

pub type TileMap = Vec<Vec<Vec<Tile>>>;

pub type TilePos = Vector3D<usize, UnknownUnit>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Entity {
    Player(Player)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PosRot {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PosRot {
    pub fn new(x: f32, y: f32, z: f32) -> PosRot {
        PosRot {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn as_vector3d(&self) -> Vector3D<f32, UnknownUnit> {
        Vector3D::new(self.x, self.y, self.z)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EntityTransform {
    pub position: PosRot,
    pub rotation: PosRot
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub transform: EntityTransform
}

pub type EntityId = u64;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EntityMap {
    entity_map: HashMap<u64, Entity>,
    id_counter: EntityId,
}

impl EntityMap {
    pub fn new() -> EntityMap {
        EntityMap {
            entity_map: HashMap::new(),
            id_counter: 0
        }
    }

    pub fn inner(&self) -> &HashMap<EntityId, Entity> {
        &self.entity_map
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<EntityId, Entity> {
        &mut self.entity_map
    }

    pub fn push(&mut self, entity: Entity) -> EntityId {
        let id = self.next_id();
        self.entity_map.insert(id, entity);
        id
    }

    // Ids will eventually run out in a save. This will have to be adapted later to allow for inputting into freed positions
    fn next_id(&mut self) -> EntityId {
        let id = self.id_counter;
        self.id_counter += 1;
        return id
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    map: TileMap,
    entities: EntityMap
}

impl Game {
    pub fn new() -> Game {
        let mut map = TileMap::with_capacity(32);
        for _x in 0..32 {
            let mut yzslice = Vec::with_capacity(32);
            for y in 0..32 {
                let mut zslice = Vec::with_capacity(32);
                for _z in 0..32 {
                    let tile = if y <= 29 { Tile::Dirt } else { Tile::Air };
                    zslice.push(tile);
                }
                yzslice.push(zslice);
            }
            map.push(yzslice);
        }

        Game {
            map: map,
            entities: EntityMap::new()
        }
    }

    pub fn get_map(&self) -> &TileMap {
        &self.map
    }

    pub fn get_entities(&self) -> &EntityMap {
        &self.entities
    }

    pub fn new_player(&mut self, name: String) -> EntityId {
        self.entities.push(Entity::Player(Player {
            name: name,
            transform: EntityTransform {
                position: PosRot {
                    x: 15.0,
                    y: 30.0,
                    z: 15.0
                },
                rotation: PosRot {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let mut game = Game::new();
    }
}
