/**
    Map generates and manages the tileset for the map using Perlin and Worley generations.
*/

use noise::*;
use rand::{thread_rng, Rng};
use tile::{Tile, TileMaterial, TileType};

const STEP_SIZE: f64 = 0.1;

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let air = Tile::new(TileType::Special, TileMaterial::Air);
        let floor_grass = Tile::new(TileType::Floor, TileMaterial::Grass);
        let water = Tile::new(TileType::Special, TileMaterial::Water);
        let floor_dirt = Tile::new(TileType::Floor, TileMaterial::Dirt);
        let floor_stone = Tile::new(TileType::Floor, TileMaterial::Stone);

        let mut map_tiles = vec![vec![air.clone(); height]; width];
        let worley_arr = generate_worley(width, height, STEP_SIZE);
        let perlin_arr = generate_perlin(width, height, STEP_SIZE);

        for i in 0..map_tiles.len() {
            for j in 0..map_tiles[i].len() {
                let num = (worley_arr[i][j] + perlin_arr[i][j]) / 2.0;
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

// need to fix so edge is weighted 0 and middle is weighted 1
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
//             circle_arr[i][j] = (sizef - (x + y).sqrt()) / sizef;
//         }
//     }
//     circle_arr
// }

// fn generate_island_size() -> usize {
//     let mut normal = distributions::normal::Normal::new(ISLAND_MEAN, ISLAND_STANDERD_DEV);
//     let mut island_size = normal.sample(&mut thread_rng());
//     while island_size <= ISLAND_LOWERBOUND && island_size >= ISLAND_UPPERBOUND {
//         island_size = normal.sample(&mut thread_rng());
//     }
//     let island_size = island_size as usize;
//     island_size
// }
