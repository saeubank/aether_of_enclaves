enum TileType {
    Floor,
    Wall,
    Special,
}

enum TileMaterial {
    Wood,
    Stone,
    Grass,
    Air,
    Water
}

struct Tile {
    tile_type: TileType,
    material: TileMaterial,
    destructable: bool,
    health: i32
}

// Should there be z levels or height?
// If so add stairs
