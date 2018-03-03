use noise::*;
use rand::{distributions, thread_rng};
use rand::distributions::Sample;

#[derive(Copy, Clone)]
pub enum TileType {
    Air,
    Grass,
    Water
}

pub struct Island {
    pub tiles: Vec<Vec<TileType>>,
}

impl Island {
    pub fn new() -> Self {
        let island_size = generate_island_size();
        let perlin_arr = generate_perlin(island_size, 0.2);
        let mut temp_tiles = vec![vec![TileType::Air; island_size]; island_size];
        for i in 0..temp_tiles.len() {
            for j in 0..temp_tiles[i].len() {
                let num = perlin_arr[i][j];
                if num < 0.0 {
                    temp_tiles[i][j] = TileType::Grass;
                }
                else {
                    temp_tiles[i][j] = TileType::Water;
                }
            }
        }
        Island {
            tiles: temp_tiles
        }
    }
}

fn generate_perlin(size: usize, step: f32) -> Vec<Vec<f32>> {
    let perlin = Perlin::new();
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

fn generate_island_size() -> usize {
    let mut normal = distributions::normal::Normal::new(10.0, 2.0);
    let mut island_size = normal.sample(&mut thread_rng());
    while island_size <= 2.0 && island_size >= 50.0 {
        island_size = normal.sample(&mut thread_rng());
    }
    let island_size = island_size as usize;
    island_size
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
}

impl Map {
    pub fn new() -> Self {
        let worley: Worley<f32> = Worley::new();
        let map_tiles = vec![vec![TileType::Air; 1]; 1];
        Map {
            tiles: map_tiles
        }
    }
}
