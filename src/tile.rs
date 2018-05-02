#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    WoodFloor,
    StoneFloor,
    StoneWall,
    GrassFloor,
    DirtFloor,
    Tree,
    Air,
    Water,
    Wheel,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub passable: bool,
    pub texture: bool,
}

/**
    Implementation of Tile object.
    Provides permutations of different tile types.
*/
impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let can_pass = match tile_type {
            TileType::Tree | TileType::Water | TileType::StoneWall | TileType::Air => false,
            _ => true,
        };
        Tile {
            tile_type: tile_type,
            passable: can_pass,
            texture: false,
        }
    }
}
