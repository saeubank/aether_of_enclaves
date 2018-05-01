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
    NE,
    NW,
    SE,
    SW,
}

impl Direction {
// Generates the corresponding string of a direction.
// @param d The direction.
pub fn direction_to_string(&self) -> String {
    match *self {
        Direction::N => "N".to_string(),
        Direction::S => "S".to_string(),
        Direction::W => "W".to_string(),
        Direction::E => "E".to_string(),
        Direction::NE => "NE".to_string(),
        Direction::SE => "SE".to_string(),
        Direction::SW => "SW".to_string(),
        Direction::NW => "NW".to_string(),
    }
}
}