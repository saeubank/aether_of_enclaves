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

// Generates the corresponding string of a direction.
// @param d The direction.
pub fn direction_to_string(d: &Direction) -> String {
    let s;
    match *d {
        Direction::N => s = "N",
        Direction::S => s = "S",
        Direction::W => s = "W",
        Direction::E => s = "E",
        Direction::NE => s = "NE",
        Direction::SE => s = "SE",
        Direction::SW => s = "SW",
        Direction::NW => s = "NW",
    }
    s.to_string()
}
