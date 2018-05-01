/**
    Ship manages the tile set, positions, movement, and generation of the player's ship.
*/

use tile::*;
use misc::*;
use piston_window::*;
use IMAGE_SIZE;
use std::collections::HashMap;

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
    directions: Vec<Direction>,
    speed: f64,
    health: i32,
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
            speed: 6.0,
            health: 1,
            width: w as f64,
            height: h as f64,
        }
    }

    pub fn x_to_be_location(&mut self) -> f64 {
        self.x + self.self_vel_x
    }
    pub fn y_to_be_location(&mut self) -> f64 {
        self.y + self.self_vel_y
    }

    pub fn reset_dir(&mut self) {
        self.directions = vec![];
    }

    // @param change The change in health.
    // fn change_health(&mut self, change: i32) {
    //     self.health += change;
    // }

    // TODO
    // fn change_tile() {}

    // pub fn update(&mut self) {
    //     self.update_position();
    // }

    pub fn draw(
        &mut self,
        textures: &HashMap<String, G2dTexture>,
        context: &Context,
        graphics: &mut G2d,
        trans_x: f64,
        trans_y: f64,
    ) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                match self.tiles[i][j].material {
                    TileMaterial::Wood => {
                        let img = "floor_boards";
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(
                                    self.x + i as f64 * IMAGE_SIZE,
                                    self.y + j as f64 * IMAGE_SIZE,
                                )
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    TileMaterial::Wheel => {
                        let img = "floor_boards";
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(
                                    self.x + i as f64 * IMAGE_SIZE,
                                    self.y + j as f64 * IMAGE_SIZE,
                                )
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                        let img = "wheel";
                        image(
                            textures.get(img).expect(&format!("Not found: {:?}", img)),
                            context
                                .transform
                                .trans(
                                    self.x + i as f64 * IMAGE_SIZE,
                                    self.y + j as f64 * IMAGE_SIZE,
                                )
                                .trans(trans_x, trans_y),
                            graphics,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Moveable for Ship {
    // Moving controls implemented for the ship
    fn handle_input(&mut self, state: &ButtonState, key: &Option<Key>) {
        match *key {
            Some(Key::W) => {
                let dir = Direction::N;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::A) => {
                let dir = Direction::W;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::S) => {
                let dir = Direction::S;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
                        self.directions.push(dir);
                    }
                }
            }
            Some(Key::D) => {
                let dir = Direction::E;
                if let Some(index) = self.directions.iter().position(|&x| x == dir) {
                    if *state == ButtonState::Release {
                        self.directions.remove(index);
                    }
                } else {
                    if *state == ButtonState::Press {
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
                Direction::N => dy -= self.speed,
                Direction::S => dy += self.speed,
                Direction::W => dx -= self.speed,
                Direction::E => dx += self.speed,
                _ => {}
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
