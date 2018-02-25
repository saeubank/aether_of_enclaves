use input_handler::Direction;
use self::Direction::*;

/**
	Implementation of the Player object.

	@field x Player's horizontal position on screen.
	@field y Player's vertical position on screen.
*/
pub struct Player {
    pub x: f64,
    pub y: f64,
}

impl Player {
    pub fn new() -> Player {
        Player { x: 0.0, y: 0.0 }
    }

    // Updates the player's position.
    // @param dir The direction player will move.
    // @param dist The distance in pixels to move.
    pub fn update_position(&mut self, dir: Option<Direction>, dist: f64) {
        match dir {
            Some(Up) => self.y -= dist,
            Some(Down) => self.y += dist,
            Some(Left) => self.x -= dist,
            Some(Right) => self.x += dist,
            _ => {}
        }
    }
}
