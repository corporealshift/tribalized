use std::collections::HashMap;

pub enum Terrain {
    Grass,
    Forest,
    Water,
    Mountain,
}

pub enum Placeable {
    Individual,
    Structure,
    Item,
}

pub struct Tile {
    terrain: Terrain,
    contents: Option<Placeable>,
}

fn mountain_tile() -> Tile {
    Tile {
        terrain: Terrain::Mountain,
        contents: None,
    }
}

fn grassland_tile(contents: Option<Placeable>) -> Tile {
    Tile {
        terrain: Terrain::Grass,
        contents,
    }
}

fn water_tile(contents: Option<Placeable>) -> Tile {
    Tile {
        terrain: Terrain::Water,
        contents,
    }
}

fn forest_tile(contents: Option<Placeable>) -> Tile {
    Tile {
        terrain: Terrain::Forest,
        contents,
    }
}

pub struct World {
    map: HashMap<(u32, u32), Tile>,
}

impl World {
    pub fn new() -> World {
        // A
        let mut tiles = HashMap::new();
        // Generate a predefined world for now
        let max = 50;
        for y in 1..max {
            for x in 1..max {
                let mut new_tile = mountain_tile();

                if y > 1 && y < max && x > 1 && x < max {
                    new_tile = grassland_tile(None);
                }
                if x > 1 && x < max {
                    if y == 20 || y == 21 || y == 30 || y == 31 {
                        new_tile = forest_tile(None);
                    } else if y == 24 || y == 25 || y == 26 {
                        new_tile = water_tile(None);
                    }
                }
                tiles.insert((x, y), new_tile);
            }
        }
        Self {
            map: HashMap::new(),
        }
    }
}
