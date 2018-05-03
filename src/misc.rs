//! Homes miscellaneous features for the game.

use piston_window::{ButtonState, Key};

pub trait Moveable {
    fn handle_input(&mut self, state: &ButtonState, key: &Option<Key>);
    fn update_position(&mut self);
    fn update_self_velocity(&mut self);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    /* 
        Generates the corresponding string of a direction.
        
        @param d The direction.
        @return String The respective string.
    */
    pub fn direction_to_string(&self) -> String {
        match *self {
            Direction::N => "N".to_string(),
            Direction::S => "S".to_string(),
            Direction::W => "W".to_string(),
            Direction::E => "E".to_string(),
        }
    }
}
