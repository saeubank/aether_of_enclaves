/**
    Map generates and manages the tileset for the map using Perlin and Worley generations.
*/

use noise::*;
// use rand::{thread_rng, Rng, distributions};
// use rand::distributions::Sample;
use rand::*;
use tile::{Tile, TileType};
use constants::*;
use piston_window::*;
use std::cmp;
use std::collections::HashMap;

const STEP_SIZE: f64 = 0.1;
const BASE_WEIGHT: f64 = 0.2;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    grass_dirt_map: HashMap<(bool, bool, bool, bool),(Option<String>, f64, f64, f64)>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let air = Tile::new(TileType::Air);
        let grass_floor = Tile::new(TileType::GrassFloor);
        let water = Tile::new(TileType::Water);
        let dirt_floor = Tile::new(TileType::DirtFloor);
        let stone_wall = Tile::new(TileType::StoneWall);
        let tree= Tile::new(TileType::Tree);

        let mut map_tiles = vec![vec![air.clone(); height]; width];
        let worley_arr = generate_worley(width, height, STEP_SIZE / 2.0);
        let perlin_arr = generate_perlin(width, height, STEP_SIZE);
        let perlin_arr = add_base_weight(&perlin_arr, BASE_WEIGHT);
        let moist = generate_perlin(width, height, STEP_SIZE);

        for i in 0..map_tiles.len() {
            for j in 0..map_tiles[i].len() {
                let num = worley_arr[i][j] * perlin_arr[i][j];
                if num <= 0.1 {
                    map_tiles[i][j] = water.clone();
                } else if num <= 0.2 {
                    map_tiles[i][j] = dirt_floor.clone();
                } else if num <= 0.6 {
                    if moist[i][j] >= 0.7 {
                        map_tiles[i][j] = tree.clone();
                    }else{
                    map_tiles[i][j] = grass_floor.clone();
                    }
                } else {
                    map_tiles[i][j] = stone_wall.clone();
                }

                if random::<f64>() < 0.10 {
                    map_tiles[i][j].texture = true;
                }
            }
        }
        Map { tiles: map_tiles, grass_dirt_map: populate_texture_maps() }
    }
    pub fn draw(
        &self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        w_width: f64,
        w_height: f64,
        player_x: f64,
        player_y: f64,
        trans_x: f64,
        trans_y: f64,
    ) {
        // Draw map.
        let draw_start_i = ((player_x - w_width / 2.0) - IMAGE_SIZE_SCALED) / IMAGE_SIZE_SCALED;
        let draw_start_j = ((player_y - w_height / 2.0) - IMAGE_SIZE_SCALED) / IMAGE_SIZE_SCALED;
        let draw_start_i = cmp::max(0, draw_start_i as i32) as usize;
        let draw_start_j = cmp::max(0, draw_start_j as i32) as usize;
        for i in draw_start_i..self.tiles.len() {
            if i as f64 * IMAGE_SIZE_SCALED > player_x + w_width / 2.0 {
                break;
            }
            for j in draw_start_j..self.tiles[i].len() {
                if j as f64 * IMAGE_SIZE_SCALED > player_y + w_height / 2.0 {
                    break;
                }
                if let (Some(img), rot, shift_x, shift_y) = self.what_to_draw(i, j) {
                    if img == IMG_TREE {
                        let grass = IMG_GRASS_FLOOR;
                        image(
                        textures.get(grass).expect(&format!("Not found: {:?}", grass)),
                        context
                            .transform
                            .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                            .trans(trans_x, trans_y)
                            .scale(IMAGE_SCALE, IMAGE_SCALE),
                        graphics,
                    );
                    }

                    image(
                        textures.get(&img).expect(&format!("Not found: {:?}", img)),
                        context
                            .transform
                            .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                            .trans(trans_x, trans_y)
                            .trans(shift_x, shift_y)
                            .scale(IMAGE_SCALE, IMAGE_SCALE)
                            .rot_deg(rot),
                        graphics,
                    );
                    
                }
            }
        }
    }

    fn what_to_draw(&self, x: usize, y: usize) -> (Option<String>, f64, f64, f64) {
        let img;
        let mut rot = 0.0;
        let mut shift_x = 0.0;
        let mut shift_y = 0.0;
        match self.tiles[x][y].tile_type {
            TileType::Water => {
                match self.tiles[x][y].texture {
                    false => img = Some(IMG_WATER.to_string()),
                    true => img = Some(IMG_WATER_TEXTURE.to_string()),
                }
            }
            TileType::StoneWall => {
                match self.tiles[x][y].texture {
                    false => img = Some(IMG_STONE_WALL.to_string()),
                    true => img = Some(IMG_STONE_WALL_TEXTURE.to_string()),
                }
            }
            TileType::GrassFloor => {
                let mut up = false;
                let mut left = false;
                let mut right = false;
                let mut down = false;

                if x > 0 {
                    if self.tiles[x - 1][y].tile_type == TileType::DirtFloor {
                        left = true;
                    }
                }
                if x < self.tiles.len() - 1 {
                    if self.tiles[x + 1][y].tile_type == TileType::DirtFloor {
                        right = true;
                    }
                }
                if y > 0 {
                    if self.tiles[x][y - 1].tile_type == TileType::DirtFloor {
                        up = true;
                    }
                }
                if y < self.tiles[x].len() - 1 {
                    if self.tiles[x][y + 1].tile_type == TileType::DirtFloor {
                        down = true;
                    }
                }


                let key = (right, down, left, up);
                if let Some(&(ref i, r, sx, sy)) = self.grass_dirt_map.get(&key) {
                    if let &Some(ref _i) = i {
                        let temp = self.grass_dirt_map.get(&key).expect(&format!("Couldn't get {:?}", key)).clone();
                       return temp;
                    }
                    else {
                        match self.tiles[x][y].texture {
                            false => img = Some(IMG_GRASS_FLOOR.to_string()),
                            true => img = Some(IMG_GRASS_FLOOR_TEXTURE.to_string()),
                        }
                    }
                }
                else {
                    img = None;
                }            
            }

            TileType::DirtFloor => {
                match self.tiles[x][y].texture {
                    false => img = Some(IMG_DIRT_FLOOR.to_string()),
                    true => img = Some(IMG_DIRT_FLOOR_TEXTURE.to_string()),
                }
            }
            TileType::Tree =>{
                img = Some(IMG_TREE.to_string());
            }
            _ => img = None,
        }
        (img, rot, shift_x, shift_y)
    }
}

