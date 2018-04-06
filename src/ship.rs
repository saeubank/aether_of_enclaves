use tile::*;
use misc::*;
use piston_window::{ButtonState, Key};

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
    pub directions: Vec<Direction>,
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
        let control = Tile::new(TileType::Special, TileMaterial::Wheel);
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
            directions: vec![],
            speed: 4.0,
            acc: 1.0,
            health: 1,
            width: w as f64,
            height: h as f64,
        }
    }

    // @param change The change in health.
    fn change_health(&mut self, change: i32) {
        self.health += change;
    }

    // TODO
    // fn change_tile() {}

    pub fn update(&mut self) {
        self.update_position();
    }
}

impl Moveable for Ship {
    // Moving controls implemented for the ship
    fn handle_input(&mut self, state: ButtonState, key: Option<Key>) {
        match key {
            Some(Key::W) => {
                let dir = Direction::Up;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::A) => {
                let dir = Direction::Left;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::S) => {
                let dir = Direction::Down;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::D) => {
                let dir = Direction::Right;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            _ => {}
        }
    }

    // Updates ship position.
    fn update_position(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Updates ship velocity.
    fn update_self_velocity(&mut self) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        // Control which direction ship accelerates in.
        for dir in &self.directions {
            match *dir {
                Direction::Up => dy -= self.acc,
                Direction::Down => dy += self.acc,
                Direction::Left => dx -= self.acc,
                Direction::Right => dx += self.acc,
            }
        }

        self.self_vel_x += dx;
        self.self_vel_y += dy;

        // Speed limiting.
        if self.self_vel_x > self.speed {
            self.self_vel_x = self.speed;
        }
        if self.self_vel_x < -self.speed {
            self.self_vel_x = -self.speed;
        }
        if self.self_vel_y > self.speed {
            self.self_vel_y = self.speed;
        }
        if self.self_vel_y < -self.speed {
            self.self_vel_y = -self.speed;
        }
    }
}
