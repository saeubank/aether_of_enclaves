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
    Water,
}

pub struct Tile {
    tile_type: TileType,
    material: TileMaterial,
    destructable: bool,
    health: i32,
}

/*
Something like this: https://forums.rpgmakerweb.com/index.php?threads/stairs-movement.9216/
Move diagonally when on stairs to simulate moving up?
Should game have jumping?
to make this effect: https://youtu.be/fbWjx2HpWPU?t=54m49s
https://lparchive.org/Dragon-Quest-Heroes-Rocket-Slime/Update%2008/12-Shockwave.gif


Main game mechanic: do something (slap tail) to make character move faster.
Moving faster is a very important mechanic in every game, movement is what makes a game fun.
GAME NEEDS TO HAVE FUN MOVEMENT (not clunkey, must be responsive and have visual & audio impact/feedback)


be able to customize ship with different rooms and also customize engine room to have defenses

jumping makes charactor invincible?

*/

impl Tile {
    fn new(tile: TileType, mat: TileMaterial, destruct: bool, hp: i32) -> Self {
        Tile {
            tile_type: tile,
            material: mat,
            destructable: destruct,
            health: hp,
        }
    }
}

// Should there be z levels or height?
// If so add stairs
