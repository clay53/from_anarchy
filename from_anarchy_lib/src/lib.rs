#[cfg(feature = "godot")]
pub mod godot;

#[cfg(feature = "commands")]
pub mod commands;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Tile {
    Air,
    Dirt,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityMap {
    entity_map: HashMap<u64, Entity>,
    id_counter: u64,
}

impl EntityMap {
    pub fn new() -> EntityMap {
        EntityMap {
            entity_map: HashMap::new(),
            id_counter: 0
        }
    }

    pub fn inner(&mut self) -> &mut HashMap<u64, Entity> {
        &mut self.entity_map
    }

    pub fn push(&mut self, entity: Entity) {
        let id = self.next_id();
        self.entity_map.insert(id, entity);
    }

    // Ids will eventually run out in a save. This will have to be adapted later to allow for inputting into freed positions
    fn next_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;
        return id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    map: Vec<Vec<Vec<Tile>>>,
    entities: EntityMap
}

impl Game {
    pub fn new() -> Game {
        let mut map = Vec::with_capacity(256);
        for _x in 0..256 {
            let mut yzslice = Vec::with_capacity(256);
            for _y in 0..256 {
                let mut zslice = Vec::with_capacity(256);
                for z in 0..256 {
                    let tile = if z <= 128 { Tile::Air } else { Tile::Dirt };
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let mut game = Game::new();
    }
}
