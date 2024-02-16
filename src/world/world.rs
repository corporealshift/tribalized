use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Terrain {
    Grass,
    Forest,
    Water,
    Mountain,
}

#[derive(PartialEq, Debug)]
pub enum Placeable {
    Individual,
    Structure,
    Item,
}

#[derive(PartialEq, Debug)]
pub struct Tile {
    pub terrain: Terrain,
    pub contents: Option<Placeable>,
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
#[derive(Debug)]
pub struct World {
    pub map: HashMap<(u16, u16), Tile>,
}

// a is a way to introduce an angle to the river (so it isn't always straight up->down/left->right)
// a = 0.5
// q = tan(sqrt(ln(x)))*sin(x)
// y_prime = (1/q(sin(2x) * log(x^(cos(q))))) + (a * x)
// how can I spread this out? if the map_size is 100...and I want to use curve over 10...
// so x = x/10 + seed, and we iterate at 0.1 instead of 1
// x between 13 and 26 is a good range...so stay in that area
// start = seed/8 + 13
// use a range of 4 for the curve. so for pt 1, y_offset = y_prime(start)
// for pt 2, y_offset = y_offset(start + x/25))
// river bed is seed > 10 ? seed / 10 : seed

// Finds a given y on a curve to use for a river. angle is used to make
// it so the curve doesn't always move exactly left to right
fn calculate_y_offset(x: f32, angle: f32) -> f32 {
    let s = x.sin() * (x.ln_1p().sqrt().tan());
    let s2 = (x.powf(s.cos())).log10();
    (1.0 / s) * ((x * 2.0).sin() * s2) + (angle * x)
}

fn generate_river(seed: u16, map_size: u16) -> Vec<(u16, u16)> {
    let fseed: f32 = seed.into();
    let fmap: f32 = map_size.into();
    let river_bed_width = if seed > 10 { seed / 10 } else { seed };
    // Angle should be between -2 and +2
    let angle = (fseed / fmap) * 4.0 - 2.0;
    // River width is between 3 and 7
    let river_width = ((fseed / fmap) * 4.0 + 3.0) as u16;
    let river_bed_start = if seed + river_bed_width > map_size {
        seed / 10
    } else {
        seed
    };

    let mut river_coords: Vec<(u16, u16)> = vec![];
    let curve_x_start: u16 = (seed / 8) + 13;
    println!(
        "River bed: {}, start: {}, width: {}, angle: {}",
        river_bed_width, river_bed_start, river_width, angle
    );
    for x in 1..map_size {
        let curve_x: f32 = curve_x_start as f32 + x as f32 / 25.0;
        let curve_value = calculate_y_offset(curve_x, angle);
        let mut y = river_bed_start as f32 + curve_value * river_bed_width as f32;
        println!("Y: {}", y);
        while y > fmap {
            y = y - fmap;
        }
        while y < 0.0 {
            y = y + fmap;
        }
        for ys in 1..=river_width {
            let y1 = y + ys as f32;
            if y1 <= fmap - 1.0 && y1 > 2.0 {
                river_coords.push((x, y1 as u16));
            }
        }
    }
    // println!("River coordinates: {:?}", river_coords);
    river_coords
}

// Forests grow near the river(s), with some amount of variation based on the
// seed. River coordinates are provided to decide if the provided x,y should
// be a forest or not.
fn generate_forests(rivers: &Vec<(u16, u16)>, seed: u16) -> Vec<(u16, u16)> {
    vec![]
}

impl World {
    pub fn new(seed: u16) -> World {
        println!("World Seed: {}", seed);
        let mut tiles = HashMap::new();
        // Generate a predefined world for now
        let mut rivers = generate_river(seed, 100);
        let mut river2 = generate_river(seed / 2, 100);
        rivers.append(&mut river2);

        let forests = generate_forests(&rivers, seed);
        let max = 100;
        for y in 1..max {
            for x in 1..max {
                let mut new_tile = mountain_tile();
                let tile_coord = (x, y);

                if y > 1 && y < max - 1 && x > 1 && x < max - 1 {
                    new_tile = grassland_tile(None);
                }
                if x > 1 && x < max - 1 {
                    if forests.contains(&tile_coord) {
                        new_tile = forest_tile(None);
                    }
                    if rivers.contains(&tile_coord) {
                        new_tile = water_tile(None);
                    }
                }
                tiles.insert((x, y), new_tile);
            }
        }
        Self { map: tiles }
    }
}
