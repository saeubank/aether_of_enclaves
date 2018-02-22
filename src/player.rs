use input_handler::{Direction};
use self::Direction::*;


/** 
	Implementation of the Player object.

	@field x Player's horizontal position on screen.
	@field y Player's vertical position on screen.
*/

pub struct Player {
	pub x: f64,
	pub y: f64
}

impl Player {

	// Updates the player's position.
	// @param dir The direction player will move.
	// @param dist The distance in pixels to move. 
    pub fn update_position(&mut self, dir: Direction, dist: f64) {
        match dir {
            Up => self.y -= dist,
            Down => self.y += dist,
            Left => self.x -= dist,
            Right => self.x += dist,
            _ => {}
        }
    }
}