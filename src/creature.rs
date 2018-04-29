use misc::*;
use piston_window::*;
use item::Item;

// const MAX_INVENTORY_SIZE: usize = 3;

/**
    The Creature object is the template for any NPC in AOE. Primarily this is used for
    the Player, but default functionality is also implemented for Monsters and Crew.
*/

pub enum CreatureType {
    Player,
    // Fighter,
    // Cook,
    // Carpenter,
    // Monster,
}

pub enum CreatureState {
    Normal,
    ControllingShip,
    // Still,
    // Moving,
    // Jumping,
    // Attacking,
    // Throwing,
    // Interacting,

}

/**
	Implementation of the Creature object.

	@field x Creature's horizontal position on screen.
	@field y Creature's vertical position on screen.
    @field self_vel_x Creature's horizontal velocity.
    @field self_vel_y Creature's vertical velocity.
    @field other_vel_x Horizontal velocity of other object(s) affecting Creature.
    @field other_vel_y Horizontal velocity of other object(s) affecting Creature.
    @field speed Creature's maximum speed when moving.
    @field health Creature's health.
    @field inventory Creature's item inventory.
    @field dir Creature's direction.
    @field sprite_index Used for animating the Creature's sprite.

*/
pub struct Creature {
    pub creature_type: CreatureType,
    pub creature_state: CreatureState,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    pub directions: Vec<Direction>,
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    pub speed: f64,
    pub health: i32,
    pub inventory: Vec<Item>,
    pub dir: Direction,
    pub sprite_index: i32,
}

impl Creature {
    // Constructor for default Creature.
    pub fn new(c_type: CreatureType) -> Creature {
        Creature {
            creature_type: c_type,
            creature_state: CreatureState::Normal,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            directions: vec![],
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 3.0,
            health: 1,
            inventory: vec![],
            dir: Direction::S,
            sprite_index: 2,
        }
    }

    // Updates the position of creature based on other objects acting on it.
    pub fn update_position_other(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
    }

    // Updates creature's position based on its own velocity.
    pub fn update_position_self(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Updates the direction that the creature is facing.
    pub fn update_direction(&mut self) {
        let mut dir_y = None;
        let mut dir_x = None;
        if !(self.self_vel_y == 0.0 && self.self_vel_x == 0.0) {
            if self.self_vel_y > 0.0 {
                dir_y = Some(Direction::S);
            } else if self.self_vel_y < 0.0 {
                dir_y = Some(Direction::N);
            }

            if self.self_vel_x > 0.0 {
                dir_x = Some(Direction::E);
            } else if self.self_vel_x < 0.0 {
                dir_x = Some(Direction::W);
            }

            match dir_y {
                Some(Direction::N) => match dir_x {
                    Some(Direction::W) => self.dir = Direction::NW,
                    Some(Direction::E) => self.dir = Direction::NE,
                    None => self.dir = Direction::N,
                    _ => {}
                },
                Some(Direction::S) => match dir_x {
                    Some(Direction::W) => self.dir = Direction::SW,
                    Some(Direction::E) => self.dir = Direction::SE,
                    None => self.dir = Direction::S,
                    _ => {}
                },
                None => match dir_x {
                    Some(Direction::W) => self.dir = Direction::W,
                    Some(Direction::E) => self.dir = Direction::E,
                    _ => {}
                },
                _ => {}
            }
        } else {
            // Override sprite when creature isn't moving.
            self.sprite_index = 2;
        }
    }

    // Determines where the creature is about to move.
    pub fn x_to_be_location(&mut self) -> f64 {
        self.x + self.self_vel_x
    }
    pub fn y_to_be_location(&mut self) -> f64 {
        self.y + self.self_vel_y
    }

    // pub fn throw_item(&mut self) {
    //     if self.inventory.len() > 0 {
    //         self.inventory.pop();
    //     }
    // }

    // pub fn pickup_item(&mut self, item: Item) -> bool {
    //     if self.inventory.len() < MAX_INVENTORY_SIZE {
    //         self.inventory.push(item);
    //         return true
    //     }
    //     false
    // }

    pub fn action(&mut self) {
        match self.creature_state {
                    CreatureState::Normal => self.state_normal(),
                    CreatureState::ControllingShip => self.state_controlling_ship(),
                }
    }

    fn state_controlling_ship(&mut self) {
        self.creature_state = CreatureState::Normal;
    }
    
    fn state_normal(&mut self) {
        self.directions = vec![];
        self.self_vel_x = 0.0;
        self.self_vel_y = 0.0;
        self.creature_state = CreatureState::ControllingShip;
    }

    // TODO Write collision function.
    // pub fn update(&mut self, &) {

    // }
}

// Moving of creature.
impl Moveable for Creature {
    fn handle_input(&mut self, state: ButtonState, key: Option<Key>) {
        match key {
            Some(Key::W) => {
                let dir = Direction::N;
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
                let dir = Direction::W;
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
                let dir = Direction::S;
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
                let dir = Direction::E;
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

    // Updates position based on velocity.
    // Overwritten for creature.
    fn update_position(&mut self) {
        self.x += self.other_vel_x;
        self.y += self.other_vel_y;
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Changes the Creature's personal velocity (unrelated to other velocities acting on
    // the creature).
    // @param dx The difference in x velocity.
    // @param dy The difference in y velocity.
    fn update_self_velocity(&mut self) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        for dir in &self.directions {
            match *dir {
                Direction::N => dy -= self.speed,
                Direction::S => dy += self.speed,
                Direction::W => dx -= self.speed,
                Direction::E => dx += self.speed,
                _ => {}
            }
        }
        self.self_vel_x = dx;
        self.self_vel_y = dy;
    }
}
