use tile::*;

const SHIP_TILES: [[i32; 11]; 10] =
[
[0,0,0,0,1,1,1,0,0,0,0],
[0,0,0,1,1,1,1,1,0,0,0],
[0,0,0,1,1,1,1,1,0,0,0],
[0,0,1,1,1,1,1,1,1,0,0],
[0,1,1,1,1,1,1,1,1,1,0],
[1,1,1,1,1,1,1,1,1,1,1],
[1,1,1,1,1,1,1,1,1,1,1],
[1,0,1,1,1,1,1,1,1,0,1],
[0,0,1,1,1,1,1,1,1,0,0],
[0,0,0,0,1,1,1,0,0,0,0],
];

struct Ship {
    pub tiles: Vec<Vec<Tile>>,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub speed: f64,
    pub acc: f64,
    pub health: i32,
}

impl Ship {
    fn new() -> Self {
        let air = Tile::new(TileType::Special, TileMaterial::Air);
        let floor_wood = Tile::new(TileType::Floor, TileMaterial::Wood);
        let mut temp_tiles = vec![vec![air.clone(); SHIP_TILES[0].len()]; SHIP_TILES.len()];

        for i in 0..SHIP_TILES.len() {
            for j in 0..SHIP_TILES[i].len() {
                match j {
                    0 => temp_tiles[i][j] = air.clone(),
                    1 => temp_tiles[i][j] = floor_wood.clone(),
                    _ => {}
                }
            }
        }
        Ship {
            tiles: temp_tiles,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            speed: 20.0,
            acc: 2.0,
            health: 1
        }
    }

    fn change_health(&mut self, change: i32) { self.health += change; }
    
    pub fn update_position(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // velocity should be based on both what the creature is on and the actual movement of the creature
    pub fn change_self_velocity(&mut self, dx: f64, dy: f64) {
        self.self_vel_x += dx;
        self.self_vel_y += dy;

        if self.self_vel_x > self.speed {
            self.self_vel_x = self.speed;
        }
        if self.self_vel_y > self.speed {
            self.self_vel_y = self.speed;
        }
        if self.self_vel_x < -self.speed {
            self.self_vel_x = -self.speed;
        }
        if self.self_vel_y < -self.speed {
            self.self_vel_y = -self.speed;
        }
    }

    fn update_tile() {}
}
