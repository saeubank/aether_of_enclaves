#[derive(Clone)]
pub enum TileType {
    Floor,
    Wall,
    Special,
}

#[derive(Clone)]
pub enum TileMaterial {
    Wood,
    Stone,
    Grass,
    Air,
    Water,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub material: TileMaterial,
}

impl Tile {
    pub fn new(tile: TileType, mat: TileMaterial) -> Self {
        Tile {
            tile_type: tile,
            material: mat,
        }
    }
}

/*
Something like this: https://forums.rpgmakerweb.com/index.php?threads/stairs-movement.9216/
Move diagonally when on stairs to simulate moving up?
Should game have jumping?
to make this effect: https://youtu.be/fbWjx2HpWPU?t=54m49s
https://lparchive.org/Dragon-Quest-Heroes-Rocket-Slime/Update%2008/12-Shockwave.gif
Should there be z levels or height?
If so add stairs


Main game mechanic: do something (slap tail) to make character move faster.
Moving faster is a very important mechanic in every game, movement is what makes a game fun.
GAME NEEDS TO HAVE FUN MOVEMENT (not clunkey, must be responsive and have visual & audio impact/feedback)


be able to customize ship with different rooms and also customize engine room to have defenses

jumping makes charactor invincible?

*/