fn generate_perlin(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Perlin::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn add_base_weight(arr: &Vec<Vec<f64>>, base: f64) -> Vec<Vec<f64>> {
    let mut weighted = vec![vec![0.0; arr[0].len()]; arr.len()];
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            let temp = arr[i][j] + base;
            let temp = temp.min(1.0);
            weighted[i][j] = temp;
        }
    }
    weighted
}

fn generate_worley(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Worley::new().set_seed(rng.gen::<u32>()).enable_range(true);
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = (noise.get([xpos, ypos]) + 1.0) / 2.0;
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn populate_texture_maps() -> HashMap<(bool, bool, bool, bool),(Option<String>, f64, f64, f64)> {
    let mut dirt_grass_map: HashMap<(bool, bool, bool, bool),(Option<String>, f64, f64, f64)> = HashMap::new();

    // Right, Down, Left, Up
    dirt_grass_map.insert((true,false,false,false),(Some(IMG_GRASS_DIRT_FLOOR_1_SIDE.to_string()), 0.0, 0.0, 0.0));
    dirt_grass_map.insert((false,true,false,false),(Some(IMG_GRASS_DIRT_FLOOR_1_SIDE.to_string()), 90.0, IMAGE_SIZE_SCALED, 0.0));
    dirt_grass_map.insert((false,false,true,false),(Some(IMG_GRASS_DIRT_FLOOR_1_SIDE.to_string()), 180.0, IMAGE_SIZE_SCALED, IMAGE_SIZE_SCALED));
    dirt_grass_map.insert((false,false,false,true),(Some(IMG_GRASS_DIRT_FLOOR_1_SIDE.to_string()), 270.0, 0.0, IMAGE_SIZE_SCALED));

    // Corners
    dirt_grass_map.insert((true,false,false,true),(Some(IMG_GRASS_DIRT_FLOOR_CORNER.to_string()), 0.0, 0.0, 0.0));
    dirt_grass_map.insert((true,true,false,false),(Some(IMG_GRASS_DIRT_FLOOR_CORNER.to_string()), 90.0, IMAGE_SIZE_SCALED, 0.0));
    dirt_grass_map.insert((false,true,true,false),(Some(IMG_GRASS_DIRT_FLOOR_CORNER.to_string()), 180.0, IMAGE_SIZE_SCALED, IMAGE_SIZE_SCALED));
    dirt_grass_map.insert((false,false,true,true),(Some(IMG_GRASS_DIRT_FLOOR_CORNER.to_string()), 270.0, 0.0, IMAGE_SIZE_SCALED));

    // 2 Sides
    dirt_grass_map.insert((true,false,true,false),(Some(IMG_GRASS_DIRT_FLOOR_2_SIDE.to_string()), 0.0, 0.0, 0.0));
    dirt_grass_map.insert((false,true,false,true),(Some(IMG_GRASS_DIRT_FLOOR_2_SIDE.to_string()), 90.0, IMAGE_SIZE_SCALED, 0.0));

    // 3 Sides
    dirt_grass_map.insert((true,false,true,true),(Some(IMG_GRASS_DIRT_FLOOR_3_SIDE.to_string()), 0.0, 0.0, 0.0));
    dirt_grass_map.insert((true,true,false,true),(Some(IMG_GRASS_DIRT_FLOOR_3_SIDE.to_string()), 90.0, IMAGE_SIZE_SCALED, 0.0));
    dirt_grass_map.insert((true,true,true,false),(Some(IMG_GRASS_DIRT_FLOOR_3_SIDE.to_string()), 180.0, IMAGE_SIZE_SCALED, IMAGE_SIZE_SCALED));
    dirt_grass_map.insert((false,true,true,true),(Some(IMG_GRASS_DIRT_FLOOR_3_SIDE.to_string()), 270.0, 0.0, IMAGE_SIZE_SCALED));

    // 4 Sides
    dirt_grass_map.insert((true,true,true,true),(Some(IMG_GRASS_DIRT_FLOOR_4_SIDE.to_string()), 0.0, 0.0, 0.0));

    dirt_grass_map.insert((false,false,false,false),(None, 0.0, 0.0, 0.0));

    dirt_grass_map

}

// struct Island {
//     pub tiles: Vec<Vec<Tile>>,
//     pub x: f64,
//     pub y: f64,
// }

// impl Island {
//     fn new() -> Self {
//         Island {
//             tiles: vec![vec![]],
//             x: 0.0,
//             y: 0.0
//         }
//     }
// }

// fn generate_weighted_circle(size: usize) -> Vec<Vec<f64>> {
//     let mut circle_arr = vec![vec![0.0; size]; size];
//     let sizef = size as f64;
//     let middle: f64 = (sizef - 1) / 2.0;
//     for i in 0..size {
//         for j in 0..size {
//             let x: f64 = middle - i as f64;
//             let x = x * x;
//             let y: f64 = middle - j as f64;
//             let y = y * y;
//             let val = (middle - (x + y).sqrt()) / middle;
//             let val = val * 1.5;
//             let val = val.max(0.0);
//             let val = val.min(1.0);
//             circle_arr[i][j] = val;
//         }
//     }
//     circle_arr
// }
// const ISLAND_MEAN: f64 = 75.0;
// const ISLAND_STANDERD_DEV: f64 = 10.0;
// const ISLAND_LOWERBOUND: f64 = 10.0;
// const ISLAND_UPPERBOUND: f64 = 200.0;
// fn generate_island_size() -> usize {
//     let mut normal = distributions::normal::Normal::new(ISLAND_MEAN, ISLAND_STANDERD_DEV);
//     let mut island_size = normal.sample(&mut thread_rng());
//     while island_size <= ISLAND_LOWERBOUND && island_size >= ISLAND_UPPERBOUND {
//         island_size = normal.sample(&mut thread_rng());
//     }
//     let island_size = island_size as usize;
//     island_size
// }
