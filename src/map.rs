/**
    Map generates and manages the tileset for the map using Perlin and Worley generations.
*/

use noise::*;
// use rand::{thread_rng, Rng, distributions};
// use rand::distributions::Sample;
use rand::{thread_rng, Rng};
use tile::{Tile, TileType};
use constants::*;
use piston_window::*;
use std::cmp;
use std::collections::HashMap;

const STEP_SIZE: f64 = 0.1;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let air = Tile::new(TileType::Air);
        let grass_floor = Tile::new(TileType::GrassFloor);
        let water = Tile::new(TileType::Water);
        let dirt_floor = Tile::new(TileType::DirtFloor);
        let stone_wall = Tile::new(TileType::StoneWall);

        let mut map_tiles = vec![vec![air.clone(); height]; width];
        let worley_arr = generate_worley(width, height, STEP_SIZE);
        let perlin_arr = generate_perlin(width, height, STEP_SIZE);

        for i in 0..map_tiles.len() {
            for j in 0..map_tiles[i].len() {
                let num = worley_arr[i][j] * perlin_arr[i][j];
                if num <= -0.3 {
                    map_tiles[i][j] = water.clone();
                } else if num <= 0.0 {
                    map_tiles[i][j] = dirt_floor.clone();
                } else if num <= 0.3 {
                    map_tiles[i][j] = grass_floor.clone();
                } else {
                    map_tiles[i][j] = stone_wall.clone();
                }
            }
        }
        Map { tiles: map_tiles }
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
                match self.tiles[i][j].tile_type {
                    TileType::Water => {
                        let img = IMG_WATER;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                                .trans(trans_x, trans_y)
                                .scale(IMAGE_SCALE, IMAGE_SCALE),
                            graphics,
                        );
                    }
                    TileType::StoneWall => {
                        let img = IMG_STONE_WALL;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                                .trans(trans_x, trans_y)
                                .scale(IMAGE_SCALE, IMAGE_SCALE),
                            graphics,
                        );
                    }
                    TileType::GrassFloor => {
                        let img = IMG_GRASS_FLOOR;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                                .trans(trans_x, trans_y)
                                .scale(IMAGE_SCALE, IMAGE_SCALE),
                            graphics,
                        );
                    }
                    TileType::DirtFloor => {
                        let img = IMG_DIRT_FLOOR;
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(i as f64 * IMAGE_SIZE_SCALED, j as f64 * IMAGE_SIZE_SCALED)
                                .trans(trans_x, trans_y)
                                .scale(IMAGE_SCALE, IMAGE_SCALE),
                            graphics,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    // fn what_to_draw(&self, x: usize, y: usize) {

    // }
}

fn generate_perlin(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Perlin::new().set_seed(rng.gen::<u32>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = noise.get([xpos, ypos]);
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
}

fn generate_worley(width: usize, height: usize, step: f64) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let noise = Worley::new().set_seed(rng.gen::<u32>()).enable_range(true);
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut arr = vec![vec![0.0; height]; width];
    for i in 0..width {
        for j in 0..height {
            arr[i][j] = noise.get([xpos, ypos]);
            xpos += step;
        }
        xpos = 0.0;
        ypos += step;
    }
    arr
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
//     let middle: f64 = sizef / 2.0;
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
