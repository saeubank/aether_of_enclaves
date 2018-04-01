// use std::collections::HashMap;
use misc::*;
// use game::Game;
use piston_window::{Button, ButtonState, Key};

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
*/
pub struct Creature {
    pub creature_type: CreatureType,
    pub x: f64,
    pub y: f64,
    pub self_vel_x: f64,
    pub self_vel_y: f64,
    // pub directions: HashMap<>,
    pub directions: [Option<Direction>; 4],
    pub other_vel_x: f64,
    pub other_vel_y: f64,
    pub speed: f64,
    pub health: i32,
    // pub inventory: [Item; 3],
}

impl Creature {
    // Constructor for default Creature.
    pub fn new(c_type: CreatureType) -> Creature {
        Creature {
            creature_type: c_type,
            x: 0.0,
            y: 0.0,
            self_vel_x: 0.0,
            self_vel_y: 0.0,
            // directions: HashMap::new(),
            directions: [None; 4],
            other_vel_x: 0.0,
            other_vel_y: 0.0,
            speed: 3.0,
            health: 1,
        }
    }

    pub fn update(&mut self) {
        self.update_position();
    }

    // TODO Write collision function.
}

impl moveable for Creature {
    fn handle_input(&mut self, state: ButtonState, key: Option<Key>) {
        if state == ButtonState::Press {
            let mut dx: f64 = 0.0;
            let mut dy: f64 = 0.0;

            if key == Some(Key::W) {
                dy -= self.speed;
            }
            if key == Some(Key::A) {
                dx -= self.speed;
            }
            if key == Some(Key::S) {
                dy += self.speed;
            }
            if key == Some(Key::D) {
                dx += self.speed;
            }
            self.change_self_velocity(dx, dy);
        }
        // Set Player's velocity to zero when key is released.
        else if state == ButtonState::Release {
            if key == Some(Key::W) || key == Some(Key::S) {
                self.reset_self_velocity_y();
            }
            if key == Some(Key::A) || key == Some(Key::D) {
                self.reset_self_velocity_x();
            }
        }
    }
    // fn collision(&mut self, game: &Game) -> bool {
    //     true
    // }

    // Updates position based on velocity.
    fn update_position(&mut self) {
        self.x += self.self_vel_x;
        self.y += self.self_vel_y;
    }

    // Changes the Creature's personal velocity (unrelated to other velocities acting on
    // the creature).
    // @param dx The difference in x velocity.
    // @param dy The difference in y velocity.
    fn change_self_velocity(&mut self, dx: f64, dy: f64) {
        self.self_vel_x += dx;
        self.self_vel_y += dy;

        // Account for exceeding Creature's max speed.
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

    // Sets horizontal velocity to zero.
    fn reset_self_velocity_x(&mut self) {
        self.self_vel_x = 0.0;
    }

    // Sets vertical velocity to zero.
    fn reset_self_velocity_y(&mut self) {
        self.self_vel_y = 0.0;
    }
}
