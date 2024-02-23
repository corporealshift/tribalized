use std::collections::HashMap;

type Position = (u16, u16);

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
    pub map: HashMap<Position, Tile>,
    pub map_contents: HashMap<Position, Placeable>,
    pub spawns: Vec<Position>,
}

// Finds a given y on a curve to use for a river. angle is used to make
// it so the curve doesn't always move exactly left to right
fn calculate_y_offset(x: f32, angle: f32) -> f32 {
    let s = x.sin() * (x.ln_1p().sqrt().tan());
    let s2 = (x.powf(s.cos())).log10();
    (1.0 / s) * ((x * 2.0).sin() * s2) + (angle * x)
}

fn generate_river(seed: u16, map_size: u16) -> Vec<Position> {
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
        // Keep the curve within 4 whole numbers of the start. map_size/4.0 gives
        // a section of the curve for each whole x number on the grid within that
        // 4 whole number spread. Keeps the river curving the same amount regardless
        // of map size.
        let curve_x: f32 = curve_x_start as f32 + x as f32 / (fmap / 4.0);
        let curve_value = calculate_y_offset(curve_x, angle);
        let mut y = river_bed_start as f32 + curve_value * river_bed_width as f32;
        // Shift y values if they are off map
        while y > fmap {
            y = y - fmap;
        }
        while y < 0.0 {
            y = y + fmap;
        }
        // Make the river as wide as it should be. Don't add if it's off map.
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
fn generate_forests(rivers: &Vec<Position>, seed: u16, map_size: u16) -> Vec<(u16, u16)> {
    // More forests the higher the seed
    let forest_probability = seed / 10;
    let mut forest_cores: Vec<Position> = vec![];
    // for every 1/3 river point, AND we haven't added a forest in <probability>,
    let mut iters_since_added = 0;
    for (x, y) in rivers.iter() {
        if (x * y) / seed > 4 {
            if iters_since_added > forest_probability {
                let x_offset = if x > &10 { x / 8 } else { x.clone() } + forest_probability;
                let pos_above = if (x + y) % 2 >= 1 { true } else { false };
                let forest_x = if pos_above {
                    if x + x_offset > map_size - 1 {
                        map_size - 2
                    } else {
                        x + x_offset
                    }
                } else {
                    if x.clone() < x_offset {
                        2
                    } else {
                        x - x_offset
                    }
                };
                forest_cores.push((forest_x, y.clone()));
                iters_since_added = 0;
            } else {
                iters_since_added += 1;
            }
        }
    }
    //let forest_centers = rivers.iter().filter(|(x, y)| y >= seed).map(|river| {});
    // Using the seed, generate some random spots for forests too
    let num_others = if seed < 30 { seed } else { seed / 2 };
    let mut last_picked_front = true;
    for i in 1..num_others {
        //
        let start = i * 3;
        let idx = if last_picked_front {
            last_picked_front = false;
            start as usize
        } else {
            last_picked_front = true;
            if start as usize >= forest_cores.len() {
                forest_cores.len() - 1
            } else {
                forest_cores.len() - start as usize
            }
        };
        let mut new_x = forest_cores
            .get(idx as usize)
            .map(|(x, y)| if i % 2 == 0 { y.clone() } else { x.clone() })
            .unwrap_or(2);
        if new_x > 99 {
            new_x = i * 4;
        }
        if new_x < 2 {
            new_x = 2;
        }
        // let y_start = i * 9;
        let mut new_y = i * 5;
        if new_y < 2 {
            new_y = 2;
        }
        if new_y > 99 {
            new_y = 98;
        }
        forest_cores.push((new_x, new_y));
    }
    // using forest_cores, generate thick forests
    let mut forests: Vec<Position> = vec![];
    let forest_thickness = if seed % 7 < 3 { 3 } else { seed % 7 };
    for (c_x, c_y) in forest_cores {
        forests.push((c_x, c_y));
        for j in 1..=forest_thickness {
            for i in 1..=j {
                if c_x > i && c_x - i > 2 {
                    if c_x > j && i != j {
                        forests.push((c_x - j, c_y));
                        forests.push((c_x - j, c_y + i));
                    }
                    forests.push((c_x - i, c_y));
                    if i != forest_thickness {
                        forests.push((c_x - i, c_y + i));
                    }
                }
                if c_y > i && c_y - i > 2 {
                    if c_y > j && i != j {
                        forests.push((c_x, c_y - j));
                        forests.push((c_x + i, c_y - j));
                    }
                    forests.push((c_x, c_y - i));
                    if i != forest_thickness {
                        forests.push((c_x + i, c_y - i));
                    }
                }
                if c_y > i && c_x > i && c_y - i > 2 && c_x - i > 2 {
                    if i != forest_thickness {
                        forests.push((c_x - i, c_y - i));
                    }
                    if c_y > j && c_x > j && i != j {
                        forests.push((c_x - j, c_y - i));
                        forests.push((c_x - i, c_y - j));
                    }
                }
                if i != j {
                    forests.push((c_x + i, c_y + j));
                    if c_x > i && c_x - i > 2 {
                        forests.push((c_x - i, c_y + j));
                    }
                    //forests.push((c_x + j, c_y + j));
                    forests.push((c_x + j, c_y + i));
                    if c_y > i && c_y - i > 2 {
                        forests.push((c_x + j, c_y - i));
                    }
                }
                forests.push((c_x + i, c_y));

                forests.push((c_x, c_y + i));
                if i != forest_thickness {
                    forests.push((c_x + i, c_y + i));
                }
            }
        }
    }

    forests
}

fn find_spawns(map_size: u16, map: &HashMap<Position, Tile>) -> Vec<Position> {
    let offset = map_size / 8;

    let start_positions = vec![
        (offset, offset),
        (map_size - offset, offset),
        (offset, map_size - offset),
        (map_size - offset, map_size - offset),
    ];
    start_positions
        .iter()
        .map(|pos| closest_valid_position(map, pos))
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .collect()
}

fn closest_valid_position(map: &HashMap<Position, Tile>, start: &Position) -> Option<Position> {
    let mut pos: Position = start.clone();
    let mut iters = 0;
    while (iters < 10
        && (map.get(&pos).is_none()
            || map
                .get(&pos)
                .is_some_and(|t| t.terrain == Terrain::Mountain || t.terrain == Terrain::Water)))
    {
        iters += 1;
        if pos.0 > 50 {
            pos = (pos.0 - 1, pos.1 - 1);
        } else {
            pos = (pos.0 + 1, pos.1 + 1);
        }
    }
    if iters == 10 {
        None
    } else {
        Some(pos)
    }
}

impl World {
    pub fn new(seed: u16) -> World {
        println!("World Seed: {}", seed);
        let mut tiles = HashMap::new();
        let map_size = 100;
        // Generate a predefined world for now
        let mut rivers = generate_river(seed, map_size);
        let mut river2 = generate_river(seed / 2, map_size);
        rivers.append(&mut river2);

        let forests = generate_forests(&rivers, seed, map_size);
        let max = map_size;
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
                if x == 1 || x == max - 1 || y == 1 || y == max - 1 {
                    new_tile = mountain_tile();
                }
                tiles.insert((x, y), new_tile);
            }
        }
        let spawns = find_spawns(map_size, &tiles);
        Self {
            map: tiles,
            map_contents: HashMap::new(),
            spawns,
        }
    }
}
