use tile::*;

/**
    Implementation of the Ship object.

    @field tiles The tilset for the ship's "floor".
    @field x Ship's horizontal position on screen.
    @field y Ship's vertical position on screen.
    @field self_vel_x Ship's horizontal velocity.
    @field self_vel_y Ship's vertical velocity.
    @field speed Ship's maximum speed when moving.
    @field acc Ship's acceleration value.
    @field health Ship's health.
    @field width Ship's thiccness.
    @field height Ship's height.
*/
pub struct Ship {
    pub tiles: Vec<Vec<Tile>>,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub speed: f64,
    pub acc: f64,
    pub health: i32,
    pub width: f64,
    pub height: f64,
}

impl Ship {
    // Ship constructor.
    // @param ship_tiles The 2D tileset of the ship.
    pub fn new(ship_tiles: Vec<Vec<i32>>) -> Self {
        // Populate ship's tileset with proper TileMaterial.
        let air = Tile::new(TileType::Special, TileMaterial::Air);
        let floor_wood = Tile::new(TileType::Floor, TileMaterial::Wood);
        let control = Tile::new(TileType::Special, TileMaterial::Grass);
        let w = ship_tiles[0].len();
        let h = ship_tiles.len();
        let mut temp_tiles = vec![vec![air.clone(); h]; w];

        for i in 0..ship_tiles.len() {
            for j in 0..ship_tiles[i].len() {
                match ship_tiles[i][j] {
                    0 => temp_tiles[j][i] = air.clone(),
                    1 => temp_tiles[j][i] = floor_wood.clone(),
                    2 => temp_tiles[j][i] = control.clone(),
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
            health: 1,
            width: w as f64,
            height: h as f64,
        }
    }

    // @param change The change in health.
    fn change_health(&mut self, change: i32) { self.health += change; }
    
    // Update ship's position with ship's velocity.
    pub fn update_position(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Changes ship's velocity.
    // @param dx The difference in x velocity.
    // @param dy The difference in y velocity.
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

    // TODO
    fn update_tile() {}
}
