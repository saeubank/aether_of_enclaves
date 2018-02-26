extern crate noise;

// use self::noise::{NoiseModule, Perlin};


enum TileType {
    // Air,
    Grass,
    // Water
}

pub struct Island {
    tiles: Vec<Vec<TileType>>,
}

impl Island {
    pub fn new() {
        // Docs here https://docs.rs/noise/0.4.1/noise/struct.Perlin.html
        // let perlin = Perlin::new();
        // println!("{:?}", perlin.get([42.4, 37.7, 2.8]));
        for thing in tiles {
            for tile in thing {
                tile = TileType::Grass;
            }
        }
    }
}

// pub struct Map {
//     map = Worley
// }
