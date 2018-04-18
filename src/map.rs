/*
    Not in use
*/

use noise::*;
use rand::{distributions, thread_rng};
use rand::distributions::Sample;
use rand::Rng;
use tile::{Tile, TileMaterial, TileType};

const STEP_SIZE: f64 = 0.2;

// const ISLAND_MEAN: f64 = 10.0;
// const ISLAND_STANDERD_DEV: f64 = 2.0;
// const ISLAND_LOWERBOUND: f64 = 2.0;
// const ISLAND_UPPERBOUND: f64 = 50.0;
// const ISLAND_STEP: f64 = 0.2;

// pub struct Island {
//     pub tiles: Vec<Vec<Tile>>,
// }

// impl Island {
//     pub fn new() -> Self {
//         let island_size = generate_island_size();
//         let perlin_arr = generate_perlin(island_size, ISLAND_STEP);
//         let mut temp_tiles = vec![vec![TileType::Air; island_size]; island_size];
//         for i in 0..temp_tiles.len() {
//             for j in 0..temp_tiles[i].len() {
//                 let num = perlin_arr[i][j];
//                 if num < 0.0 {
//                     temp_tiles[i][j] = TileType::Grass;
//                 } else {
//                     temp_tiles[i][j] = TileType::Water;
//                 }
//             }
//         }
//         Island { tiles: temp_tiles }
//     }
// }

fn generate_perlin(size: usize, step: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new();
    let mut rng = thread_rng();
    let perlin = perlin.set_seed(rng.gen::<usize>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut perlin_arr = vec![vec![0.0; size]; size];
    for i in 0..perlin_arr.len() {
        for j in 0..perlin_arr[i].len() {
            perlin_arr[i][j] = perlin.get([xpos, ypos]);
            xpos += step;
        }
        ypos += step;
    }
    perlin_arr
}

// need to fix so edge is weighted 0 and middle is weighted 1
fn generate_weighted_circle(size: usize) -> Vec<Vec<f64>> {
    let mut circle_arr = vec![vec![0.0; size]; size];
    let sizef = size as f64;
    let middle: f64 = sizef / 2.0;
    for i in 0..size {
        for j in 0..size {
            let x: f64 = middle - i as f64;
            let x = x * x;
            let y: f64 = middle - j as f64;
            let y = y * y;
            circle_arr[i][j] = (sizef - (x + y).sqrt()) / sizef;
        }
    }
    circle_arr
}

// fn generate_island_size() -> usize {
//     let mut normal = distributions::normal::Normal::new(ISLAND_MEAN, ISLAND_STANDERD_DEV);
//     let mut island_size = normal.sample(&mut thread_rng());
//     while island_size <= ISLAND_LOWERBOUND && island_size >= ISLAND_UPPERBOUND {
//         island_size = normal.sample(&mut thread_rng());
//     }
//     let island_size = island_size as usize;
//     island_size
// }

const MAP_WIDTH: usize = 20;
const MAP_HEIGHT: usize = 20;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Self {
        let air = Tile::new(TileType::Special, TileMaterial::Air);
        let floor_grass = Tile::new(TileType::Floor, TileMaterial::Grass);
        let water = Tile::new(TileType::Special, TileMaterial::Water);
        let floor_dirt = Tile::new(TileType::Floor, TileMaterial::Dirt);
        let floor_stone = Tile::new(TileType::Floor, TileMaterial::Stone);

        let mut map_tiles = vec![vec![air.clone(); MAP_HEIGHT]; MAP_WIDTH];
        let worley_arr = generate_worley(MAP_WIDTH, STEP_SIZE);
        let perlin_arr = generate_perlin(MAP_WIDTH, STEP_SIZE);

        for i in 0..map_tiles.len() {
            for j in 0..map_tiles[i].len() {
                let num = worley_arr[i][j] * perlin_arr[i][j];
                // let num = perlin_arr[i][j];
                if num <= -0.5 {
                    map_tiles[i][j] = water.clone();
                } else if num <= 0.0 {
                    map_tiles[i][j] = floor_dirt.clone();
                } else if num <= 0.5 {
                    map_tiles[i][j] = floor_grass.clone();
                } else {
                    map_tiles[i][j] = floor_stone.clone();
                }
            }
        }
        Map { tiles: map_tiles }
    }
}

fn generate_worley(size: usize, step: f64) -> Vec<Vec<f64>> {
    let worley: Worley<f64> = Worley::new();
    let mut rng = thread_rng();
    let worley = worley.set_seed(rng.gen::<usize>());
    let mut xpos = 0.0;
    let mut ypos = 0.0;
    let mut worley_arr = vec![vec![0.0; size]; size];
    for i in 0..worley_arr.len() {
        for j in 0..worley_arr[i].len() {
            worley_arr[i][j] = worley.get([xpos, ypos]);
            xpos += step;
        }
        ypos += step;
    }
    worley_arr
}
