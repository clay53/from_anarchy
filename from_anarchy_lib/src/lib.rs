#[cfg(feature = "godot")]
pub mod godot;

#[cfg(feature = "commands")]
pub mod commands;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Tile {
    Air,
    Dirt
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {

}

#[derive(Debug)]
pub struct Game {
    map: Vec<Vec<Vec<Tile>>>,
    entities: Vec<Entity>
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
            entities: Vec::new()
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
