//! The tile object specifies properties of different tiles in the game.
//! Floor tiles can be walked on.
//! Wall tiles cannot be walked on.

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    WoodFloor,
    StoneWall,
    GrassFloor,
    DirtFloor,
    Tree,
    Air,
    Water,
    Wheel,
    Portal,
}

/* 
    Implementation of Tile object.

    @field tile_type The tile's type.
    @field passable Whether the tile can be walked on.
    @field texture Whether the tile will have extra texture (for graphics).
*/

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub passable: bool,
    pub texture: bool,
}

impl Tile {
    /*
        Tile constructor.

        @param tile_type The type of the tile.
        @return Tile Returns itself.
    */
    pub fn new(tile_type: TileType) -> Self {
        let can_pass = match tile_type {
            TileType::Water | TileType::StoneWall | TileType::Air => false,
            _ => true,
        };
        Tile {
            tile_type: tile_type,
            passable: can_pass,
            texture: false,
        }
    }
}
