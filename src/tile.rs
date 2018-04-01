#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Floor,
    // Wall,
    Special,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileMaterial {
    Wood,
    // Stone,
    // Grass,
    // Sand,
    // long grass,
    // Dirt,
    // figure out how to add trees
    Air,
    // Water,
    Wheel,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub material: TileMaterial,
}

/**
    Implementation of Tile object.
    Provides permutations of different tile types.
*/
impl Tile {
    pub fn new(tile: TileType, mat: TileMaterial) -> Self {
        Tile {
            tile_type: tile,
            material: mat,
        }
    }
}
