/*
    Not used Currently
*/

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
        // https://doc.rust-lang.org/1.0.0/rand/distributions/normal/struct.Normal.html
        let mut normal = distributions::normal::Normal::new(10.0, 2.0);
        let mut island_size = normal.sample(&mut thread_rng());
        while island_size <= 2.0 {
            island_size = normal.sample(&mut thread_rng());
        }
        let island_size = island_size as usize;
        let mut temp_tiles = vec![vec![TileType::Air; island_size]; island_size];

        // Docs here https://docs.rs/noise/0.4.1/noise/struct.Perlin.html
        let perlin = Perlin::new();
        // maybe need to use Perlin::set_seed(self, seed: usize)
        let mut xpos = 0.0;
        let mut ypos = 0.0;
        for i in 0..temp_tiles.len() {
            for j in 0..temp_tiles[i].len() {
                let num = perlin.get([xpos, ypos]);
                if num < 0.0 {
                    temp_tiles[i][j] = TileType::Grass;
                }
                else {
                    temp_tiles[i][j] = TileType::Water;
                }
                println!("| {:?} |", num);
                xpos += 0.2;
                ypos += 0.2;
            }
        }
        Island {
            tiles: temp_tiles
        }
        // Island {
        //     tiles: vec![vec![TileType::Air; 10]; 10]
        // }
    }
}

// pub struct Map {
//     map = Worley
// }
